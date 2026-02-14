"""
Document generation macros.

Public API:
- generate: Build a file from a Rust DSL source file

Example:
    generate(
        src = "molten.document.rs",
        destination = "Molten/index.html",
        deps = ["//system/document:vantle"],
        data = ["//Molten/test/resource:data"],
    )
"""

load("@rules_rust//rust:defs.bzl", "rust_binary")
load(":rule.bzl", "document_generate")

def generate(src, destination, name = None, data = [], deps = [], **kwargs):
    """
    Build a file from a Rust DSL source file.

    Produces two targets:
      - {name}              - rust_binary that generates the file
      - document_{name}     - the generated file (via ctx.actions.run)

    Args:
        src: Rust source file
        destination: Workspace-relative output path (e.g., "Molten/Readme.html")
        name: Target name (derived from src if omitted)
        data: Runtime data files (code injection, WASM)
        deps: Additional compile deps (page-to-page deps)
        **kwargs: Standard Bazel attrs (visibility, tags, testonly)
    """
    if name == None:
        name = src.removesuffix(".document.rs") if src.endswith(".document.rs") else src.removesuffix(".rs")
    standard = ["//component:web", "//system/web:render", "@crates//:miette"]

    rust_binary(
        name = name,
        srcs = [src],
        deps = standard + deps,
        data = data,
        **{k: kwargs[k] for k in ["visibility", "tags", "testonly"] if k in kwargs}
    )

    document_generate(
        name = "document_" + name,
        binary = ":" + name,
        destination = destination,
        data = data,
        **{k: kwargs[k] for k in ["visibility", "tags"] if k in kwargs}
    )
