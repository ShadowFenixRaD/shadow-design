#[derive(Clone, PartialEq, Debug)]
pub enum TokenKind {
    Keyword,
    Type,
    StringLit,
    Number,
    Comment,
    Prop,
    Plain,
}

pub fn tokenize_rsx(code: &str) -> Vec<(TokenKind, String)> {
    let mut tokens = Vec::new();
    let mut chars = code.chars().peekable();

    while let Some(&ch) = chars.peek() {
        // Comment
        if ch == '/' {
            chars.next();
            if let Some(&next_ch) = chars.peek() {
                if next_ch == '/' {
                    chars.next();
                    let mut comment = String::from("//");
                    while let Some(&c) = chars.peek() {
                        if c == '\n' {
                            break;
                        }
                        comment.push(chars.next().unwrap());
                    }
                    tokens.push((TokenKind::Comment, comment));
                    continue;
                }
            }
            tokens.push((TokenKind::Plain, "/".to_string()));
            continue;
        }

        // String
        if ch == '"' {
            chars.next();
            let mut s = String::from("\"");
            while let Some(&c) = chars.peek() {
                chars.next();
                s.push(c);
                if c == '\\' {
                    if let Some(&_escaped) = chars.peek() {
                        s.push(chars.next().unwrap());
                    }
                } else if c == '"' {
                    break;
                }
            }
            tokens.push((TokenKind::StringLit, s));
            continue;
        }

        // Identifiers or numbers
        if ch.is_alphabetic() || ch == '_' {
            let mut ident = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_alphanumeric() || c == '_' {
                    ident.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            // Check if followed by colon (prop name e.g. "variant:")
            if let Some(&':') = chars.peek() {
                // Peek one further to see if not '::'
                let mut iter = chars.clone();
                iter.next(); // skip ':'
                if iter.peek() != Some(&':') {
                    chars.next(); // consume ':'
                    ident.push(':');
                    tokens.push((TokenKind::Prop, ident));
                    continue;
                }
            }

            match ident.as_str() {
                "fn" | "let" | "pub" | "use" | "mut" | "struct" | "enum" | "impl" | "match"
                | "if" | "else" | "move" | "true" | "false" | "return" | "async" | "for"
                | "in" | "rsx" => {
                    tokens.push((TokenKind::Keyword, ident));
                }
                "Some" | "None" | "Ok" | "Err" | "String" | "Option" | "Signal" | "Element"
                | "Button" | "Card" | "Badge" | "Input" | "TextInput" | "Select" | "MultiSelect"
                | "Combobox" | "Modal" | "Drawer" | "Dropdown" | "Tooltip" | "Tabs" | "Tab"
                | "TabList" | "TabPanel" | "Switch" | "Checkbox" | "Alert" | "Toast" | "Navbar"
                | "Heading" | "Text" | "Icon" | "IconKind" | "CardVariant" | "ButtonVariant"
                | "BadgeVariant" => {
                    tokens.push((TokenKind::Type, ident));
                }
                _ if ident.starts_with(|c: char| c.is_uppercase()) => {
                    tokens.push((TokenKind::Type, ident));
                }
                _ => {
                    tokens.push((TokenKind::Plain, ident));
                }
            }
            continue;
        }

        if ch.is_ascii_digit() {
            let mut num = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() || c == '.' {
                    num.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            tokens.push((TokenKind::Number, num));
            continue;
        }

        // Whitespace and symbols
        tokens.push((TokenKind::Plain, ch.to_string()));
        chars.next();
    }

    tokens
}
