"""
Code generation rule definitions.

Public generate rule for creating test files from templates and test cases.
"""

load("@bazel_skylib//rules:common_settings.bzl", "BuildSettingInfo")
load(":impls.bzl", "generate_impl")
load(":types.bzl", "GeneratedInfo")

#############################################################################
# PUBLIC RULES
#############################################################################

generate = rule(
    implementation = generate_impl,
    provides = [DefaultInfo, GeneratedInfo],
    attrs = {
        "template": attr.label(
            mandatory = True,
            allow_files = True,
            doc = "Template target providing TemplateInfo",
        ),
        "cases": attr.label(
            allow_single_file = True,
            mandatory = True,
            doc = "Test cases data file (JSON)",
        ),
        "generator": attr.label(
            mandatory = True,
            executable = True,
            cfg = "exec",
            doc = "Generator binary for creating test files",
        ),
        "deps": attr.label_list(
            doc = "Build dependencies (typically includes template)",
        ),
        "_symlink_prefix": attr.label(
            default = "//:symlink_prefix",
            providers = [BuildSettingInfo],
        ),
    },
    doc = "Generates test code from a template and test cases",
)
