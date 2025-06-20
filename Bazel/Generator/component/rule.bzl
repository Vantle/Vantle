"""
Code generation rules for generating test files from templates and data at build time.

This module provides:
- A general generate rule that generates files from template and data
- rust_autotest macro for creating a Rust test target from a single data file

Example usage:
    rust_autotest(
        name = "my_test",
        data = ":test_data.json",
        template = ":test.template.rs",
        deps = ["//some:dependency"],
    )
    
    # Creates:
    # - my_test_template_validation (rust_library): validates template compiles
    # - my_test_generation (generate rule): test_data.json + template → test_data.generated.rs  
    # - my_test (rust_test): compiles and runs test_data.generated.rs
"""

load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

# Language configuration provider
LanguageInfo = provider(
    doc = "Configuration for a programming language in the codegen system",
    fields = {
        "extension": "File extension for generated files (e.g., 'rs' for Rust)",
    },
)

# Language configurations mapping
LANGUAGES = {
    "rust": LanguageInfo(extension = "rs"),
}

def _generate(ctx, template, data, language, generator, lang):
    """Helper function to generate a single file."""
    test_name = ctx.label.name

    # Remove _generation suffix if present
    if test_name.endswith("_generation"):
        test_name = test_name[:-len("_generation")]
    output_name = "{}.generated.{}".format(test_name, lang.extension)
    output = ctx.actions.declare_file(output_name)

    ctx.actions.run(
        executable = generator,
        arguments = [
            "--template",
            template.path,
            "--data",
            data.path,
            "--language",
            language,
            "--output",
            output.path,
        ],
        inputs = [template, data],
        outputs = [output],
        mnemonic = "Generator",
        progress_message = "Generating: {}".format(output.basename),
    )

    return output

def _generate_impl(ctx):
    """Code generation rule implementation that generates files."""

    # Get the rule's generate attributes
    template = ctx.file.template
    data = ctx.file.data
    language = ctx.attr.language
    generator = ctx.executable.generator

    # Get language configuration
    lang = LANGUAGES.get(language)
    if not lang:
        fail("Unsupported language: {}. Supported languages: {}".format(language, list(LANGUAGES.keys())))

    # The template validation dependency is implicit - if it's specified and fails to build,
    # this rule won't execute. We don't need to explicitly use it in the action.

    # Generate output file
    output = _generate(ctx, template, data, language, generator, lang)

    return [DefaultInfo(files = depset([output]))]

# Code generation rule that generates files from template and data
generate = rule(
    implementation = _generate_impl,
    attrs = {
        "template": attr.label(
            allow_single_file = True,
            mandatory = True,
            doc = "Template file to use for code generation",
        ),
        "data": attr.label(
            allow_single_file = True,
            mandatory = True,
            doc = "Data file for code generation",
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
    },
    doc = "Rule for code generation from a single template and data file",
)

def rust_autotest(name, template, data, generator = "//Bazel/Generator/system:generator", **kwargs):
    """
    Generate a Rust test file and create a test target from a single data file.

    Args:
        name: Name of the target
        template: Template .rs file to use for generation
        data: Single data file for test generation
        generator: Generator tool to use (defaults to standard generator)
        **kwargs: Additional attributes passed to pipeline targets
    """

    # Create template validation library
    validation_name = "{}_validation".format(name)
    rust_library(
        name = validation_name,
        srcs = [template],
        testonly = True,
        rustc_flags = kwargs.get("rustc_flags", []) + ["-A", "dead_code"],
        **{k: v for k, v in kwargs.items() if k not in ["visibility", "testonly", "rustc_flags", "srcs", "size", "timeout", "flaky", "shard_count", "local"]}
    )

    # Create generate rule for the data file
    generate_target_name = "{}_generation".format(name)
    generate(
        name = generate_target_name,
        template = template,
        data = data,  # Single data file
        language = "rust",
        generator = generator,
        deps = [":{}".format(validation_name)],  # Depend on template validation
        testonly = True,
        **{attr: kwargs[attr] for attr in ["visibility", "tags", "deprecation"] if attr in kwargs}
    )

    # Create rust_test that uses the generated file
    rust_test(
        name = name,
        srcs = [":{}".format(generate_target_name)],
        **kwargs
    )
