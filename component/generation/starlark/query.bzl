"""
Validate shell commands against real binaries at build time.

Public API:
- query: Validate a command binary with arguments, producing an extractable shell snippet
"""

load(":extract.bzl", "extract")

def _query_impl(ctx):
    output = ctx.outputs.output

    arguments = [
        "?",
        "--label",
        ctx.attr.label,
        "--output",
        output.path,
    ] + ctx.attr.arguments

    ctx.actions.run(
        executable = ctx.executable.binary,
        arguments = arguments,
        outputs = [output],
        mnemonic = "Query",
        progress_message = "Validating: %s" % ctx.attr.label,
    )

    return [DefaultInfo(files = depset([output]))]

_query = rule(
    implementation = _query_impl,
    attrs = {
        "binary": attr.label(
            mandatory = True,
            executable = True,
            cfg = "exec",
        ),
        "label": attr.string(mandatory = True),
        "arguments": attr.string_list(),
    },
    outputs = {"output": "%{name}.sh"},
)

def query(name, binary, label, arguments = [], visibility = None):
    """
    Validate a command binary with arguments and produce an extractable shell snippet.

    Runs the binary with ? at build time to validate arguments, then chains
    the output into extract to produce a rust_library with EXTRACTIONS.

    Args:
        name: Target name (becomes the crate name)
        binary: Binary target to validate
        label: Bazel label for the binary (used in the generated command string)
        arguments: CLI arguments to validate
        visibility: Bazel visibility
    """
    _query(
        name = name + ".query",
        binary = binary,
        label = label,
        arguments = arguments,
    )

    extract(
        name = name,
        source = ":" + name + ".query",
        language = "bash",
        visibility = visibility,
    )
