"""
Types and language configurations for code generation.

Shared providers and language configs used across rules and macros.
"""

load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

#############################################################################
# PROVIDERS
#############################################################################

GeneratedInfo = provider(
    doc = "Metadata about generated test files",
    fields = {
        "generated": "Generated test file",
        "template": "Template source file",
        "cases": "Test cases data file",
        "language": "Language string",
    },
)

TemplateInfo = provider(
    doc = "Template metadata for code generation",
    fields = {
        "source": "Template source file",
        "language": "Language string (e.g., 'rust')",
        "test": "Test rule identifier",
        "deps": "Template dependencies",
    },
)

#############################################################################
# LANGUAGE CONFIGURATION
#############################################################################

# Language: struct type
# Fields:
#   extension: str - File extension for generated files (e.g., 'rs' for Rust)
#   test: function - Test rule function (e.g., rust_test)
#   library: function - Library rule function (e.g., rust_library)
#   deps: list - Standard dependencies for this language
#   flags: list - Standard compiler flags for this language

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

# Language configurations using the Language struct factory
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
