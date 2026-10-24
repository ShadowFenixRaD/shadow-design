#!/usr/bin/env python3
"""Shadow Design lint: enforces ADR-0003 / ADR-0004 rules a machine can check.

Usage:
  python scripts/lint_design.py                 # whole repo
  python scripts/lint_design.py FILE [FILE...]  # only these files (scss or rs)
  python scripts/lint_design.py --no-compile    # skip the compiled-CSS class check

A single line may opt out with a trailing comment:  // sd-lint-allow: <reason>
Exit code 1 when any violation is found.
"""
from __future__ import annotations

import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SCSS_COMPONENTS = ROOT / "crate" / "scss" / "components"
RUST_SRC = ROOT / "crate" / "src"
MAIN_SCSS = ROOT / "crate" / "scss" / "main.scss"

ALLOW = "sd-lint-allow"

# ---------------------------------------------------------------- SCSS rules
SCSS_RULES: list[tuple[str, re.Pattern[str], str]] = [
    ("color-literal", re.compile(r"#[0-9a-fA-F]{3,8}\b(?!\s*\{)"),
     "colour literal: use a var(--sd-*) token (docs/design/tokens.md)"),
    ("color-literal", re.compile(r"\b(?:rgba?|hsla?)\(\s*[\d.]"),
     "colour literal: use a token or rgba(var(--sd-*-rgb), a)"),
    ("static-var", re.compile(r"\$(?:violet|crimson|cyan|emerald|obsidian|zinc|color|radius|palette|success|danger|warning|info)\b[\w-]*"),
     "static SCSS colour/radius variable: use the runtime var(--sd-*) token"),
    ("z-index", re.compile(r"z-index\s*:\s*(?!z\()(?!-?[01]\s*[;}])"),
     "z-index must use z('<layer>') (only -1/0/1 allowed for local stacking)"),
    ("important", re.compile(r"!important"), "!important is forbidden (ADR-0004 §7)"),
    ("box-shadow", re.compile(r"(?<![\w-])box-shadow\s*:"),
     "write shadows through @include sd-shadow + --sd-shadow-* slots"),
    ("transition", re.compile(r"(?<![\w-])transition\s*:"),
     "use @include sd-transition"),
    ("animation", re.compile(r"(?<![\w-])animation(?:-name)?\s*:"),
     "use @include sd-animate / sd-animate-essential"),
    ("keyframes", re.compile(r"@keyframes"),
     "keyframes live in _animations.scss (sd-* names)"),
    ("font-size", re.compile(r"font-size\s*:\s*[\d.]+(?:px|rem|em)"),
     "use var(--sd-text-*)"),
    ("font-family", re.compile(r"font-family\s*:\s*(?!var\()"),
     "use var(--sd-font-sans) / var(--sd-font-mono)"),
    ("radius", re.compile(r"border(?:-[a-z]+)*-radius\s*:\s*(?!var\()(?!0\s*[;}])(?!50%\s*[;}])(?!inherit)"),
     "use var(--sd-radius-*)"),
    ("media", re.compile(r"@media\s*\(\s*(?:min|max)-width"),
     "use @include bp-up(<bp>)"),
]

# ---------------------------------------------------------------- Rust rules
UTILITY_COLOR = re.compile(
    r"(?<![\w-])(?:[a-z-]+:)*(?:bg|text|border|ring|shadow|from|to|via|fill|stroke|outline|divide|placeholder|accent|caret)-"
    r"(?:purple|violet|rose|red|emerald|green|cyan|sky|blue|amber|yellow|zinc|gray|slate|neutral|white|black|pink|fuchsia|indigo|teal|orange|lime)\b"
)
UTILITY_OTHER = re.compile(
    r"(?<![\w-])(?:[a-z-]+:)*(?:z-\[?\d|rounded(?:-[\w\[\]]+)?(?![\w-])|shadow-(?:sm|md|lg|xl|2xl|inner)\b|"
    r"flex(?![\w-])|inline-flex|grid-cols-|items-(?:center|start|end)|justify-(?:center|between|start|end)|"
    r"gap-\d|[pm][xytblr]?-\d|w-full|h-full|text-(?:xs|sm|base|lg|xl|2xl)\b|font-(?:mono|bold|semibold|medium)\b|"
    r"absolute(?![\w-])|relative(?![\w-])|fixed(?![\w-])|inset-0|backdrop-blur|opacity-\d|transition(?:-all)?(?![\w-])|duration-\d)"
)
HEX_IN_STRING = re.compile(r"#[0-9a-fA-F]{3,8}\b")
ITALIAN = re.compile(
    r"\b(?:Chiudi|Apri|Avanti|Indietro|Completa|Passo|Caricamento|Nessun[ao]?|Pagina|Paginazione|Precedente|Successiva|"
    r"Prec|Succ|Copia(?:to)?|Scarica|Condividi|Rispondi|Scrivi|Annulla|Invia(?:to)?|Cerca|Seleziona|Mostra|percorso|"
    r"Decrementa|Incrementa|Minimo|Massimo|Valore|Percentuale|Gennaio|Febbraio|Marzo|Aprile|Maggio|Giugno|Luglio|Agosto|"
    r"Settembre|Ottobre|Novembre|Dicembre|Luned[iì]|Gio|Ven|Dom|Principale|navigare|selezionare|chiudere|Altri|"
    r"Aggiungi|attivit[aà]|registrata|evidenza|Elementi|disponibile|Trascina|Rilascia|Rimuovi|Elimina|Salva)\b"
)
STRING_LIT = re.compile(r'"(?:[^"\\\n]|\\.)*"')
RUST_EXEMPT_HEX = {"theme.rs", "util.rs"}
RUST_EXEMPT_ITALIAN = {"locale.rs"}


