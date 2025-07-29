"""
Shared types, providers, and language configurations for the code generation library.

This file contains data structures that need to be shared between rule definitions
and their implementations.
"""

load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

#############################################################################
# PROVIDERS
#############################################################################

GeneratedInfo = provider(
    doc = "Information about generated files",
    fields = {
        "generated": "Generated file",
        "template": "Template",
        "cases": "Cases",
        "language": "Generated language",
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
    Create a language configuration struct.

    Args:
        extension: File extension for generated files (e.g., 'rs' for Rust)
        test: Test rule function (e.g., rust_test)
        library: Library rule function (e.g., rust_library)
        deps: Standard dependencies for this language (default: [])
        flags: Standard compiler flags for this language (default: [])

    Returns:
        A struct containing the language configuration
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
        flags = ["-A", "dead_code"],
    ),
}
