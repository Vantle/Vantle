"""
Public API for code generation.

Exports: asset, autotest, autotest_template, bindgen, rust_autotest, rust_autotest_template, generate, document
"""

load(":action.bzl", _asset = "asset", _bindgen = "bindgen", _generate = "generate")
load(":autotest.bzl", _autotest = "autotest", _autotest_template = "autotest_template", _rust_autotest = "rust_autotest", _rust_autotest_template = "rust_autotest_template")
load(":web.bzl", _document = "document")

asset = _asset
bindgen = _bindgen
autotest = _autotest
autotest_template = _autotest_template
rust_autotest = _rust_autotest
rust_autotest_template = _rust_autotest_template
generate = _generate
document = _document