def strip_scss_comment(line: str) -> str:
    # keep `//` inside url(...) etc. out of scope: components should not contain urls
    idx = line.find("//")
    code = line if idx == -1 else line[:idx]
    return re.sub(r"/\*.*?\*/", "", code)


def lint_scss(path: Path) -> list[str]:
    errors: list[str] = []
    text = path.read_text(encoding="utf-8")
    if "@layer sd-components" not in text:
        errors.append(f"{rel(path)}:1: [layer] component rules must sit inside @layer sd-components")
    in_block_comment = False
    for no, raw in enumerate(text.splitlines(), 1):
        if ALLOW in raw:
            continue
        line = raw
        if in_block_comment:
            if "*/" in line:
                line = line.split("*/", 1)[1]
                in_block_comment = False
            else:
                continue
        if "/*" in line and "*/" not in line:
            line = line.split("/*", 1)[0]
            in_block_comment = True
        code = strip_scss_comment(line)
        if not code.strip() or code.lstrip().startswith("@use") or code.lstrip().startswith("@forward"):
            continue
        for name, rx, msg in SCSS_RULES:
            if rx.search(code):
                errors.append(f"{rel(path)}:{no}: [{name}] {msg}  →  {raw.strip()[:110]}")
    return errors


def rust_strings(text: str):
    for no, raw in enumerate(text.splitlines(), 1):
        stripped = raw.lstrip()
        if stripped.startswith("//") or ALLOW in raw:
            continue
        code = raw.split(" //", 1)[0]
        for m in STRING_LIT.finditer(code):
            yield no, raw, m.group(0)[1:-1]


def lint_rust(path: Path) -> list[str]:
    errors: list[str] = []
    text = path.read_text(encoding="utf-8")
    # ignore #[cfg(test)] modules: everything after the first `#[cfg(test)]`
    cut = text.find("#[cfg(test)]")
    if cut != -1:
        text = text[:cut]
    name = path.name
    for no, _, s in rust_strings(text):
        where = f"{rel(path)}:{no}"
        if UTILITY_COLOR.search(s):
            errors.append(f"{where}: [utility-color] library code emits only sd-* classes (ADR-0004 §2)  →  \"{s[:90]}\"")
        elif UTILITY_OTHER.search(s) and _looks_like_classes(s):
            errors.append(f"{where}: [utility] library code emits only sd-* classes (ADR-0004 §2)  →  \"{s[:90]}\"")
        if name not in RUST_EXEMPT_HEX and HEX_IN_STRING.search(s) and "{" not in s.replace("{{", ""):
            errors.append(f"{where}: [color-literal] use var(--sd-*) tokens, not hex  →  \"{s[:90]}\"")
        if name not in RUST_EXEMPT_ITALIAN and ITALIAN.search(s):
            errors.append(f"{where}: [locale] UI text must come from Locale (ADR-0004 §14)  →  \"{s[:90]}\"")
    return errors


def _looks_like_classes(s: str) -> bool:
    toks = s.split()
    if not toks:
        return False
    classy = sum(1 for t in toks if re.fullmatch(r"[a-z0-9:\[\]/._%{}-]+", t))
    return classy == len(toks)


def compiled_classes() -> set[str] | None:
    sass = shutil.which("sass")
    if not sass:
        print("lint_design: `sass` not found, skipping compiled-class check (run inside `nix develop`)", file=sys.stderr)
        return None
    with tempfile.NamedTemporaryFile(suffix=".css", delete=False) as tmp:
        out = tmp.name
    res = subprocess.run([sass, "--no-source-map", f"--load-path={MAIN_SCSS.parent}", str(MAIN_SCSS), out],
                         capture_output=True, text=True)
    if res.returncode != 0:
        print(res.stderr, file=sys.stderr)
        raise SystemExit("lint_design: main.scss failed to compile")
    css = Path(out).read_text(encoding="utf-8")
    return set(re.findall(r"\.(sd-[A-Za-z0-9_-]+)", css))


def lint_missing_classes(files: list[Path], css: set[str]) -> list[str]:
    errors: list[str] = []
    for path in files:
        text = path.read_text(encoding="utf-8")
        cut = text.find("#[cfg(test)]")
        if cut != -1:
            text = text[:cut]
        for no, _, s in rust_strings(text):
            if "{" in s:
                continue
            for cls in re.findall(r"(?<![\w-])(sd-[a-z0-9_-]+)", s):
                if cls.endswith("-") or cls in css:
                    continue
                errors.append(f"{rel(path)}:{no}: [missing-class] `{cls}` is emitted but has no CSS rule")
    return errors


def rel(p: Path) -> str:
    try:
        return str(p.relative_to(ROOT))
    except ValueError:
        return str(p)


def main(argv: list[str]) -> int:
    compile_check = "--no-compile" not in argv
    args = [a for a in argv if not a.startswith("--")]
    if args:
        targets = [Path(a).resolve() for a in args]
        scss = [p for p in targets if p.suffix == ".scss" and SCSS_COMPONENTS in p.parents]
        rust = [p for p in targets if p.suffix == ".rs" and RUST_SRC in p.parents]
    else:
        scss = sorted(SCSS_COMPONENTS.glob("*.scss"))
        rust = sorted(RUST_SRC.rglob("*.rs"))
    errors: list[str] = []
    for p in scss:
        errors += lint_scss(p)
    for p in rust:
        errors += lint_rust(p)
    if compile_check and rust:
        css = compiled_classes()
        if css is not None:
            errors += lint_missing_classes(rust, css)
    for e in errors:
        print(e)
    n = len(errors)
    print(f"lint_design: {n} violation{'s' if n != 1 else ''} in {len(scss)} scss + {len(rust)} rust files", file=sys.stderr)
    return 1 if n else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
