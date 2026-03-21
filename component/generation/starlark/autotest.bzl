"""
Autotest code generation macros and rules.

Public API:
- autotest_template: Generic template creation
- autotest_function: Generic function test generation
- autotest_performance: Generic performance test generation
- rust_autotest_template: Rust template with defaults
- rust_autotest_function: Rust function test generation with defaults
- rust_autotest_performance: Rust performance test generation with defaults

Example:
    rust_autotest_template(
        name = "template",
        src = "test.template.rs",
        deps = [],  # Optional custom deps
    )

    rust_autotest_function(
        name = "my_test",
        template = ":template",
        cases = ":cases.json",
        deps = [],  # Optional custom deps (must match template if customized)
    )

    rust_autotest_performance(
        name = "my_perf_test",
        template = ":template",
        cases = ":cases.json",
        specification = ":specification.json",
        deps = [],
    )
"""

load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")
load(":action.bzl", "SinkInfo", "action", "execute", "generate")

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

def Language(extension, test, library, deps = [], proc_macro_deps = [], flags = []):
    """
    Create a language configuration.

    Args:
        extension: File extension (e.g., 'rs')
        test: Test rule function (e.g., rust_test)
        library: Library rule function (e.g., rust_library)
        deps: Standard dependencies
        proc_macro_deps: Standard proc macro dependencies
        flags: Standard compiler flags

    Returns:
        Language configuration struct
    """
    return struct(
        extension = extension,
        test = test,
        library = library,
        deps = deps,
        proc_macro_deps = proc_macro_deps,
        flags = flags,
    )

LANGUAGES = {
    "rust": Language(
        extension = "rs",
        test = rust_test,
        library = rust_library,
        deps = [],
        proc_macro_deps = [
            "//system/observation:macro",
        ],
    ),
}

_CRATE = struct(
    miette = "@crates//:miette",
    nalgebra = "@crates//:nalgebra",
    serde = "@crates//:serde",
    serde_json = "@crates//:serde_json",
)

_SYSTEM = struct(
    command = "//system:command",
    diagnostic = "//system:diagnostic",
    observation = "//system:observation",
    runtime = "//system/generation/runtime:runtime",
)

_HARNESS = struct(
    function = "//test/system:function",
    performance = "//test/system:performance",
    regression = "//component/math:regression",
    utility = "//test:utility",
)

_BASE = [
    _CRATE.miette,
    _CRATE.serde,
    _CRATE.serde_json,
    _SYSTEM.command,
    _SYSTEM.diagnostic,
    _SYSTEM.observation,
    _SYSTEM.runtime,
]

dependencies = struct(
    template = [_CRATE.serde],
    rust = struct(
        function = _BASE + [
            _HARNESS.function,
            _HARNESS.utility,
        ],
        performance = _BASE + [
            _CRATE.nalgebra,
            _HARNESS.performance,
            _HARNESS.regression,
        ],
    ),
)

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

def _validate(ctx):
    if TemplateInfo in ctx.attr.template:
        template_info = ctx.attr.template[TemplateInfo]
        template_file = template_info.source
        language = template_info.language
    else:
        fail("Template target '{}' must provide TemplateInfo. Use autotest_template or rust_autotest_template to create template targets.".format(ctx.attr.template.label))

    cases = ctx.file.cases
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

    return template_file, language, cases, lang, output

def _autotest_function_generate_impl(ctx):
    template_file, language, cases, _lang, output = _validate(ctx)

    arguments = [
        "--template",
        template_file.path,
        "--cases",
        cases.path,
        "--language",
        language,
    ]
    inputs = [template_file, cases]

    action(ctx, ctx.executable.generator, arguments, inputs, output, mnemonic = "Generator")

    return [DefaultInfo(files = depset([output]))]

_autotest_function_generate = rule(
    implementation = _autotest_function_generate_impl,
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
    },
    doc = "Generates function test code from a template and test cases",
)

def _autotest_performance_generate_impl(ctx):
    template_file, language, cases, _lang, output = _validate(ctx)

    arguments = [
        "--template",
        template_file.path,
        "--cases",
        cases.path,
        "--language",
        language,
        "--specification",
        ctx.file.specification.path,
    ]
    inputs = [template_file, cases, ctx.file.specification]

    action(ctx, ctx.executable.generator, arguments, inputs, output, mnemonic = "Generator")

    return [DefaultInfo(files = depset([output]))]

