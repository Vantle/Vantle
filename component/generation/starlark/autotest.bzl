"""
Autotest code generation macros and rules.

Public API:
- autotest_template: Generic template creation
- autotest: Generic test generation
- rust_autotest_template: Rust template with defaults
- rust_autotest: Rust test generation with defaults

Example:
    rust_autotest_template(
        name = "template",
        src = "test.template.rs",
        deps = [],  # Optional custom deps
    )

    rust_autotest(
        name = "my_test",
        template = ":template",
        cases = ":cases.json",
        deps = [],  # Optional custom deps (must match template if customized)
    )
"""

load("@bazel_skylib//rules:common_settings.bzl", "BuildSettingInfo")
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")
load(":action.bzl", "GenerationInfo", "action")

#############################################################################
# PROVIDERS
#############################################################################

TemplateInfo = provider(
    doc = "Template metadata for code generation",
    fields = {
        "source": "Template source file",
        "language": "Language string (e.g., 'rust')",
        "deps": "Template dependencies",
    },
)

#############################################################################
# LANGUAGE CONFIGURATION
#############################################################################

def Language(extension, test, library, deps = [], flags = []):
    """
    Create a language configuration.

    Args:
        extension: File extension (e.g., 'rs')
        test: Test rule function (e.g., rust_test)
        library: Library rule function (e.g., rust_library)
        deps: Standard dependencies
        flags: Standard compiler flags

    Returns:
        Language configuration struct
    """
    return struct(
        extension = extension,
        test = test,
        library = library,
        deps = deps,
        flags = flags,
    )

LANGUAGES = {
    "rust": Language(
        extension = "rs",
        test = rust_test,
        library = rust_library,
        deps = [
            "@crates//:miette",
            "@crates//:serde",
            "@crates//:serde_json",
            "//:module",
            "//system:diagnostic",
            "//system/generation/runtime:runtime",
        ],
    ),
}

#############################################################################
# INTERNAL RULES
#############################################################################

def _autotest_template_provider_impl(ctx):
    return [
        DefaultInfo(files = depset([ctx.file.src])),
        TemplateInfo(
            source = ctx.file.src,
            language = ctx.attr.language,
            deps = ctx.attr.deps,
        ),
        ctx.attr.library[OutputGroupInfo],
    ]

_autotest_template_provider = rule(
    implementation = _autotest_template_provider_impl,
    attrs = {
        "src": attr.label(
            allow_single_file = True,
            mandatory = True,
            doc = "Template source file",
        ),
        "language": attr.string(
            mandatory = True,
            doc = "Template language string",
        ),
        "deps": attr.label_list(
            doc = "Template dependencies",
        ),
        "library": attr.label(
            mandatory = True,
            doc = "Validation library target",
        ),
    },
    doc = "Internal rule providing TemplateInfo for code generation templates",
)

def _autotest_generate_impl(ctx):
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
                ctx.attr.cases.label if hasattr(ctx.attr.cases, "label") else ctx.attr.cases,
            ),
        )

    if TemplateInfo in ctx.attr.template:
        template_info = ctx.attr.template[TemplateInfo]
        template_file = template_info.source
        language = template_info.language
    else:
        fail("Template target '{}' must provide TemplateInfo. Use autotest_template or rust_autotest_template to create template targets.".format(ctx.attr.template.label))

    cases = ctx.file.cases
    prefix = ctx.attr._symlink_prefix[BuildSettingInfo].value

    if language not in LANGUAGES:
        fail("Unsupported language '{}' from template '{}'. Supported languages are: {}".format(
            language,
            ctx.attr.template.label,
            ", ".join(sorted(LANGUAGES.keys())),
        ))
    lang = LANGUAGES[language]

    test_name = ctx.label.name
    if test_name.endswith(".generation"):
        test_name = test_name[:-len(".generation")]
    output = ctx.actions.declare_file("{}.generated.{}".format(test_name, lang.extension))

    action(ctx, ctx.executable.generator, [
        "--template",
        template_file.path,
        "--cases",
        cases.path,
        "--language",
        language,
        "--prefix",
        prefix,
    ], [template_file, cases], output, mnemonic = "Generator")

    return [
        DefaultInfo(files = depset([output])),
        GenerationInfo(
            output = output,
            destination = output.basename,
        ),
    ]

