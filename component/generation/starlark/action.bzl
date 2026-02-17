"""
Shared generation rule and provider.

Public API:
- GenerationInfo: Provider for generated file metadata
- action: Helper to execute a generator binary
- generate: Rule to run a generator binary and produce an output file
"""

load("@rules_rust_wasm_bindgen//:defs.bzl", "RustWasmBindgenInfo")

GenerationInfo = provider(
    doc = "Metadata about a generated file",
    fields = {
        "output": "The generated output file",
        "destination": "Intended destination path",
    },
)

def action(ctx, generator, arguments, inputs, output, mnemonic = "Generate"):
    """
    Execute a generator binary to produce an output file.

    Appends --output <path> to arguments automatically.

    Args:
        ctx: Rule context
        generator: Generator executable
        arguments: CLI arguments (before --output)
        inputs: Input files for the action
        output: Declared output file
        mnemonic: Action mnemonic for build logs
    """
    ctx.actions.run(
        executable = generator,
        arguments = arguments + ["--output", output.path],
        inputs = inputs,
        outputs = [output],
        mnemonic = mnemonic,
        progress_message = "Generating: %s" % output.basename,
    )

def _generate_impl(ctx):
    """
    Implementation for the generic generate rule.

    Args:
        ctx: Rule context

    Returns:
        Providers: DefaultInfo, GenerationInfo
    """
    output = ctx.actions.declare_file(ctx.attr.output)

    arguments = list(ctx.attr.arguments)
    inputs = []

    for src in ctx.attr.srcs:
        for file in src.files.to_list():
            inputs.append(file)

    for dep in ctx.attr.data:
        for file in dep.files.to_list():
            arguments.extend(["--data", file.path])
            inputs.append(file)

    action(ctx, ctx.executable.generator, arguments, inputs, output)

    return [
        DefaultInfo(files = depset([output])),
        GenerationInfo(
            output = output,
            destination = ctx.attr.destination if hasattr(ctx.attr, "destination") else output.basename,
        ),
    ]

generate = rule(
    implementation = _generate_impl,
    provides = [DefaultInfo, GenerationInfo],
    attrs = {
        "generator": attr.label(
            mandatory = True,
            executable = True,
            cfg = "exec",
            doc = "Generator binary to execute",
        ),
        "arguments": attr.string_list(
            doc = "CLI arguments passed before --output",
        ),
        "srcs": attr.label_list(
            allow_files = True,
            doc = "Input files (available as action inputs, referenced by arguments)",
        ),
        "data": attr.label_list(
            allow_files = True,
            doc = "Runtime data files (each passed as --data <path>)",
        ),
        "output": attr.string(
            mandatory = True,
            doc = "Output filename",
        ),
        "destination": attr.string(
            doc = "Intended destination path (for publish manifests)",
        ),
    },
    doc = "Run a generator binary to produce an output file",
)

def _asset_impl(ctx):
    source = ctx.file.src
    output = ctx.actions.declare_file(ctx.attr.destination.replace("/", "_"))
    ctx.actions.symlink(output = output, target_file = source)
    return [
        DefaultInfo(files = depset([output])),
        GenerationInfo(
            output = output,
            destination = ctx.attr.destination,
        ),
    ]

asset = rule(
    implementation = _asset_impl,
    provides = [DefaultInfo, GenerationInfo],
    attrs = {
        "src": attr.label(
            mandatory = True,
            allow_single_file = True,
            doc = "Source file to wrap with a destination",
        ),
        "destination": attr.string(
            mandatory = True,
            doc = "Workspace-relative destination path",
        ),
    },
    doc = "Wrap an existing file with GenerationInfo for publish manifests",
)

def _bindgen_impl(ctx):
    info = ctx.attr.src[RustWasmBindgenInfo]
    source = info.js.to_list()[0] if ctx.attr.kind == "js" else info.wasm
    destination = ctx.attr.directory + "/" + source.basename
    output = ctx.actions.declare_file(destination.replace("/", "_"))
    ctx.actions.symlink(output = output, target_file = source)
    return [
        DefaultInfo(files = depset([output])),
        GenerationInfo(output = output, destination = destination),
    ]

_bindgen = rule(
    implementation = _bindgen_impl,
    provides = [DefaultInfo, GenerationInfo],
    attrs = {
        "src": attr.label(mandatory = True, providers = [RustWasmBindgenInfo]),
        "kind": attr.string(mandatory = True, values = ["js", "wasm"]),
        "directory": attr.string(mandatory = True),
    },
)

def bindgen(name, src, directory, **kwargs):
    _bindgen(name = name + ".js", src = src, kind = "js", directory = directory, **kwargs)
    _bindgen(name = name + ".wasm", src = src, kind = "wasm", directory = directory, **kwargs)
