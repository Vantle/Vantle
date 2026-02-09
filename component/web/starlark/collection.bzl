"""
Collection rule for aggregating document targets.

Collects all DocumentInfo providers and writes a manifest for the publish binary.
"""

load(":impls.bzl", "PUBLISH_ATTRS", "publish_impl")

def _collection_impl(ctx):
    return publish_impl(ctx)

collection = rule(
    implementation = _collection_impl,
    executable = True,
    attrs = PUBLISH_ATTRS,
    doc = "Aggregates document targets for batch generation",
)
