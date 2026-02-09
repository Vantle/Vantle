"""
Public API for document generation.

Exports: generate, collection, drift, validate
"""

load(":aspect.bzl", _validate = "validate")
load(":collection.bzl", _collection = "collection")
load(":drift.bzl", _drift = "drift")
load(":macro.bzl", _generate = "generate")

generate = _generate
collection = _collection
drift = _drift
validate = _validate
