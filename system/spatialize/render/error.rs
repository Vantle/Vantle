#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum Error {
    #[error("missing required configuration: {field}")]
    #[diagnostic(
        code(render::configuration),
        help("call .{field}() on Assembler before .assemble()")
    )]
    Configuration { field: String },

    #[error("failed to create wgpu surface")]
    #[diagnostic(
        code(render::surface),
        help("ensure graphics drivers are installed and window is valid")
    )]
    Surface {
        #[source]
        source: wgpu::CreateSurfaceError,
    },

    #[error("no suitable graphics adapter found")]
    #[diagnostic(code(render::adapter), help("ensure a compatible GPU is available"))]
    Adapter,

    #[error("failed to request graphics device")]
    #[diagnostic(code(render::device), help("try updating graphics drivers"))]
    Device {
        #[source]
        source: wgpu::RequestDeviceError,
    },

    #[error("failed to read shader source")]
    #[diagnostic(
        code(render::shader),
        help("ensure shader file exists at configured path")
    )]
    Shader {
        #[source]
        source: std::io::Error,
    },

    #[error("failed to acquire swapchain texture")]
    #[diagnostic(code(render::swapchain), help("the surface may need reconfiguration"))]
    Swapchain {
        #[source]
        source: wgpu::SurfaceError,
    },

    #[error("failed to read font")]
    #[diagnostic(code(render::font), help("ensure font file exists at configured path"))]
    Font {
        #[source]
        source: std::io::Error,
    },

    #[error("failed to render text")]
    #[diagnostic(code(render::text), help("ensure text was prepared before rendering"))]
    Text,

    #[error("resource not found in graph: {tag}")]
    #[diagnostic(
        code(render::resource),
        help("ensure the resource is registered before accessing it")
    )]
    Resource { tag: String },

    #[error("resource type mismatch for {tag}: expected {expected}, found {found}")]
    #[diagnostic(
        code(render::mismatch),
        help("use texture() for texture resources and buffer() for buffer resources")
    )]
    Mismatch {
        tag: String,
        expected: String,
        found: String,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
