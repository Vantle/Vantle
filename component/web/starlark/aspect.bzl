"""
W3C validation aspect using the Nu HTML Checker.

Validates HTML, CSS, and SVG outputs from ValidationInfo targets during the build.
"""

load("@bazel_skylib//rules:common_settings.bzl", "BuildSettingInfo")

ValidationInfo = provider(
    doc = "Declares that a target should be validated",
    fields = {
        "output": "File to validate",
        "kind": "Validation kind: 'html', 'css', or 'svg'",
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
        "kind": attr.string(mandatory = True, values = ["html", "css", "svg"]),
    },
)

def _validate_impl(target, ctx):
    info = target[ValidationInfo]
    output = info.output
    kind = info.kind.capitalize()

    report = ctx.actions.declare_file(target.label.name + ".validation")
    validator = ctx.file._validator
    runtime = ctx.toolchains["@bazel_tools//tools/jdk:runtime_toolchain_type"].java_runtime
    renderer = ctx.executable._renderer
    prefix = ctx.attr._symlink_prefix[BuildSettingInfo].value

    ctx.actions.run(
        executable = renderer,
        arguments = [
            "--source",
            output.path,
            "--java",
            runtime.java_executable_exec_path,
            "--validator",
            validator.path,
            "--kind",
            info.kind,
            "--prefix",
            prefix,
            "--output",
            report.path,
        ],
        inputs = depset(
            [output, validator],
            transitive = [runtime.files],
        ),
        outputs = [report],
        mnemonic = "Validate" + kind,
        progress_message = "Validating: %s" % output.short_path,
    )

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
