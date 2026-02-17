"""
Document generation macros.

Public API:
- document: Build a file from a Rust DSL source file

Example:
    document(
        srcs = ["molten.document.rs"],
        destination = "Molten/index.html",
        deps = ["//system/generation/web:style"],
        data = ["//Molten/test/resource:data"],
    )
"""

load("@rules_rust//rust:defs.bzl", "rust_binary")
load(":action.bzl", "generate")

def document(srcs, destination, name = None, data = [], deps = [], compile_data = [], **kwargs):
    """
    Build a file from a Rust DSL source file.

    Produces two targets:
      - {name}              - rust_binary that generates the file
      - document.{name}     - the generated file (via generate rule)

    Args:
        srcs: Rust source files
        destination: Workspace-relative output path (e.g., "Molten/Readme.html")
        name: Target name (derived from first src if omitted)
        data: Runtime data files (code injection, WASM)
        deps: Additional compile deps (page-to-page deps)
        compile_data: Compile-time data files (include_str! sources)
        **kwargs: Standard Bazel attrs (visibility, tags, testonly)
    """
    if name == None:
        src = srcs[0]
        name = src.removesuffix(".document.rs") if src.endswith(".document.rs") else src.removesuffix(".rs")
    standard = ["//component:web", "//system/generation/web:html", "@crates//:miette"]

    rust_binary(
        name = name,
        srcs = srcs,
        compile_data = compile_data,
        deps = standard + deps,
        data = data,
        **{k: kwargs[k] for k in ["visibility", "tags", "testonly"] if k in kwargs}
    )

    generate(
        name = "document." + name,
        generator = ":" + name,
        arguments = ["--destination", destination],
        data = data,
        output = destination.replace("/", "_"),
        destination = destination,
        **{k: kwargs[k] for k in ["visibility", "tags"] if k in kwargs}
    )
