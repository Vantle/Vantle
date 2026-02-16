pub struct Token {
    pub role: &'static str,
    pub light: &'static str,
    pub dark: &'static str,
}

pub const PALETTE: &[Token] = &[
    Token {
        role: "background",
        light: "#fafaf9",
        dark: "#0f0f0f",
    },
    Token {
        role: "text",
        light: "#1a1a1a",
        dark: "#e5e5e5",
    },
    Token {
        role: "text-secondary",
        light: "#6b7280",
        dark: "#9ca3af",
    },
    Token {
        role: "accent",
        light: "#d45d00",
        dark: "#ff8c42",
    },
    Token {
        role: "accent-hover",
        light: "#b84e00",
        dark: "#ffa366",
    },
    Token {
        role: "border",
        light: "#e5e5e5",
        dark: "#2e2e2e",
    },
    Token {
        role: "code-background",
        light: "#f5f5f4",
        dark: "#1e1e1e",
    },
    Token {
        role: "code-text",
        light: "#1a1a1a",
        dark: "#e5e5e5",
    },
    Token {
        role: "nav-background",
        light: "#fafaf9cc",
        dark: "#0f0f0fcc",
    },
    Token {
        role: "table-stripe",
        light: "#f9fafb",
        dark: "#1a1a1a",
    },
];

pub const SYNTAX: &[Token] = &[
    Token {
        role: "syntax-keyword",
        light: "#7b2d8e",
        dark: "#d4a0e0",
    },
    Token {
        role: "syntax-entity",
        light: "#b24a00",
        dark: "#f0943e",
    },
    Token {
        role: "syntax-string",
        light: "#2a7a4c",
        dark: "#7ec89e",
    },
    Token {
        role: "syntax-comment",
        light: "#8b8685",
        dark: "#847f7d",
    },
    Token {
        role: "syntax-constant",
        light: "#8b6513",
        dark: "#debb6b",
    },
    Token {
        role: "syntax-storage",
        light: "#3d5aa0",
        dark: "#8fa8d4",
    },
    Token {
        role: "syntax-punctuation",
        light: "#7d7872",
        dark: "#928d87",
    },
    Token {
        role: "syntax-variable",
        light: "#1a6b6a",
        dark: "#6ec4c0",
    },
    Token {
        role: "syntax-function",
        light: "#5644a6",
        dark: "#a794d6",
    },
    Token {
        role: "syntax-operator",
        light: "#a8294a",
        dark: "#d88a9c",
    },
    Token {
        role: "syntax-macro",
        light: "#567b2e",
        dark: "#a4be7a",
    },
];
