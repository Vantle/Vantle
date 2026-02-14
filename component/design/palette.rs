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
        light: "#9d174d",
        dark: "#c792ea",
    },
    Token {
        role: "syntax-entity",
        light: "#d45d00",
        dark: "#ff8c42",
    },
    Token {
        role: "syntax-string",
        light: "#166534",
        dark: "#c3e88d",
    },
    Token {
        role: "syntax-comment",
        light: "#6b7280",
        dark: "#9ca3af",
    },
    Token {
        role: "syntax-constant",
        light: "#b45309",
        dark: "#f78c6c",
    },
    Token {
        role: "syntax-storage",
        light: "#6d28d9",
        dark: "#82aaff",
    },
    Token {
        role: "syntax-punctuation",
        light: "#64748b",
        dark: "#89ddff",
    },
];
