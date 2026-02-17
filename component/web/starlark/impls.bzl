"""
Implementation for document publishing rules.

Private implementations for the publish and verify rules.
"""

load("//component/generation/starlark:action.bzl", "GenerationInfo")

def publish_impl(ctx, verify = False):
    """
    Aggregates GenerationInfo from source targets into a manifest and launches the publisher.

    Args:
        ctx: Rule context
        verify: When True, passes --verify to the publisher

    Returns:
        List of providers: [DefaultInfo]
    """
    outputs = []
    lines = []

    for src in ctx.attr.srcs:
        info = src[GenerationInfo]
        outputs.append(info.output)
        lines.append("{source}\t{destination}".format(
            source = info.output.short_path,
            destination = info.destination,
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
            binary = ctx.executable._generator.short_path,
            manifest = manifest.short_path,
            suffix = suffix,
        ),
        is_executable = True,
    )

    runfiles = ctx.runfiles(files = outputs + [manifest])
    runfiles = runfiles.merge(ctx.attr._generator[DefaultInfo].default_runfiles)

    return [
        DefaultInfo(
            executable = launcher,
            runfiles = runfiles,
        ),
    ]

PUBLISH_ATTRS = {
    "srcs": attr.label_list(
        mandatory = True,
        providers = [GenerationInfo],
        doc = "Targets providing GenerationInfo (documents, assets, stylesheets)",
    ),
    "_generator": attr.label(
        default = "//system/generation:source",
        executable = True,
        cfg = "exec",
    ),
}
