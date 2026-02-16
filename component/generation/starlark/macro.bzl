"""
Code generation macros for test generation from templates and test cases.

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

load("@rules_rust//rust:defs.bzl", "rust_library")
load(":rule.bzl", "generate")
load(":types.bzl", "LANGUAGES", "TemplateInfo")

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

def _autotest_template_provider_impl(ctx):
    """Internal implementation providing TemplateInfo."""
    return [
        DefaultInfo(files = depset([ctx.file.src])),
        TemplateInfo(
            source = ctx.file.src,
            language = ctx.attr.language,
            test = ctx.attr.language,
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

    generate(
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
