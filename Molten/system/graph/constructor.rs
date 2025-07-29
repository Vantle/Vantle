use log::debug;

use component::graph::arena::Valued;
use component::graph::attribute::{Attribute, Category, Value};
use component::graph::matrix::{Constructor, Related as Relations};
use component::graph::node::Node;
use component::graph::traits::attribute::{Categorized, Contextualized};
use component::graph::traits::node::{Aliased as _, Valued as _};

pub fn relate<T: Value>(
    attribute: &Attribute<T>,
    index: &Valued<Attribute<T>>,
) -> Relations<Node<Node<usize>>> {
    debug!("==> Relating {:#?} contexts", attribute);

    let mut relations = Constructor::<Node<Node<usize>>>::default();

    related(
        None, // Latenization is explicit
        attribute.context(),
        index,
        &mut relations,
    );

    debug!("<== Related {:#?} contexts", attribute);
    relations.identified()
}

fn related<T: Value>(
    latent: Option<Node<usize>>,
    level: &[Attribute<T>],
    index: &Valued<Attribute<T>>,
    relations: &mut Constructor<Node<Node<usize>>>,
) {
    let partitions = partition(level, index);

    partitions.iter().for_each(|partition| {
        partition
            .iter()
            .filter(|&&source| {
                index
                    .value(source)
                    .is_ok_and(|value| value.category() == &Category::Context)
            })
            .for_each(|&source| {
                let value = index.value(source).unwrap();
                let sources = Node::<Node<usize>>::from(
                    crate::partition(value.context(), index)
                        .iter()
                        .map(|join| Node::from(join.as_slice()))
                        .chain(latent.iter().cloned())
                        .collect::<Vec<Node<usize>>>()
                        .as_slice(),
                );

                partition
                    .iter()
                    .filter(|&&destination| destination != source)
                    .for_each(|&destination| {
                        let alias = index.value(destination).unwrap();
                        let sinks = match alias.category() {
                            Category::Context => Node::<Node<usize>>::from(
                                crate::partition(alias.context(), index)
                                    .iter()
                                    .map(|join| Node::from(join.as_slice()))
                                    .chain(latent.iter().cloned())
                                    .collect::<Vec<Node<usize>>>()
                                    .as_slice(),
                            ),
                            _ => Node::<Node<usize>>::unit(Node::unit(destination)),
                        };
                        relations.relate(&sources, &sinks);
                    });
            });

        // Latentization
        partition
            .iter()
            .filter(|&&destination| {
                index
                    .value(destination)
                    .is_ok_and(|value| value.category() == &Category::Group)
            })
            .for_each(|&destination| {
                let value = index.value(destination).unwrap();
                related(
                    Some(Node::unit(index.alias(value).unwrap())),
                    value.context(),
                    index,
                    relations,
                );
            });
    });
    // mutated in-place
}

fn partition<T: Value>(
    attributes: &[Attribute<T>],
    index: &Valued<Attribute<T>>,
) -> Vec<Vec<usize>> {
    debug!("==> Partitioning {:#?}", attributes);
    let partitions = attributes
        .iter()
        .fold(Vec::default(), |mut accumulator, value| {
            match *value.category() {
                Category::Partition => accumulator.push(Vec::default()),
                _ => {
                    if accumulator.is_empty() {
                        accumulator.push(Vec::default());
                    }
                    accumulator
                        .last_mut()
                        .unwrap()
                        .push(index.alias(value).unwrap())
                }
            };
            accumulator
        });
    debug!("<== Partitioned {:#?} for {:#?}", partitions, attributes);
    partitions
}