_autotest_generate = rule(
    implementation = _autotest_generate_impl,
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

#############################################################################
# PUBLIC MACROS
#############################################################################

def autotest_template(name, src, language, library, deps = [], **kwargs):
    """
    Create a template target that validates compilation and provides metadata.

    Language-agnostic base. Use language-specific wrappers (rust_autotest_template) for convenience.

    Args:
        name: Template target name (typically "template")
        src: Template source file
        language: Language string (e.g., "rust")
        library: Library rule function (e.g., rust_library)
        deps: Template dependencies
        **kwargs: Passed to library rule
    """

    library_name = "{}.library".format(name)
    crate_name = "{}_library".format(name)
    library(
        name = library_name,
        crate_name = crate_name,
        srcs = [src],
        deps = deps,
        testonly = True,
        **kwargs
    )

    _autotest_template_provider(
        name = name,
        src = src,
        language = language,
        deps = deps,
        library = ":{}".format(library_name),
        testonly = True,
    )

def autotest(name, template, cases, language, generator = "//system/generation:generator", **kwargs):
    """
    Create a test target from a template and test cases.

    Language-agnostic base. Use language-specific wrappers (rust_autotest) for convenience.

    Args:
        name: Test target name
        template: Template target (created with autotest_template)
        cases: Test case data file (JSON)
        language: Language name or Language struct
        generator: Generator binary (default: //system/generation:generator)
        **kwargs: Passed to test target (deps, data, visibility, etc.)
    """

    if type(language) == "string":
        target = LANGUAGES.get(language)
        if not target:
            fail("Unsupported language '{}' for '{}'. Supported: {}".format(
                language,
                name,
                ", ".join(sorted(LANGUAGES.keys())),
            ))
    else:
        target = language

    if not template:
        fail("autotest requires 'template' for '{}'".format(name))
    if not cases:
        fail("autotest requires 'cases' for '{}'".format(name))

    generate_target = "{}.generation".format(name)

    _autotest_generate(
        name = generate_target,
        template = template,
        cases = cases,
        generator = generator,
        deps = [template],
        testonly = True,
        **{attr: kwargs[attr] for attr in ["visibility", "tags", "deprecation"] if attr in kwargs}
    )

    if "timeout" not in kwargs:
        kwargs["timeout"] = "short"

    target.test(
        name = name,
        srcs = [":{}".format(generate_target)],
        **kwargs
    )

def rust_autotest_template(name, src, deps = [], **kwargs):
    """
    Create a Rust template that validates compilation and enables IDE support.

    Automatically adds `-A dead_code` flag. Templates are compiled as rust_library,
    enabling rust-analyzer and other tooling to understand the code.

    Args:
        name: Template target name (typically "template")
        src: Template source file
        deps: Template dependencies
        **kwargs: Passed to rust_library
    """

    rustc_flags = kwargs.get("rustc_flags", [])
    if "-A" not in " ".join(rustc_flags):
        rustc_flags = rustc_flags + ["-A", "dead_code"]
    kwargs["rustc_flags"] = rustc_flags

    rust_language = LANGUAGES.get("rust")
    merged = depset(deps + rust_language.deps).to_list()
    kwargs["deps"] = merged

    autotest_template(
        name = name,
        src = src,
        language = "rust",
        library = rust_library,
        **kwargs
    )

def rust_autotest(name, template, cases, generator = "//system/generation:generator", deps = [], **kwargs):
    """
    Generate and run Rust tests from a template and test cases.

    Standard deps auto-included: serde, serde_json, vantle, component, utility.
    Only specify deps if template has custom dependencies beyond defaults.

    Args:
        name: Test target name
        template: Template target (created with rust_autotest_template)
        cases: Test case data file (JSON)
        generator: Generator binary (default: //system/generation:generator)
        deps: Custom dependencies (only if template uses non-default deps)
        **kwargs: Passed to rust_test (data, visibility, etc.)
    """

    rust_language = LANGUAGES.get("rust")
    standard_deps = rust_language.deps
    merged = depset(deps + standard_deps).to_list()
    kwargs["deps"] = merged

    autotest(
        name = name,
        template = template,
        cases = cases,
        language = "rust",
        generator = generator,
        **kwargs
    )
