"""
W3C validation aspect using the Nu HTML Checker.

Validates HTML and CSS outputs from DocumentInfo targets during the build.
"""

load("@bazel_skylib//rules:common_settings.bzl", "BuildSettingInfo")
load(":types.bzl", "DocumentInfo")

def _validate_impl(target, ctx):
    info = target[DocumentInfo]
    path = info.output.path

    if path.endswith(".html"):
        kind = "Html"
        flags = ""
    elif path.endswith(".css"):
        kind = "Css"
        flags = "--css "
    else:
        return [OutputGroupInfo(validation = depset())]

    json = ctx.actions.declare_file(target.label.name + ".validation.json")
    report = ctx.actions.declare_file(target.label.name + ".validation")
    validator = ctx.file._validator
    runtime = ctx.toolchains["@bazel_tools//tools/jdk:runtime_toolchain_type"].java_runtime
    renderer = ctx.executable._renderer

    ctx.actions.run_shell(
        inputs = depset(
            [info.output, validator],
            transitive = [runtime.files],
        ),
        outputs = [json],
        command = "{java} -jar {validator} --format json {flags}{input} > {json} 2>&1; true".format(
            java = runtime.java_executable_exec_path,
            validator = validator.path,
            flags = flags,
            input = info.output.path,
            json = json.path,
        ),
        mnemonic = "Check" + kind,
        progress_message = "Checking: %s" % info.destination,
    )

    prefix = ctx.attr._symlink_prefix[BuildSettingInfo].value

    ctx.actions.run(
        executable = renderer,
        arguments = [
            "--source",
            info.output.path,
            "--report",
            json.path,
            "--output",
            report.path,
            "--prefix",
            prefix,
        ],
        inputs = [info.output, json],
        outputs = [report],
        mnemonic = "Validate" + kind,
        progress_message = "Validating: %s" % info.destination,
    )

    return [OutputGroupInfo(validation = depset([report]))]

validate = aspect(
    implementation = _validate_impl,
    required_providers = [DocumentInfo],
    toolchains = ["@bazel_tools//tools/jdk:runtime_toolchain_type"],
    attrs = {
        "_validator": attr.label(
            default = "@vnu//:package/build/dist/vnu.jar",
            allow_single_file = True,
        ),
        "_renderer": attr.label(
            default = "//component/web:validate",
            executable = True,
            cfg = "exec",
        ),
        "_symlink_prefix": attr.label(
            default = "//:symlink_prefix",
            providers = [BuildSettingInfo],
        ),
    },
)
