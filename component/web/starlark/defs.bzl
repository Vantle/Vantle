"""
Public API for document generation.

Exports: generate, collection, drift, serve, validate
"""

load(":aspect.bzl", _validate = "validate")
load(":collection.bzl", _collection = "collection")
load(":drift.bzl", _drift = "drift")
load(":macro.bzl", _generate = "generate")
load(":serve.bzl", _serve = "serve")

generate = _generate
collection = _collection
drift = _drift
serve = _serve
validate = _validate
