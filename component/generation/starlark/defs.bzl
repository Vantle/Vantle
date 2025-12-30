"""
Public API for code generation.

Exports: autotest, autotest_template, rust_autotest, rust_autotest_template
"""

load(":macro.bzl", _autotest = "autotest", _autotest_template = "autotest_template", _rust_autotest = "rust_autotest", _rust_autotest_template = "rust_autotest_template")

# Re-export public macros
autotest = _autotest
autotest_template = _autotest_template
rust_autotest = _rust_autotest
rust_autotest_template = _rust_autotest_template