_autotest_performance_generate = rule(
    implementation = _autotest_performance_generate_impl,
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
        "specification": attr.label(
            allow_single_file = True,
            mandatory = True,
            doc = "Performance specification file (JSON)",
        ),
    },
    doc = "Generates performance test code from a template, test cases, and specification",
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

    testonly = kwargs.pop("testonly", True)

    library_name = "{}.library".format(name)
    crate_name = "{}_library".format(name)
    library(
        name = library_name,
        crate_name = crate_name,
        srcs = [src],
        deps = deps,
        testonly = testonly,
        **kwargs
    )

    _autotest_template_provider(
        name = name,
        src = src,
        language = language,
        deps = deps,
        library = ":{}".format(library_name),
        testonly = testonly,
    )

def _resolve_language(language, name):
    if type(language) == "string":
        target = LANGUAGES.get(language)
        if not target:
            fail("Unsupported language '{}' for '{}'. Supported: {}".format(
                language,
                name,
                ", ".join(sorted(LANGUAGES.keys())),
            ))
        return target
    return language

def autotest_function(name, template, cases, language, generator = "//system/generation:generator", **kwargs):
    """
    Create a function test target from a template and test cases.

    Language-agnostic base. Use language-specific wrappers (rust_autotest_function) for convenience.

    Args:
        name: Test target name
        template: Template target (created with autotest_template)
        cases: Test case data file (JSON)
        language: Language name or Language struct
        generator: Generator binary (default: //system/generation:generator)
        **kwargs: Passed to test target (deps, data, visibility, etc.)
    """

    testonly = kwargs.pop("testonly", True)

    target = _resolve_language(language, name)

    if not template:
        fail("autotest_function requires 'template' for '{}'".format(name))
    if not cases:
        fail("autotest_function requires 'cases' for '{}'".format(name))

    generate_target = "{}.generation".format(name)

    generate_attrs = {attr: kwargs[attr] for attr in ["visibility", "tags", "deprecation"] if attr in kwargs}

    _autotest_function_generate(
        name = generate_target,
        template = template,
        cases = cases,
        generator = generator,
        deps = [template],
        testonly = testonly,
        **generate_attrs
    )

    if "timeout" not in kwargs:
        kwargs["timeout"] = "short"

    proc_macro_deps = kwargs.pop("proc_macro_deps", [])
    proc_macro_deps = depset(proc_macro_deps + target.proc_macro_deps).to_list()

    target.test(
        name = name,
        srcs = [":{}".format(generate_target)],
        use_libtest_harness = False,
        testonly = testonly,
        proc_macro_deps = proc_macro_deps,
        **kwargs
    )

