"""
Document generation rule definitions.

Public document.generate rule for creating files from Rust document binaries.
"""

load(":impls.bzl", "generate_impl")
load(":types.bzl", "DocumentInfo")

document_generate = rule(
    implementation = generate_impl,
    provides = [DefaultInfo, DocumentInfo],
    attrs = {
        "binary": attr.label(
            mandatory = True,
            executable = True,
            cfg = "exec",
            doc = "Document binary that generates the output",
        ),
        "destination": attr.string(
            mandatory = True,
            doc = "Workspace-relative output path (e.g., 'Molten/index.html')",
        ),
        "data": attr.label_list(
            allow_files = True,
            doc = "Runtime data files (code injection, WASM)",
        ),
    },
    doc = "Generates a file by running a Rust document binary",
)
