use effect::{Effected, outline};
use hypergraph::Label;
use layout::Simulation;
use primitive::{Arrow, Primitive, Sphere};
use space::{Connector, Space};
use vector::Vector;

pub struct Edge {
    pub arrow: Arrow,
    pub label: Label,
    pub highlighted: bool,
}

impl Edge {
    fn new(arrow: Arrow, label: Label, highlighted: bool) -> Self {
        Self {
            arrow,
            label,
            highlighted,
        }
    }

    #[must_use]
    pub fn effect(self) -> Effected<Arrow> {
        self.arrow.effect(outline::Outline::new())
    }
}

pub fn edges<'a>(space: &'a Space, layout: &'a Simulation) -> impl Iterator<Item = Edge> + 'a {
    space.edges().flat_map(move |connector| {
        let label = connector.label();
        let highlighted = space.highlighted(label);
        arrows(space, layout, connector, label, highlighted)
    })
}

fn arrows<'a>(
    space: &'a Space,
    layout: &'a Simulation,
    connector: &'a Connector,
    label: Label,
    highlighted: bool,
) -> impl Iterator<Item = Edge> + 'a {
    let sources = connector
        .sources()
        .iter()
        .filter_map(move |&label| {
            let position = layout.position(label)?;
            let radius = space.node(label).map_or(0.0, space::Node::radius);
            Some((position, radius))
        })
        .collect::<Vec<_>>();

    let sinks = connector
        .sinks()
        .iter()
        .filter_map(move |&label| {
            let position = layout.position(label)?;
            let radius = space.node(label).map_or(0.0, space::Node::radius);
            Some((position, radius))
        })
        .collect::<Vec<_>>();

    let width = connector.width() * 0.5;
    let color = palette::EDGE;

    sources.into_iter().flat_map(move |(source, sr)| {
        sinks.clone().into_iter().filter_map(move |(sink, tr)| {
            let delta = sink - source;
            let length = delta.magnitude();

            if length < 0.001 {
                return None;
            }

            let direction = delta / length;
            let from = source + direction * sr;
            let to = sink - direction * tr;

            Some(Edge::new(
                Arrow::new(from, to, width, color),
                label,
                highlighted,
            ))
        })
    })
}

pub struct Vertex {
    pub sphere: Sphere,
    pub label: Label,
    pub text: String,
    pub offset: Vector,
    pub highlighted: bool,
}

impl Vertex {
    #[must_use]
    pub fn effect(self) -> Effected<Sphere> {
        self.sphere.effect(outline::Outline::new())
    }
}

pub fn nodes<'a>(space: &'a Space, layout: &'a Simulation) -> impl Iterator<Item = Vertex> + 'a {
    space.nodes().filter_map(move |node| {
        let highlighted = space.highlighted(node.label());
        let position = layout.position(node.label())?;
        let sphere = Sphere::new(position, node.radius(), palette::NODE);
        let margin = node.radius() + scale::margin();
        let offset = Vector::new(position.x + margin, position.y, position.z);

        Some(Vertex {
            sphere,
            label: node.label(),
            text: node.text().to_string(),
            offset,
            highlighted,
        })
    })
}
