"""
Public API for code generation.

Exports: autotest_document, autotest_function, autotest_performance, autotest_template, rust_autotest_function, rust_autotest_performance, rust_autotest_template, generate, execute, document, extract, query
"""

load(":action.bzl", _execute = "execute", _generate = "generate")
load(":autotest.bzl", _autotest_document = "autotest_document", _autotest_function = "autotest_function", _autotest_performance = "autotest_performance", _autotest_template = "autotest_template", _rust_autotest_function = "rust_autotest_function", _rust_autotest_performance = "rust_autotest_performance", _rust_autotest_template = "rust_autotest_template")
load(":extract.bzl", _extract = "extract")
load(":query.bzl", _query = "query")
load(":web.bzl", _document = "document")

autotest_document = _autotest_document
autotest_function = _autotest_function
autotest_performance = _autotest_performance
autotest_template = _autotest_template
rust_autotest_function = _rust_autotest_function
rust_autotest_performance = _rust_autotest_performance
rust_autotest_template = _rust_autotest_template
generate = _generate
execute = _execute
document = _document
extract = _extract
query = _query
