"""
Serve rule for static file serving.

Creates a launcher that serves a directory over HTTP.
"""

def _serve_impl(ctx):
    directory = ctx.attr.directory if ctx.attr.directory else "$BUILD_WORKSPACE_DIRECTORY"
    arguments = ["--directory", "\"{}\"".format(directory)]

    if ctx.attr.address:
        arguments.extend(["--address", ctx.attr.address])

    if ctx.attr.port:
        arguments.extend(["--port", str(ctx.attr.port)])

    launcher = ctx.actions.declare_file(ctx.label.name)
    ctx.actions.write(
        output = launcher,
        content = "#!/usr/bin/env bash\nRUNFILES=\"${{RUNFILES_DIR:-$0.runfiles}}/_main\"\nexec \"$RUNFILES/{server}\" {arguments} \"$@\"\n".format(
            server = ctx.executable.server.short_path,
            arguments = " ".join(arguments),
        ),
        is_executable = True,
    )

    runfiles = ctx.runfiles()
    runfiles = runfiles.merge(ctx.attr.server[DefaultInfo].default_runfiles)

    return [DefaultInfo(
        executable = launcher,
        runfiles = runfiles,
    )]

serve = rule(
    implementation = _serve_impl,
    executable = True,
    attrs = {
        "server": attr.label(
            mandatory = True,
            executable = True,
            cfg = "exec",
            doc = "Static file server binary",
        ),
        "directory": attr.string(
            doc = "Directory to serve (defaults to workspace root)",
        ),
        "address": attr.string(
            doc = "Bind address (e.g., '0.0.0.0' for public, '127.0.0.1' for local)",
        ),
        "port": attr.int(
            doc = "Port to serve on",
        ),
    },
    doc = "Serves a directory over HTTP via a static file server",
)
