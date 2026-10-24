#!/usr/bin/env python3
"""
scripts/check_classes.py
Scans RSX string literals in crate/src and showcase/src for CSS classes,
compares them against selectors in compiled CSS, and reports any unknown utility classes.
"""

import os
import re
import shutil
import subprocess
import sys

# Paths below are relative to the workspace root, whatever the caller's cwd.
os.chdir(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))

def sass_command():
    # Prefer a native `sass` (e.g. dart-sass from the Nix dev shell), fall back to npx.
    # shutil.which resolves Windows shims like npx.cmd, so no shell=True is needed.
    sass = shutil.which("sass")
    if sass:
        return [sass]
    npx = shutil.which("npx")
    if npx:
        return [npx, "--yes", "sass"]
    print("Error: neither `sass` nor `npx` found in PATH (run inside `nix develop`).")
    sys.exit(1)

def compile_scss():
    cmd = sass_command() + ["--no-source-map", "crate/scss/main.scss", "target/compiled_styles.css"]
    res = subprocess.run(cmd, capture_output=True, text=True)
    if res.returncode != 0:
        print(f"Error compiling SCSS:\n{res.stderr}")
        sys.exit(1)
    # Showcase chrome (.sc-*) is a second stylesheet.
    cmd = sass_command() + ["--no-source-map", "showcase/assets/showcase.scss", "target/compiled_showcase.css"]
    res = subprocess.run(cmd, capture_output=True, text=True)
    if res.returncode != 0:
        print(f"Error compiling showcase SCSS:\n{res.stderr}")
        sys.exit(1)

def extract_selectors():
    content = ""
    for css in ("target/compiled_styles.css", "target/compiled_showcase.css"):
        with open(css, "r", encoding="utf-8") as f:
            content += f.read()
    # Find all class selectors
    raw_classes = re.findall(r'\.([a-zA-Z0-9_\-\\\/\[\]\.\:\#\(\)\,]+)', content)
    unescaped = set()
    for c in raw_classes:
        # Strip pseudo-classes like :hover, :active, :focus, ::before
        base = re.split(r'(?<!\\):', c)[0]
        clean = base.replace('\\', '').rstrip(',')
        unescaped.add(clean)
    return unescaped

def scan_source_classes():
    used = set()
    root_dirs = ["crate/src", "showcase/src"]
    # Look for class: "..." or class_modifiers or class tokens
    token_re = re.compile(r'class(?:_modifiers)?\s*:\s*"([^"]+)"')
    for root_dir in root_dirs:
        for root, _, files in os.walk(root_dir):
            for file in files:
                if file == 'nav.rs':
                    continue
                if file.endswith('.rs'):
                    path = os.path.join(root, file)
                    with open(path, "r", encoding="utf-8") as f:
                        text = f.read()
                    for match in token_re.findall(text):
                        for token in match.split():
                            # Ignore rsx interpolation like {id}, {class}, etc.
                            if not token.startswith("{") and not token.endswith("}"):
                                used.add(token)
    return used

def main():
    print("Compiling SCSS to target/compiled_styles.css...")
    compile_scss()
    css_classes = extract_selectors()
    print(f"Extracted {len(css_classes)} distinct class selectors from compiled CSS.")

    used_classes = scan_source_classes()
    print(f"Found {len(used_classes)} distinct class tokens in RSX sources.")

    missing = []
    # Dynamic or special tokens to skip
    ignore_prefixes = ('lucide', 'rsx')
    for token in sorted(used_classes):
        if token in css_classes:
            continue
        # Check if hover: or focus: or media prefixes match
        clean_token = token
        if ":" in token:
            clean_token = token.split(":")[-1]
        if token not in css_classes and clean_token not in css_classes:
            # Let's see if it's dynamic
            if not any(token.startswith(p) for p in ignore_prefixes):
                missing.append(token)

    if missing:
        print(f"Warning: {len(missing)} classes used in RSX not directly found in CSS:")
        for m in missing[:30]:
            print(f"  - {m}")
        if len(missing) > 30:
            print(f"  ... and {len(missing) - 30} more.")
    else:
        print("All analyzed RSX classes have matching CSS rules!")

if __name__ == "__main__":
    main()
