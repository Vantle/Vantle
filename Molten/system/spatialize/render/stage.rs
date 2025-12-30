use error::Result;
use graph::Graph;

pub trait Stage {
    fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, graph: &Graph) -> Result<()>;

    fn draw(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        surface: &wgpu::TextureView,
        graph: &Graph,
    ) -> Result<()>;
}
