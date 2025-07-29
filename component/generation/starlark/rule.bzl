"""
Code generation rule definitions.

This file contains the generate rule for code generation from templates and data.
"""

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
            allow_single_file = True,
            mandatory = True,
            doc = "Template file to use for code generation",
        ),
        "cases": attr.label(
            allow_single_file = True,
            mandatory = True,
            doc = "Cases file for code generation",
        ),
        "language": attr.string(
            mandatory = True,
            doc = "Language for the generated code",
        ),
        "generator": attr.label(
            mandatory = True,
            executable = True,
            cfg = "exec",
            doc = "The generator tool to use for creating the files",
        ),
        "deps": attr.label_list(
            doc = "Dependencies that must be built before generation",
        ),
        "_symlink_prefix": attr.label(
            default = "//:symlink_prefix",
            doc = "Symlink prefix configuration",
        ),
    },
    doc = "Rule for code generation from a single template and data file",
)
