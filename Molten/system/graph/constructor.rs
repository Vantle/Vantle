use attribute::{Attribute, Category};
use attributes::{Categorized, Contextualized};

use arena::Valued;
use arenas::Identified as References;

use state::State;

use matrix::{Constructor, Related as Relations};

use log::debug;

pub fn relate<Value: attribute::Value>(
    attributes: &[Attribute<Value>],
    index: &Valued<Attribute<Value>>,
) -> Relations<State<usize>> {
    debug!("==> Relating {:#?} contexts", attributes);
    let mut relations = Constructor::<State<usize>>::default();
    let partitions = partition(attributes, index);
    related(partitions, index, None, &mut relations);

    attributes.iter().for_each(|attribute| {
        attribute.breadth().for_each(|element| {
            let scope = index.label(element).unwrap();
            let partitions = partition(element.context(), index);
            related(partitions, index, Some(scope), &mut relations);
        });
    });

    debug!("<== Relating {:#?} contexts", attributes);
    relations.identified()
}

fn related<'view, Value: attribute::Value>(
    partitions: Vec<Vec<usize>>,
    index: &'view Valued<Attribute<Value>>,
    _scope: Option<usize>,
    relations: &'view mut Constructor<State<usize>>,
) -> &'view mut Constructor<State<usize>> {
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
                                        // let sources = State::initialize(
                                        //     value
                                        //         .context()
                                        //         .iter()
                                        //         .map(|context| index.label(context).unwrap())
                                        //         .chain(scope)
                                        //         .collect::<Vec<Vec<usize>>>()
                                        //         .borrow(),
                                        // );
                                        // let destinations = State::initialize(&[destination]);
                                        // relations.relate(sources.borrow(), destinations.borrow());
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
    relations
}

fn partition<Value: attribute::Value>(
    attributes: &[Attribute<Value>],
    index: &Valued<Attribute<Value>>,
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
                        .push(index.label(value).unwrap())
                }
            };
            accumulator
        });
    debug!("<== Partitioned {:#?} for {:#?}", partitions, attributes);
    partitions
}
