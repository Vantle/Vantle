use error::Result;
use hypergraph::Hypergraph;
use layout::Simulation;
use space::Space;

pub struct Scene {
    graph: Hypergraph<String>,
    space: Space,
    layout: Simulation,
}

impl Scene {
    #[must_use]
    pub fn new(graph: Hypergraph<String>, space: Space, layout: Simulation) -> Self {
        Self {
            graph,
            space,
            layout,
        }
    }

    #[must_use]
    pub fn graph(&self) -> &Hypergraph<String> {
        &self.graph
    }

    #[must_use]
    pub fn space(&self) -> &Space {
        &self.space
    }

    #[must_use]
    pub fn layout(&self) -> &Simulation {
        &self.layout
    }

    pub fn highlight(&mut self) -> (&Hypergraph<String>, &mut Space) {
        (&self.graph, &mut self.space)
    }

    pub fn step(&mut self, bounds: f32) -> Result<()> {
        self.layout.step(&self.space, bounds)
    }

    #[must_use]
    pub fn positions(&self) -> &std::collections::BTreeMap<hypergraph::Label, vector::Vector> {
        self.layout.positions()
    }

    #[must_use]
    pub fn converged(&self) -> bool {
        self.layout.converged()
    }
}

impl AsMut<Space> for Scene {
    fn as_mut(&mut self) -> &mut Space {
        &mut self.space
    }
}

impl AsMut<Simulation> for Scene {
    fn as_mut(&mut self) -> &mut Simulation {
        &mut self.layout
    }
}
