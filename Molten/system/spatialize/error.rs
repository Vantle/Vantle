use hypergraph::Label;

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Space(#[from] space::Error),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Render(#[from] render::Error),

    #[error("missing required configuration: {field}")]
    #[diagnostic(
        code(spatialize::configuration),
        help("call .{field}() on Assembler before .assemble()")
    )]
    Configuration { field: String },

    #[error("failed to read shader source")]
    #[diagnostic(
        code(spatialize::shader),
        help("ensure shader file exists at configured path")
    )]
    Shader {
        #[source]
        source: std::io::Error,
    },

    #[error("missing position for label {label:?}")]
    #[diagnostic(
        code(spatialize::layout::missing),
        help("ensure all nodes in the space have corresponding positions in the simulation")
    )]
    Missing { label: Label },

    #[error("failed to create event loop")]
    #[diagnostic(
        code(spatialize::event),
        help("ensure a display server is running and accessible")
    )]
    Event {
        #[source]
        source: winit::error::EventLoopError,
    },

    #[error("failed to create window")]
    #[diagnostic(
        code(spatialize::window),
        help("ensure the display server supports window creation")
    )]
    Window {
        #[source]
        source: winit::error::OsError,
    },

    #[error("failed to create graphics surface")]
    #[diagnostic(
        code(spatialize::surface),
        help("ensure graphics drivers are installed and window is valid")
    )]
    Surface {
        #[source]
        source: wgpu::CreateSurfaceError,
    },

    #[error("no suitable graphics adapter found")]
    #[diagnostic(
        code(spatialize::adapter),
        help("ensure a compatible GPU is available with updated drivers")
    )]
    Adapter {
        #[source]
        source: wgpu::RequestAdapterError,
    },

    #[error("failed to assemble renderer")]
    #[diagnostic(
        code(spatialize::renderer),
        help("check that shader files exist and GPU supports required features")
    )]
    Renderer {
        #[source]
        source: render::Error,
    },

    #[error("event loop terminated unexpectedly")]
    #[diagnostic(
        code(spatialize::run),
        help("the application encountered an error during execution")
    )]
    Run {
        #[source]
        source: winit::error::EventLoopError,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
