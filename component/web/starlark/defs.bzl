"""
Public API for document generation.

Exports: document, publish, verify, distribute, validate
"""

load("//component/generation/starlark:web.bzl", _document = "document")
load(":distribute.bzl", _distribute = "distribute")
load(":publish.bzl", _publish = "publish")
load(":aspect.bzl", _validate = "validate")
load(":verify.bzl", _verify = "verify")

document = _document
publish = _publish
verify = _verify
distribute = _distribute
validate = _validate
