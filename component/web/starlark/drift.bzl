"""
Drift verification rule for document targets.

Verifies that generated files match what is checked into the repository.
"""

load(":impls.bzl", "PUBLISH_ATTRS", "publish_impl")

def _drift_impl(ctx):
    return publish_impl(ctx, verify = True)

drift = rule(
    implementation = _drift_impl,
    executable = True,
    attrs = PUBLISH_ATTRS,
    doc = "Verifies generated documents match the repository",
)
