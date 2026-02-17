#[must_use]
pub fn classify(node: &tree_sitter::Node, text: &str) -> Option<&'static str> {
    if node.is_error() || node.is_missing() {
        return None;
    }

    let kind = node.kind();

    if node.is_named() {
        return match kind {
            "comment" => Some("syntax-comment"),
            "string" | "string_content" | "string_start" | "string_end" | "escape_sequence" => {
                Some("syntax-string")
            }
            "integer" | "float" | "true" | "false" | "none" => Some("syntax-constant"),
            "identifier" => Some(identifier(node, text)),
            "type" => Some("syntax-entity"),
            _ => None,
        };
    }

    match kind {
        "def" | "if" | "elif" | "else" | "for" | "in" | "return" | "pass" | "break"
        | "continue" | "and" | "or" | "not" | "lambda" | "class" | "import" | "from" | "as"
        | "with" | "while" | "try" | "except" | "finally" | "raise" | "yield" | "del"
        | "assert" | "global" | "nonlocal" | "is" => Some("syntax-keyword"),
        "=" | "+=" | "-=" | "*=" | "/=" | "//=" | "%=" | "**=" | "&=" | "|=" | "^=" | ">>="
        | "<<=" | "+" | "-" | "*" | "/" | "//" | "%" | "**" | "<" | ">" | "<=" | ">=" | "=="
        | "!=" | "|" | "&" | "^" | "~" | "<<" | ">>" | "@" => Some("syntax-operator"),
        "(" | ")" | "[" | "]" | "{" | "}" | ":" | "," | "." | ";" => Some("syntax-punctuation"),
        _ => None,
    }
}

fn identifier(node: &tree_sitter::Node, text: &str) -> &'static str {
    if text == "load" {
        return "syntax-macro";
    }
    if matches!(text, "True" | "False" | "None") {
        return "syntax-constant";
    }

    if let Some(parent) = node.parent() {
        match parent.kind() {
            "function_definition" | "class_definition" => {
                if parent.child_by_field_name("name").as_ref() == Some(node) {
                    return "syntax-entity";
                }
            }
            "call" => {
                if parent.child_by_field_name("function").as_ref() == Some(node) {
                    return "syntax-function";
                }
            }
            "decorator" => return "syntax-macro",
            "keyword_argument"
            | "parameter"
            | "typed_parameter"
            | "default_parameter"
            | "typed_default_parameter" => {
                if parent.child_by_field_name("name").as_ref() == Some(node) {
                    return "syntax-variable";
                }
            }
            _ => {}
        }
    }

    "syntax-variable"
}
