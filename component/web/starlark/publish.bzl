"""
Publish rule for aggregating document targets.

Collects all GenerationInfo providers and writes a manifest for the publish binary.
"""

load(":impls.bzl", "PUBLISH_ATTRS", "publish_impl")

def _publish_impl(ctx):
    return publish_impl(ctx)

publish = rule(
    implementation = _publish_impl,
    executable = True,
    attrs = PUBLISH_ATTRS,
    doc = "Aggregates document targets for batch publishing",
)
