use body::Chain;
use extraction::Query;
use navigation::Composition;

pub fn page(root: &str) -> page::Result {
    navigation::layout("Extract", &index::generation::extract(root), root, |c| {
        c.title("Extract")
            .subtitle("Code extraction via tree-sitter queries")
            .rule()
            .paragraph(|p| {
                p.text("The ")
                    .bold("Extract")
                    .text(" system pulls code snippets from source files at build time using tree-sitter queries. Extracted snippets become ")
                    .code("EXTRACTIONS")
                    .text(" constants in generated Rust libraries, enabling documentation pages to embed live source code that stays in sync with the codebase.")
            })
            .rule()
            .section("Extract", |s| {
                s.paragraph(|p| {
                    p.text("The ")
                        .code("extract")
                        .text(" rule runs a tree-sitter query against a source file and produces a ")
                        .code("rust_library")
                        .text(" with a ")
                        .code("pub static EXTRACTIONS")
                        .text(" array.")
                })
                .extract(extract_document::EXTRACTIONS.one())
                .table(|t| {
                    t.header(["Parameter", "Description"])
                        .describe("name", "Target name (becomes the crate name)")
                        .describe("source", "Source file to extract from")
                        .describe("language", "Source language for parsing")
                        .describe("query", "Tree-sitter query with @capture (empty for whole file)")
                        .describe("visibility", "Bazel visibility")
                })
                .paragraph(|p| {
                    p.text("When ")
                        .code("query")
                        .text(" is empty, the entire file is embedded. Each ")
                        .code("extract")
                        .text(" target also produces a ")
                        .code(".source")
                        .text(" target for linking to the original file in rendered documentation.")
                })
            })
            .rule()
            .section("Query", |s| {
                s.paragraph(|p| {
                    p.text("The ")
                        .code("query")
                        .text(" rule validates a command binary at build time and produces an extractable shell snippet. It chains into ")
                        .code("extract")
                        .text(" to generate a ")
                        .code("rust_library")
                        .text(" with the validated command string.")
                })
                .extract(query_document::EXTRACTIONS.one())
                .table(|t| {
                    t.header(["Parameter", "Description"])
                        .describe("name", "Target name (becomes the crate name)")
                        .describe("binary", "Binary target to validate")
                        .describe("label", "Bazel label for the generated command string")
                        .describe("arguments", "CLI arguments to validate")
                        .describe("visibility", "Bazel visibility")
                })
                .paragraph(|p| {
                    p.text("The binary is invoked with ")
                        .code("?")
                        .text(" at build time to validate arguments without executing the full command. This ensures all documented commands are syntactically valid against their binaries.")
                })
            })
            .rule()
            .section("Language", |s| {
                s.paragraph(|p| {
                    p.text("Extraction supports multiple source languages via tree-sitter grammars:")
                })
                .table(|t| {
                    t.header(["Language", "Extensions", "Grammar"])
                        .row(["Rust", ".rs", "tree-sitter-rust"])
                        .row(["Starlark", ".bzl, .bazel, .star", "tree-sitter-python"])
                        .row(["Bash", ".sh", "tree-sitter-bash"])
                        .row(["JSON", ".json", "tree-sitter-json"])
                        .row(["Molten", ".magma, .lava", "tree-sitter-molten"])
                })
            })
            .rule()
            .section("Pattern", |s| {
                s.paragraph(|p| {
                    p.text("Tree-sitter queries use S-expression syntax. The ")
                        .code("@capture")
                        .text(" name marks which nodes to extract:")
                })
                .list(|ul| {
                    ul.item(|s| {
                        s.code("(function_item name: (identifier) @name (#eq? @name \"target\")) @capture")
                            .text(" \u{2014} extract a Rust function by name")
                    })
                    .item(|s| {
                        s.code("(call function: (identifier) @fn (#eq? @fn \"rule\")) @capture")
                            .text(" \u{2014} extract a Starlark rule invocation")
                    })
                    .item(|s| {
                        s.code("(attribute_item) @capture . (function_item) @capture")
                            .text(" \u{2014} extract a function with its preceding attribute")
                    })
                })
                .paragraph(|p| {
                    p.text("All queries must include at least one ")
                        .code("@capture")
                        .text(" binding. Multiple ")
                        .code("@capture")
                        .text(" nodes within the same match are merged into a single extraction.")
                })
            })
    })
}
