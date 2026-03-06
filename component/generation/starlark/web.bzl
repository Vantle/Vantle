"""
Document generation macros.

Public API:
- document: Build a file from a Rust DSL source file
"""

load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//component/web/starlark:aspect.bzl", "validation")
load(":action.bzl", "generate")

def document(src, destination, name = None, data = [], deps = [], compile_data = [], **kwargs):
    """
    Build a file from a Rust DSL source file.

    Produces targets:
      - {name}.binary       - rust_binary that generates the file
      - {name}              - the generated file (via generate rule)
      - {name}.validation   - validation marker (aspect attaches here)

    Args:
        src: Rust binary source file (.document.rs)
        destination: Workspace-relative output path (e.g., "Molten/index.html")
        name: Target name (derived from src if omitted)
        data: Runtime data files (code injection, WASM)
        deps: Compile deps (page libraries, extraction targets, etc.)
        compile_data: Compile-time data files (include_str! sources)
        **kwargs: Standard Bazel attrs (visibility, tags, testonly)
    """
    if name == None:
        name = src.removesuffix(".rs")

    passthrough = {k: kwargs[k] for k in ["visibility", "tags", "testonly"] if k in kwargs}

    standard = ["//system/generation/web:html", "@crates//:miette"]
    binary = name + ".binary"
    rust_binary(
        name = binary,
        crate_name = binary.replace(".", "_"),
        srcs = [src],
        compile_data = compile_data,
        deps = standard + deps,
        data = data,
        **passthrough
    )

    depth = destination.count("/")
    root = "../" * depth if depth > 0 else "./"

    generate(
        name = name,
        generator = ":" + name + ".binary",
        parameters = {"root": root},
        data = data,
        output = destination.replace("/", "."),
        sink = destination,
        **{k: v for k, v in passthrough.items() if k != "testonly"}
    )

    kind = "css" if destination.endswith(".css") else "html"
    validation(
        name = name + ".validation",
        src = ":" + name,
        kind = kind,
        **{k: v for k, v in passthrough.items() if k != "testonly"}
    )
