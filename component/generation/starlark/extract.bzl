"""
Extract code snippets from source files via tree-sitter queries.

Public API:
- extract: Run a tree-sitter query against a source file, producing a rust_library
"""

load("@rules_rust//rust:defs.bzl", "rust_library")
load(":action.bzl", "SinkInfo")

def _source_impl(ctx):
    source = ctx.file.source
    return [
        DefaultInfo(files = depset([source])),
        SinkInfo(path = source.short_path),
    ]

_source = rule(
    implementation = _source_impl,
    attrs = {
        "source": attr.label(mandatory = True, allow_single_file = True),
    },
)

def _extract_impl(ctx):
    output = ctx.outputs.output
    source = ctx.file.source

    arguments = [
        "--source",
        source.path,
        "--language",
        ctx.attr.language,
        "--label",
        source.short_path,
        "--output",
        output.path,
    ]
    if ctx.attr.query:
        arguments += ["--query", ctx.attr.query]
    else:
        arguments += ["--query", ""]

    ctx.actions.run(
        executable = ctx.executable._extractor,
        arguments = arguments,
        inputs = [source],
        outputs = [output],
        mnemonic = "Extract",
        progress_message = "Extracting: %s" % ctx.attr.name,
    )

    return [DefaultInfo(files = depset([output]))]

_extract = rule(
    implementation = _extract_impl,
    attrs = {
        "source": attr.label(
            mandatory = True,
            allow_single_file = True,
        ),
        "query": attr.string(default = ""),
        "language": attr.string(mandatory = True),
        "_extractor": attr.label(
            default = "//system/graph/analysis/extract:command",
            executable = True,
            cfg = "exec",
        ),
    },
    outputs = {"output": "%{name}.generated.rs"},
)

def extract(name, source, language, query = "", visibility = None):
    """
    Extract a code snippet from a source file via tree-sitter query.

    Produces a rust_library with pub static EXTRACTIONS: &[Extraction].
    When query is empty, embeds the entire file.

    Args:
        name: Target name (becomes the crate name)
        source: Source file to query
        language: Source language (rust, starlark, bash, json, molten)
        query: Tree-sitter query with @capture pattern (empty for whole file)
        visibility: Bazel visibility
    """
    kwargs = {}
    if visibility != None:
        kwargs["visibility"] = visibility

    _extract(
        name = name + ".generate",
        source = source,
        query = query,
        language = language,
    )

    crate_name = name.replace(".", "_").replace("-", "_")
    rust_library(
        name = name,
        srcs = [":" + name + ".generate"],
        crate_name = crate_name,
        deps = [
            "//component/web:extraction",
            "//component/web:language",
        ],
        **kwargs
    )

    _source(
        name = name + ".source",
        source = source,
        **kwargs
    )
