"""
Code generation macros for generating test files from templates and data at build time.

This module provides:
- autotest: Generic macro for creating test targets from templates and data
- rust_autotest: Rust-specific convenience wrapper

Example usage:
    rust_autotest(
        name = "my_test",
        cases = ":cases.json",
        template = ":test.template.rs",
        deps = ["//some:dependency"],
    )
    
    # Creates:
    # - my_test_validation (rust_library): validates template compiles
    # - my_test_generation (generate rule): cases.json + template → cases.generated.rs  
    # - my_test (rust_test): compiles and runs test_data.generated.rs
"""

load(":rule.bzl", "generate")
load(":types.bzl", "LANGUAGES")

def autotest(name, template, cases, language, generator = "//system/generation:generator", **kwargs):
    """
    Generic autotest macro that generates a test file and creates a test target.

    Args:
        name: Name of the target
        template: Template file to use for generation
        cases: Single test case data file (e.g., JSON) used for generation (currently assumed JSON)
        language: Language name (must be a key in LANGUAGES) or a Language struct
        generator: Generator tool to use (defaults to standard generator)
        **kwargs: Additional attributes passed to pipeline targets
    """

    # Handle both string language names and Language structs
    if type(language) == "string":
        target = LANGUAGES.get(language)
        if not target:
            fail("Unsupported language '{}' for autotest target '{}'. Supported languages are: {}".format(
                language,
                name,
                ", ".join(sorted(LANGUAGES.keys())),
            ))
        language_name = language
    else:
        # Assume it's a Language struct
        target = language

        # Try to find the language name from LANGUAGES dict
        language_name = None
        for lang_name, lang in LANGUAGES.items():
            if lang == language:
                language_name = lang_name
                break
        if not language_name:
            language_name = "custom"  # Fallback for custom Language structs

    # Validate required parameters
    if not template:
        fail("autotest requires 'template' attribute for target '{}'".format(name))
    if not cases:
        fail("autotest requires 'cases' attribute for target '{}'".format(name))

    # Create template validation library
    validation_name = "{}_validation".format(name)

    # Extract library-specific kwargs
    library_kwargs = {k: v for k, v in kwargs.items() if k not in ["visibility", "testonly", "srcs", "size", "timeout", "flaky", "shard_count", "local"]}

    # Add language-specific flags if applicable
    if hasattr(target, "flags") and target.flags:
        # Handle different flag attributes based on language
        if language_name == "rust":
            library_kwargs["rustc_flags"] = kwargs.get("rustc_flags", []) + target.flags

    target.library(
        name = validation_name,
        srcs = [template],
        testonly = True,
        **library_kwargs
    )

    # Create generate rule for the data file
    generate_target_name = "{}_generation".format(name)

    generate(
        name = generate_target_name,
        template = template,
        cases = cases,
        language = language_name,
        generator = generator,
        deps = [":{}".format(validation_name)],  # Depend on template validation
        testonly = True,
        **{attr: kwargs[attr] for attr in ["visibility", "tags", "deprecation"] if attr in kwargs}
    )

    # Ensure a reasonable default timeout for tests unless explicitly overridden.
    if "timeout" not in kwargs:
        kwargs["timeout"] = "short"

    # Create test that uses the generated file
    target.test(
        name = name,
        srcs = [":{}".format(generate_target_name)],
        **kwargs
    )

def rust_autotest(name, template, cases, generator = "//system/generation:generator", **kwargs):
    """
    Generate a Rust test file and create a test target from a single data file.

    This is a convenience wrapper around autotest for Rust.

    Args:
        name: Name of the target
        template: Template .rs file to use for generation
        cases: Single test case data file (e.g., JSON) used for generation (currently assumed JSON)
        generator: Generator tool to use (defaults to standard generator)
        **kwargs: Additional attributes passed to pipeline targets
    """

    # Merge user-supplied deps with serde_json, avoiding duplicates
    deps = kwargs.get("deps", [])
    extra = ["@crates//:serde_json", "@crates//:serde", "//:module"]
    merged = depset(deps + extra).to_list()
    kwargs["deps"] = merged

    autotest(
        name = name,
        template = template,
        cases = cases,
        language = "rust",
        generator = generator,
        **kwargs
    )
