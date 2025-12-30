use std::collections::{BTreeMap, HashMap};

use effect::Effect;
use form::{Billboard, Cylinder, Geometry, Sphere};
use hypergraph::Label;
use space::Space;
use vector::Vector;

pub trait Synchronization {
    fn synchronize(
        &mut self,
        space: &Space,
        positions: &BTreeMap<Label, Vector>,
        density: f32,
        widths: &HashMap<Label, f32>,
    );
}

impl Synchronization for Effect<Geometry> {
    fn synchronize(
        &mut self,
        space: &Space,
        positions: &BTreeMap<Label, Vector>,
        density: f32,
        widths: &HashMap<Label, f32>,
    ) {
        let mut forms = Vec::new();
        let font = scale::font();
        let tolerance = scale::tolerance();

        for node in space.nodes() {
            let center = positions.get(&node.label()).copied().unwrap_or_default();

            forms.push(Geometry::Sphere(Sphere::new(
                node.label(),
                center,
                node.radius(),
            )));

            let offset = node.radius() + scale::margin();
            let text = Vector::new(center.x + offset, center.y, center.z);
            let pixels = widths.get(&node.label()).copied().unwrap_or(font);
            let width = pixels * density * tolerance;
            let height = font * density * tolerance;
            forms.push(Geometry::Billboard(Billboard::new(
                node.label(),
                text,
                width,
                height,
            )));
        }

        for connector in space.edges() {
            for &source in connector.sources() {
                for &sink in connector.sinks() {
                    let ps = positions.get(&source).copied();
                    let pt = positions.get(&sink).copied();

                    if let (Some(ps), Some(pt)) = (ps, pt) {
                        forms.push(Geometry::Cylinder(Cylinder::new(
                            connector.label(),
                            ps,
                            pt,
                            connector.width() * 0.5,
                        )));
                    }
                }
            }
        }

        self.react(forms);
    }
}
