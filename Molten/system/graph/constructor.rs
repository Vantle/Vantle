use component::graph::attribute::{Attribute, Category, Value};
use component::graph::traits::attribute::{Categorized, Contextualized};

use component::graph::arena::Valued;
use component::graph::traits::node::Aliased;

use component::graph::node::Node;

use component::graph::matrix::{Constructor, Related as Relations};

use itertools::chain;
use log::debug;

pub fn relate<T: Value>(
    attributes: &[Attribute<T>],
    index: &Valued<Attribute<T>>,
) -> Relations<Node<Node<usize>>> {
    debug!("==> Relating {:#?} contexts", attributes);
    let mut relations = Constructor::<Node<Node<usize>>>::default();
    let partitions = partition(attributes, index);
    related(partitions, index, None, &mut relations);

    attributes.iter().for_each(|attribute| {
        attribute.breadth().for_each(|element| {
            let scope = index.alias(element).unwrap();
            let partitions = partition(element.context(), index);
            related(partitions, index, Some(scope), &mut relations);
        });
    });

    debug!("<== Relating {:#?} contexts", attributes);
    relations.identified()
}

fn related<'view, T: Value>(
    partitions: Vec<Vec<usize>>,
    index: &'view Valued<Attribute<T>>,
    scope: Option<usize>,
    relations: &'view mut Constructor<Node<Node<usize>>>,
) -> &'view mut Constructor<Node<Node<usize>>> {
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
                partition
                    .iter()
                    .filter(|&destination| *destination != source)
                    .for_each(|&destination| {
                        let related = index.value(destination).unwrap();
                        match related.category() {
                            Category::Context | Category::Group => {
                                let joins = crate::partition(value.context(), index);
                                let sources = joins
                                    .iter()
                                    .map(|join| {
                                        let combined =
                                            chain(scope.iter().copied(), join.iter().copied())
                                                .collect::<Vec<usize>>();
                                        Node::from(combined.as_slice())
                                    })
                                    .collect::<Vec<Node<usize>>>();

                                relations.relate(
                                    &Node::from(sources.as_slice()),
                                    &Node::from(&[Node::from(&[destination][..])][..]),
                                );
                            }
                            category => {
                                debug!(
                                    "No relation semantic from {:#?} to {:#?}",
                                    Category::<T>::Context,
                                    category
                                );
                            }
                        }
                    });
            });
    });
    relations
}

fn partition<T: Value>(
    attributes: &[Attribute<T>],
    index: &Valued<Attribute<T>>,
) -> Vec<Vec<usize>> {
    debug!("==> Partitioning {:#?}", attributes);
    let partitions = attributes
        .iter()
        .fold(Vec::default(), |mut accumulator, value| {
            match value.category().to_owned() {
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
