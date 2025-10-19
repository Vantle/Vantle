"""
Implementation for code generation rules.

Private implementations for the generate rule.
"""

load("@bazel_skylib//rules:common_settings.bzl", "BuildSettingInfo")
load(":types.bzl", "GeneratedInfo", "LANGUAGES", "TemplateInfo")

def validate_inputs(ctx):
    """
    Validate required inputs for code generation.

    Args:
        ctx: Rule context
    """
    if not ctx.attr.template:
        fail(
            "Template not found for target '{}'. Please ensure '{}' exists and is a valid library target.".format(
                ctx.label,
                ctx.attr.template.label if hasattr(ctx.attr.template, "label") else ctx.attr.template,
            ),
        )

    if not ctx.file.cases:
        fail(
            "Cases file not found for target '{}'. Please ensure '{}' exists and is a valid file.".format(
                ctx.label,
                ctx.attr.data.label if hasattr(ctx.attr.data, "label") else ctx.attr.data,
            ),
        )

def generate_file(ctx, template, cases, language, generator, lang):
    """
    Execute generator to create test file from template and cases.

    Args:
        ctx: Rule context
        template: Template source file
        cases: Test cases data file
        language: Language string
        generator: Generator executable
        lang: Language configuration struct

    Returns:
        Generated test file
    """
    test_name = ctx.label.name
    if test_name.endswith("_generation"):
        test_name = test_name[:-len("_generation")]
    output_name = "{}.generated.{}".format(test_name, lang.extension)
    output = ctx.actions.declare_file(output_name)

    ctx.actions.run(
        executable = generator,
        arguments = [
            "--template",
            template.path,
            "--cases",
            cases.path,
            "--language",
            language,
            "--output",
            output.path,
        ],
        inputs = [template, cases],
        outputs = [output],
        mnemonic = "Generator",
        progress_message = "Generating: {}".format(output.basename),
        env = {
            "OUTPUT_PATH": ctx.attr._symlink_prefix[BuildSettingInfo].value + "bin/" + output.short_path,
        },
    )

    return output

def generate_impl(ctx):
    """
    Implementation for generate rule.

    Args:
        ctx: Rule context

    Returns:
        Providers: DefaultInfo, GeneratedInfo
    """

    validate_inputs(ctx)

    if TemplateInfo in ctx.attr.template:
        template_info = ctx.attr.template[TemplateInfo]
        template_file = template_info.source
        language = template_info.language
    else:
        fail("Template target '{}' must provide TemplateInfo. Use autotest_template or rust_autotest_template to create template targets.".format(ctx.attr.template.label))

    cases = ctx.file.cases
    generator = ctx.executable.generator

    if language not in LANGUAGES:
        fail("Unsupported language '{}' from template '{}'. Supported languages are: {}".format(
            language,
            ctx.attr.template.label,
            ", ".join(sorted(LANGUAGES.keys())),
        ))
    lang = LANGUAGES[language]

    output = generate_file(ctx, template_file, cases, language, generator, lang)

    return [
        DefaultInfo(files = depset([output])),
        GeneratedInfo(
            generated = output,
            template = template_file,
            cases = cases,
            language = language,
        ),
    ]
