"""
Shared generation rule.

Public API:
- SinkInfo: Provider carrying workspace-relative output path
- action: Helper to execute a generator binary
- generate: Rule to run a generator binary and produce an output file
- execute: Rule to run a binary and capture its output
"""

SinkInfo = provider(
    doc = "Declares a workspace-relative output path for distribution",
    fields = {"path": "Workspace-relative output path"},
)

def action(ctx, generator, arguments, inputs, output, sink = None, mnemonic = "Generate"):
    """
    Execute a generator binary with CLI arguments.

    Appends --output automatically. When sink is provided, appends --sink
    and includes the sink file in outputs.

    Args:
        ctx: Rule context
        generator: Generator executable
        arguments: Flat list of CLI arguments (e.g., ["--flag", "value"])
        inputs: Input files for the action
        output: Declared output file
        sink: Optional declared sink output file
        mnemonic: Action mnemonic for build logs
    """
    outputs = [output]
    if sink:
        arguments = arguments + ["--sink", sink.path]
        outputs.append(sink)

    ctx.actions.run(
        executable = generator,
        arguments = arguments + ["--output", output.path],
        inputs = inputs,
        outputs = outputs,
        mnemonic = mnemonic,
        progress_message = "Generating: %s" % output.basename,
    )

def _generate_impl(ctx):
    output = ctx.outputs.output
    observation = ctx.actions.declare_file(output.basename + ".trace.jsonl")

    arguments = []
    for key, value in ctx.attr.parameters.items():
        arguments.extend(["--" + key, value])

    inputs = []
    for src in ctx.attr.srcs:
        for file in src.files.to_list():
            inputs.append(file)

    for dep in ctx.attr.data:
        for file in dep.files.to_list():
            arguments.extend(["--data", file.path])
            inputs.append(file)

    action(ctx, ctx.executable.generator, arguments, inputs, output, sink = observation)

    providers = [
        DefaultInfo(files = depset([output])),
        OutputGroupInfo(observation = depset([observation])),
    ]
    if ctx.attr.sink:
        providers.append(SinkInfo(path = ctx.attr.sink))
    return providers

generate = rule(
    implementation = _generate_impl,
    attrs = {
        "generator": attr.label(
            mandatory = True,
            executable = True,
            cfg = "exec",
        ),
        "parameters": attr.string_dict(),
        "srcs": attr.label_list(allow_files = True),
        "data": attr.label_list(allow_files = True),
        "output": attr.output(mandatory = True),
        "sink": attr.string(),
    },
)

def _execute_impl(ctx):
    output = ctx.outputs.output

    arguments = []
    for key, value in ctx.attr.parameters.items():
        arguments.extend(["--" + key, value])

    inputs = []
    for dep in ctx.attr.data:
        for file in dep.files.to_list():
            inputs.append(file)

    runfiles = ctx.attr.binary[DefaultInfo].default_runfiles
    if runfiles:
        inputs.extend(runfiles.files.to_list())

    if ctx.attr.allow_failure:
        parts = [ctx.executable.binary.path] + arguments
        parts.extend(["--output", output.path])
        ctx.actions.run_shell(
            command = " ".join(parts) + " || true",
            inputs = inputs,
            outputs = [output],
            tools = [ctx.executable.binary],
            mnemonic = "Execute",
            progress_message = "Executing: %s" % output.basename,
        )
    else:
        action(ctx, ctx.executable.binary, arguments, inputs, output, mnemonic = "Execute")

execute = rule(
    implementation = _execute_impl,
    attrs = {
        "binary": attr.label(
            mandatory = True,
            executable = True,
            cfg = "exec",
        ),
        "parameters": attr.string_dict(),
        "data": attr.label_list(allow_files = True),
        "output": attr.output(mandatory = True),
        "allow_failure": attr.bool(default = False),
    },
)
