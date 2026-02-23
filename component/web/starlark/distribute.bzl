"""
Distribution rules for serving generated documentation.

Public API:
- copy: Wrap a raw file with SinkInfo for distribution
- folder: Assemble SinkInfo sources into a directory tree
- distribute: Serve a folder over HTTP
"""

load("//component/generation/starlark:action.bzl", "SinkInfo")

def _copy_impl(ctx):
    files = ctx.attr.src[DefaultInfo].files.to_list()
    if ctx.attr.filename:
        source = [f for f in files if f.basename == ctx.attr.filename][0]
    else:
        source = files[0]
    return [
        DefaultInfo(files = depset([source])),
        SinkInfo(path = ctx.attr.path),
    ]

copy = rule(
    implementation = _copy_impl,
    attrs = {
        "src": attr.label(mandatory = True, allow_files = True),
        "path": attr.string(mandatory = True),
        "filename": attr.string(),
    },
)

def _folder_impl(ctx):
    root = ctx.label.name
    outputs = []
    for src in ctx.attr.srcs:
        file = src[DefaultInfo].files.to_list()[0]
        path = src[SinkInfo].path
        output = ctx.actions.declare_file(root + "/" + path)
        ctx.actions.symlink(output = output, target_file = file)
        outputs.append(output)
    return [DefaultInfo(files = depset(outputs))]

folder = rule(
    implementation = _folder_impl,
    attrs = {
        "srcs": attr.label_list(mandatory = True, providers = [SinkInfo]),
    },
)

def _distribute_impl(ctx):
    executable = ctx.actions.declare_file(ctx.label.name)
    ctx.actions.symlink(output = executable, target_file = ctx.executable._server, is_executable = True)

    argfile = ctx.actions.declare_file("arguments")
    ctx.actions.write(output = argfile, content = "--input\n" + ctx.attr.folder.label.name + "\n")

    runfiles = ctx.runfiles(files = ctx.attr.folder[DefaultInfo].files.to_list() + [argfile])
    runfiles = runfiles.merge(ctx.attr._server[DefaultInfo].default_runfiles)

    return [DefaultInfo(executable = executable, runfiles = runfiles)]

distribute = rule(
    implementation = _distribute_impl,
    executable = True,
    attrs = {
        "folder": attr.label(mandatory = True),
        "_server": attr.label(
            default = "//system/distribution:command",
            executable = True,
            cfg = "exec",
        ),
    },
)
