"""
Types and providers for document generation.

Shared providers used across rules and macros.
"""

DocumentInfo = provider(
    doc = "Metadata about generated document files",
    fields = {
        "output": "The generated document file",
        "destination": "Workspace-relative output path",
    },
)
