"""
Public API for code generation.

Exports: autotest, autotest_template, rust_autotest, rust_autotest_template, generate, document
"""

load(":action.bzl", _generate = "generate")
load(":autotest.bzl", _autotest = "autotest", _autotest_template = "autotest_template", _rust_autotest = "rust_autotest", _rust_autotest_template = "rust_autotest_template")
load(":web.bzl", _document = "document")

autotest = _autotest
autotest_template = _autotest_template
rust_autotest = _rust_autotest
rust_autotest_template = _rust_autotest_template
generate = _generate
document = _document
