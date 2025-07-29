"""
Public API for the code generation library.

This file re-exports all public symbols for convenient access.
"""

load(":macro.bzl", _autotest = "autotest", _rust_autotest = "rust_autotest")

# Re-export public macros
autotest = _autotest
rust_autotest = _rust_autotest
