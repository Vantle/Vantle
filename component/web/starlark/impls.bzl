"""
Implementation for document generation rules.

Private implementations for the document_generate and publish rules.
"""

load(":types.bzl", "DocumentInfo")

_EXTENSIONS = [".js", ".wasm"]

def generate_impl(ctx):
    """
    Implementation for document_generate rule.

    Runs the document binary to produce a file.

    Args:
        ctx: Rule context

    Returns:
        Providers: DefaultInfo, DocumentInfo
    """
    output = ctx.actions.declare_file(ctx.attr.destination.replace("/", "_"))

    arguments = [
        "--output",
        output.path,
        "--destination",
        ctx.attr.destination,
    ]
    inputs = []

    for dep in ctx.attr.data:
        for file in dep.files.to_list():
            arguments.extend(["--data", file.path])
            inputs.append(file)

    ctx.actions.run(
        executable = ctx.executable.binary,
        arguments = arguments,
        inputs = inputs,
        outputs = [output],
        mnemonic = "Document",
        progress_message = "Generating: %s" % ctx.attr.destination,
    )

    return [
        DefaultInfo(files = depset([output])),
        DocumentInfo(
            output = output,
            destination = ctx.attr.destination,
        ),
    ]

def publish_impl(ctx, verify = False):
    """
    Aggregates DocumentInfo from page targets into a manifest and launches the publisher.

    Args:
        ctx: Rule context
        verify: When True, passes --verify to the publisher

    Returns:
        List of providers: [DefaultInfo]
    """
    outputs = []
    lines = []

    for page in ctx.attr.pages:
        if DocumentInfo in page:
            info = page[DocumentInfo]
            outputs.append(info.output)
            lines.append("{source}\t{destination}".format(
                source = info.output.short_path,
                destination = info.destination,
            ))

    for label, destination in ctx.attr.static.items():
        for file in label.files.to_list():
            if any([file.basename.endswith(extension) for extension in _EXTENSIONS]):
                outputs.append(file)
                lines.append("{source}\t{destination}".format(
                    source = file.short_path,
                    destination = destination + "/" + file.basename,
                ))

    manifest = ctx.actions.declare_file(ctx.label.name + ".manifest")
    ctx.actions.write(
        output = manifest,
        content = "\n".join(lines) + "\n",
    )

    suffix = " --verify" if verify else ""
    launcher = ctx.actions.declare_file(ctx.label.name)
    ctx.actions.write(
        output = launcher,
        content = "#!/usr/bin/env bash\nRUNFILES=\"${{RUNFILES_DIR:-$0.runfiles}}/_main\"\nexec \"$RUNFILES/{binary}\" --runfiles \"$RUNFILES\" --workspace \"$BUILD_WORKSPACE_DIRECTORY\" --manifest \"$RUNFILES/{manifest}\"{suffix} \"$@\"\n".format(
            binary = ctx.executable.generator.short_path,
            manifest = manifest.short_path,
            suffix = suffix,
        ),
        is_executable = True,
    )

    runfiles = ctx.runfiles(files = outputs + [manifest])
    runfiles = runfiles.merge(ctx.attr.generator[DefaultInfo].default_runfiles)

    return [
        DefaultInfo(
            executable = launcher,
            runfiles = runfiles,
        ),
    ]

PUBLISH_ATTRS = {
    "pages": attr.label_list(
        doc = "Document targets providing DocumentInfo",
    ),
    "static": attr.label_keyed_string_dict(
        doc = "Static files to copy: label -> workspace-relative destination directory",
    ),
    "generator": attr.label(
        mandatory = True,
        executable = True,
        cfg = "exec",
        doc = "Publish runner binary (system/document:publish)",
    ),
}
