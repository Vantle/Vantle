"""
W3C validation aspect using the Nu HTML Checker.

Validates HTML and CSS outputs from ValidationInfo targets during the build.
"""

load("@bazel_skylib//rules:common_settings.bzl", "BuildSettingInfo")
load("//component/generation/starlark:action.bzl", "action")

ValidationInfo = provider(
    doc = "Declares that a target should be validated",
    fields = {
        "output": "File to validate",
        "kind": "Validation kind: 'html' or 'css'",
    },
)

def _validation_impl(ctx):
    source = ctx.attr.src[DefaultInfo].files.to_list()[0]
    return [
        DefaultInfo(files = depset([source])),
        ValidationInfo(output = source, kind = ctx.attr.kind),
    ]

validation = rule(
    implementation = _validation_impl,
    attrs = {
        "src": attr.label(mandatory = True),
        "kind": attr.string(mandatory = True, values = ["html", "css"]),
    },
)

def _validate_impl(target, ctx):
    info = target[ValidationInfo]
    output = info.output
    kind = info.kind.capitalize()
    flags = "--%s " % info.kind

    json = ctx.actions.declare_file(target.label.name + ".validation.json")
    report = ctx.actions.declare_file(target.label.name + ".validation")
    validator = ctx.file._validator
    runtime = ctx.toolchains["@bazel_tools//tools/jdk:runtime_toolchain_type"].java_runtime
    renderer = ctx.executable._renderer

    ctx.actions.run_shell(
        inputs = depset(
            [output, validator],
            transitive = [runtime.files],
        ),
        outputs = [json],
        command = "{java} -jar {validator} --format json {flags}{input} > {json} 2>&1; true".format(
            java = runtime.java_executable_exec_path,
            validator = validator.path,
            flags = flags,
            input = output.path,
            json = json.path,
        ),
        mnemonic = "Check" + kind,
        progress_message = "Checking: %s" % output.short_path,
    )

    prefix = ctx.attr._symlink_prefix[BuildSettingInfo].value

    action(ctx, renderer, [
        "--source",
        output.path,
        "--report",
        json.path,
        "--prefix",
        prefix,
    ], [output, json], report, mnemonic = "Validate" + kind)

    return [OutputGroupInfo(validation = depset([report]))]

validate = aspect(
    implementation = _validate_impl,
    required_providers = [ValidationInfo],
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
