"""
Implementation functions for code generation rules.

This file contains the private implementation details for the generate rule.
"""

load(":types.bzl", "GeneratedInfo", "LANGUAGES")

def validate_inputs(ctx):
    """Validate rule inputs and provide helpful error messages.

    Args:
      ctx: The rule context containing attributes and files to validate.
    """
    if not ctx.file.template:
        fail(
            "Template file not found for target '{}'. Please ensure '{}' exists and is a valid file.".format(
                ctx.label,
                ctx.attr.template.label if hasattr(ctx.attr.template, "label") else ctx.attr.template,
            ),
        )

    if not ctx.file.data:
        fail(
            "Data file not found for target '{}'. Please ensure '{}' exists and is a valid file.".format(
                ctx.label,
                ctx.attr.data.label if hasattr(ctx.attr.data, "label") else ctx.attr.data,
            ),
        )

    if ctx.attr.language not in LANGUAGES:
        fail(
            "Unsupported language '{}' for target '{}'. Supported languages are: {}. ".format(
                ctx.attr.language,
                ctx.label,
                ", ".join(sorted(LANGUAGES.keys())),
            ) + "Please use one of the supported languages or add a new language configuration to LANGUAGES.",
        )

def generate_file(ctx, template, data, language, generator, lang):
    """
    Helper function to generate a single file.

    Args:
      ctx: The rule context.
      template: The template file to use for generation.
      data: The data file containing test cases.
      language: The target language for generation.
      generator: The generator executable.
      lang: Language configuration from LANGUAGES dict.

    Returns:
      The generated output file.
    """
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

def generate_impl(ctx):
    """
    Code generation rule implementation that generates files.

    Args:
      ctx: The rule context containing attributes and configuration.

    Returns:
      A list of providers including DefaultInfo and GeneratedInfo.
    """

    # Validate inputs first
    validate_inputs(ctx)

    # Get the rule's generate attributes
    template = ctx.file.template
    data = ctx.file.data
    language = ctx.attr.language
    generator = ctx.executable.generator

    # Get language configuration (already validated)
    lang = LANGUAGES[language]

    # The template validation dependency is implicit - if it's specified and fails to build,
    # this rule won't execute. We don't need to explicitly use it in the action.

    # Generate output file
    output = generate_file(ctx, template, data, language, generator, lang)

    return [
        DefaultInfo(files = depset([output])),
        GeneratedInfo(
            generated = output,
            template = template,
            data = data,
            language = language,
        ),
    ]