def autotest_performance(name, template, cases, specification, language, generator = "//system/generation:generator", **kwargs):
    """
    Create a performance test target from a template, test cases, and specification.

    Language-agnostic base. Use language-specific wrappers (rust_autotest_performance) for convenience.

    Args:
        name: Test target name
        template: Template target (created with autotest_template)
        cases: Test case data file (JSON)
        specification: Performance specification file (JSON)
        language: Language name or Language struct
        generator: Generator binary (default: //system/generation:generator)
        **kwargs: Passed to test target (deps, data, visibility, etc.)
    """

    testonly = kwargs.pop("testonly", True)

    target = _resolve_language(language, name)

    if not template:
        fail("autotest_performance requires 'template' for '{}'".format(name))
    if not cases:
        fail("autotest_performance requires 'cases' for '{}'".format(name))
    if not specification:
        fail("autotest_performance requires 'specification' for '{}'".format(name))

    generate_target = "{}.generation".format(name)

    generate_attrs = {attr: kwargs[attr] for attr in ["visibility", "tags", "deprecation"] if attr in kwargs}
    generate_attrs["specification"] = specification

    _autotest_performance_generate(
        name = generate_target,
        template = template,
        cases = cases,
        generator = generator,
        deps = [template],
        testonly = testonly,
        **generate_attrs
    )

    if "timeout" not in kwargs:
        kwargs["timeout"] = "short"

    proc_macro_deps = kwargs.pop("proc_macro_deps", [])
    proc_macro_deps = depset(proc_macro_deps + target.proc_macro_deps).to_list()

    target.test(
        name = name,
        srcs = [":{}".format(generate_target)],
        use_libtest_harness = False,
        testonly = testonly,
        proc_macro_deps = proc_macro_deps,
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

    kwargs["deps"] = depset(deps + dependencies.template).to_list()

    autotest_template(
        name = name,
        src = src,
        language = "rust",
        library = rust_library,
        **kwargs
    )

def rust_autotest_function(name, template, cases, generator = "//system/generation:generator", deps = [], **kwargs):
    """
    Generate and run Rust function tests from a template and test cases.

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

    merged = depset(deps + dependencies.rust.function).to_list()
    kwargs["deps"] = merged

    autotest_function(
        name = name,
        template = template,
        cases = cases,
        language = "rust",
        generator = generator,
        **kwargs
    )

def rust_autotest_performance(name, template, cases, specification, deps = [], **kwargs):
    """
    Generate and run Rust performance tests from a template, test cases, and a performance specification.

    Auto-includes standard deps plus performance-specific deps (regression, nalgebra).
    Selects cases by tag, times them, auto-infers best-fit model, and asserts bounds.

    Args:
        name: Test target name
        template: Template target (created with rust_autotest_template)
        cases: Test case data file (JSON)
        specification: Performance specification file (JSON)
        deps: Custom dependencies (only if template uses non-default deps)
        **kwargs: Passed to rust_test (data, visibility, etc.)
    """

    merged = depset(deps + dependencies.rust.performance).to_list()
    kwargs["deps"] = merged

    rustc_flags = kwargs.get("rustc_flags", [])
    if "-A" not in " ".join(rustc_flags):
        rustc_flags = rustc_flags + ["-A", "dead_code"]
    kwargs["rustc_flags"] = rustc_flags

    autotest_performance(
        name = name,
        template = template,
        cases = cases,
        specification = specification,
        language = "rust",
        **kwargs
    )

def _template_source_impl(ctx):
    template_info = ctx.attr.template[TemplateInfo]
    source = template_info.source
    return [
        DefaultInfo(files = depset([source])),
        SinkInfo(path = source.short_path),
    ]

_template_source = rule(
    implementation = _template_source_impl,
    attrs = {
        "template": attr.label(mandatory = True, providers = [TemplateInfo]),
    },
    doc = "Expose a template source file with SinkInfo for documentation distribution",
)

def _file_source_impl(ctx):
    source = ctx.attr.source[DefaultInfo].files.to_list()[0]
    return [
        DefaultInfo(files = depset([source])),
        SinkInfo(path = source.short_path),
    ]

_file_source = rule(
    implementation = _file_source_impl,
    attrs = {
        "source": attr.label(mandatory = True, allow_files = True),
    },
    doc = "Expose a file with SinkInfo for documentation distribution",
)

def _cases_label(template):
    """Derive the cases target label from the template label's package."""
    if ":" in template:
        package, _ = template.rsplit(":", 1)
    else:
        package = ""
    return (package + ":cases") if package else ":cases"

def autotest_document(name, test, template, parameters = {}, deps = [], **kwargs):
    """
    Generate a Rust library providing card::Group visualization for a test suite.

    Executes the test target to produce execution JSON, then embeds that data
    into generated Rust code via AST emission. The resulting library exposes
    a single `cards()` function returning visualization card groups.

    Resource files flow automatically from the test target's runfiles.

    Produces:
      - {name}.execute           - execution JSON from running the test
      - {name}.generation        - generated Rust source
      - {name}                   - compiled rust_library exposing cards()
      - {name}.template.source   - template source file with SinkInfo
      - {name}.cases.source      - cases file with SinkInfo

    Args:
        name: Target name (e.g., "proportion.document")
        test: Autotest target (e.g., ":arena")
        template: Template target (source .rs file)
        parameters: CLI parameters passed to the test binary (e.g., {"bound": "on"})
        deps: Additional compile deps
        **kwargs: Standard Bazel attrs (visibility, tags)
    """
    execute_target = name + ".execute"

    execute(
        name = execute_target,
        binary = test,
        allow_failure = True,
        parameters = parameters,
        output = name + ".execution.json",
    )

    generate_target = name + ".generation"

    generate(
        name = generate_target,
        generator = "//system/generation/rust/document:card",
        data = [":" + execute_target, template],
        output = name + ".rs",
    )

    passthrough = {k: kwargs[k] for k in ["visibility", "tags"] if k in kwargs}

    rust_library(
        name = name,
        srcs = [":" + generate_target],
        crate_name = name.replace(".", "_"),
        deps = [
            "//system/generation/rust/document:visualize",
            "//component/web/dashboard:card",
            "@crates//:serde_json",
        ] + deps,
        **passthrough
    )

    _template_source(
        name = name + ".template.source",
        template = template,
        **passthrough
    )

    _file_source(
        name = name + ".cases.source",
        source = _cases_label(template),
        **passthrough
    )
