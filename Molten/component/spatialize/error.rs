use hypergraph::Label;

#[derive(thiserror::Error, miette::Diagnostic, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Duplicate(#[from] Duplicate),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Missing(#[from] Missing),
}

#[derive(thiserror::Error, miette::Diagnostic, Debug, Clone, PartialEq, Eq)]
pub enum Duplicate {
    #[error("duplicate node: {label:?}")]
    #[diagnostic(
        code(space::duplicate::node),
        help("each node must have a unique label")
    )]
    Node { label: Label },

    #[error("duplicate connector: {label:?}")]
    #[diagnostic(
        code(space::duplicate::connector),
        help("each connector must have a unique label")
    )]
    Connector { label: Label },
}

#[derive(thiserror::Error, miette::Diagnostic, Debug, Clone, PartialEq, Eq)]
pub enum Missing {
    #[error("missing node: {label:?}")]
    #[diagnostic(code(space::missing::node), help("node does not exist in space"))]
    Node { label: Label },

    #[error("missing connector: {label:?}")]
    #[diagnostic(
        code(space::missing::connector),
        help("connector does not exist in space")
    )]
    Connector { label: Label },
}
