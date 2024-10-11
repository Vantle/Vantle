use std::borrow::Borrow;

use attribute::{Attribute, Category};
use attributes::{Categorized, Contextualized};

use arena::Valued;
use arenas::Identified as References;

use state::State;

use matrix::{Constructor, Related as Relations};

use log::debug;

pub fn relate<Value: attribute::Value>(
    attribute: &Attribute<Value>,
    index: &Valued<Attribute<Value>>,
) -> Relations<State<usize>> {
    debug!("==> Relating {:#?} contexts", attribute);
    let mut relations = Constructor::<State<usize>>::default();
    attribute.breadth().for_each(|element| {
        let scope = index.label(element).unwrap();
        let partitions = partition(element, index);
        partitions.iter().for_each(|partition| {
            partition
                .iter()
                .enumerate()
                .for_each(|(enumeration, &source)| {
                    let value = index.value(source).unwrap();
                    match *value.category() {
                        Category::Context => {
                            partition
                                .iter()
                                .skip(enumeration + 1)
                                .for_each(|&destination| {
                                    let related = index.value(destination).unwrap();
                                    match *related.category() {
                                        Category::Context | Category::Group => {
                                            let sources = State::new(
                                                value
                                                    .context()
                                                    .iter()
                                                    .map(|context| index.label(context).unwrap())
                                                    .chain(std::iter::once(scope))
                                                    .collect::<Vec<usize>>()
                                                    .borrow(),
                                            );
                                            let destinations = State::new(
                                                related
                                                    .context()
                                                    .iter()
                                                    .map(|context| index.label(context).unwrap())
                                                    .collect::<Vec<usize>>()
                                                    .borrow(),
                                            );
                                            relations
                                                .relate(sources.borrow(), destinations.borrow());
                                        }
                                        category => {
                                            debug!(
                                                "No relation semantic from {:#?} to {:#?}",
                                                Category::Context,
                                                category
                                            );
                                        }
                                    }
                                });
                        }
                        category => {
                            debug!("No relation semantic for {:#?} from {:#?}", category, value);
                        }
                    };
                });
        });
    });
    debug!("<== Relating {:#?} contexts", attribute);
    relations.identified()
}

fn partition<Value: attribute::Value>(
    attribute: &Attribute<Value>,
    index: &Valued<Attribute<Value>>,
) -> Vec<Vec<usize>> {
    debug!("==> Partitioning {:#?}", attribute);
    let partitions = attribute
        .context()
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
                        .push(index.label(value).unwrap_or_else(|_| {
                            panic!(
                                "Attribute {:?} was unlabeled. Index {:#?} provided is invalid",
                                value, index
                            )
                        }))
                }
            };
            accumulator
        });
    debug!("<== Partitioned {:#?} for {:#?}", partitions, attribute);
    partitions
}
