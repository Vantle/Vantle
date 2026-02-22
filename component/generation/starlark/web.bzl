"""
Document generation macros.

Public API:
- document: Build a file from a Rust DSL source file
"""

load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//component/web/starlark:aspect.bzl", "validation")
load(":action.bzl", "generate")

def document(srcs, destination, name = None, data = [], deps = [], compile_data = [], **kwargs):
    """
    Build a file from a Rust DSL source file.

    Produces three targets:
      - {name}.binary       - rust_binary that generates the file
      - {name}              - the generated file (via generate rule)
      - {name}.validation   - validation marker (aspect attaches here)

    Args:
        srcs: Rust source files
        destination: Workspace-relative output path (e.g., "Molten/index.html")
        name: Target name (derived from first src if omitted)
        data: Runtime data files (code injection, WASM)
        deps: Additional compile deps (page-to-page deps)
        compile_data: Compile-time data files (include_str! sources)
        **kwargs: Standard Bazel attrs (visibility, tags, testonly)
    """
    if name == None:
        name = srcs[0].removesuffix(".rs")
    standard = ["//component:web", "//system/generation/web:html", "@crates//:miette"]

    binary = name + ".binary"
    rust_binary(
        name = binary,
        crate_name = binary.replace(".", "_"),
        srcs = srcs,
        compile_data = compile_data,
        deps = standard + deps,
        data = data,
        **{k: kwargs[k] for k in ["visibility", "tags", "testonly"] if k in kwargs}
    )

    generate(
        name = name,
        generator = ":" + name + ".binary",
        parameters = {"destination": destination},
        data = data,
        output = destination.replace("/", "_"),
        sink = destination,
        **{k: kwargs[k] for k in ["visibility", "tags"] if k in kwargs}
    )

    kind = "css" if destination.endswith(".css") else "html"
    validation(
        name = name + ".validation",
        src = ":" + name,
        kind = kind,
        **{k: kwargs[k] for k in ["visibility", "tags"] if k in kwargs}
    )
