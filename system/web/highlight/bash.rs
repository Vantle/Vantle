#[must_use]
pub fn classify(node: &tree_sitter::Node, text: &str) -> Option<&'static str> {
    if node.is_error() || node.is_missing() {
        return None;
    }

    let kind = node.kind();

    if node.is_named() {
        return match kind {
            "comment" => Some("syntax-comment"),
            "string" | "raw_string" | "ansi_c_string" | "heredoc_body" | "heredoc_content"
            | "heredoc_start" | "string_content" | "regex" => Some("syntax-string"),
            "number" | "file_descriptor" => Some("syntax-constant"),
            "variable_name" | "special_variable_name" | "simple_expansion" | "expansion" => {
                Some("syntax-variable")
            }
            "command_name" => Some(command(text)),
            "word" => word(node, text),
            _ => None,
        };
    }

    match kind {
        "if" | "then" | "else" | "elif" | "fi" | "for" | "while" | "do" | "done" | "case"
        | "esac" | "in" | "function" | "select" | "until" | "declare" | "export" | "local"
        | "readonly" | "unset" | "return" | "exit" => Some("syntax-keyword"),
        "|" | "||" | "&&" | ">" | ">>" | "<" | "&" | "!" | "=" | "+=" | "==" | "!=" | "-eq"
        | "-ne" | "-lt" | "-gt" | "-le" | "-ge" | "-z" | "-n" | "-f" | "-d" | "-e" => {
            Some("syntax-operator")
        }
        "(" | ")" | "{" | "}" | "[" | "]" | "[[" | "]]" | ";;" | ";" => Some("syntax-punctuation"),
        "$" => Some("syntax-variable"),
        "\"" | "'" | "`" => Some("syntax-string"),
        _ => None,
    }
}

fn command(text: &str) -> &'static str {
    if keyword(text) {
        return "syntax-keyword";
    }
    "syntax-function"
}

fn word(node: &tree_sitter::Node, text: &str) -> Option<&'static str> {
    if node
        .parent()
        .is_some_and(|parent| parent.kind() == "command_name")
    {
        return Some(command(text));
    }
    if text.starts_with("--") || text.starts_with('-') {
        return Some("syntax-storage");
    }
    if text.contains("//") || text.starts_with(':') {
        return Some("syntax-entity");
    }
    if text.contains("://") || text.contains('/') {
        return Some("syntax-string");
    }
    None
}

fn keyword(text: &str) -> bool {
    matches!(
        text,
        "if" | "then"
            | "else"
            | "elif"
            | "fi"
            | "for"
            | "while"
            | "do"
            | "done"
            | "case"
            | "esac"
            | "in"
            | "function"
            | "return"
            | "exit"
            | "export"
            | "source"
            | "alias"
            | "sudo"
    )
}
