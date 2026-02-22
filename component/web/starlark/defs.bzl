"""
Public API for document generation.

Exports: copy, distribute, document, folder, validate, validation
"""

load("//component/generation/starlark:web.bzl", _document = "document")
load(":aspect.bzl", _validate = "validate", _validation = "validation")
load(":distribute.bzl", _copy = "copy", _distribute = "distribute", _folder = "folder")

copy = _copy
distribute = _distribute
document = _document
folder = _folder
validate = _validate
validation = _validation
