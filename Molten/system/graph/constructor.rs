use log::debug;

use component::graph::arena::Valued;
use component::graph::attribute::{Attribute, Category, Value};
use component::graph::matrix::{Constructor, Related as Relations};
use component::graph::state::{Particle, Wave};
use component::graph::traits::attribute::{Categorized, Contextualized};
use component::graph::traits::node::{Aliased as _, Valued as _};

pub fn relate<T: Value>(
    attribute: &Attribute<T>,
    index: &Valued<Attribute<T>>,
) -> Relations<Wave<usize>> {
    debug!("==> Relating {:#?} contexts", attribute);

    let mut relations = Constructor::<Wave<usize>>::default();

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
    latent: Option<Particle<usize>>,
    level: &[Attribute<T>],
    index: &Valued<Attribute<T>>,
    relations: &mut Constructor<Wave<usize>>,
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
                let sources = {
                    let packets: Vec<Particle<usize>> = crate::partition(value.context(), index)
                        .iter()
                        .map(|join| Particle::from(join.as_slice()))
                        .chain(latent.iter().cloned())
                        .collect();

                    Wave::from(packets.as_slice())
                };

                partition
                    .iter()
                    .filter(|&&destination| destination != source)
                    .for_each(|&destination| {
                        let alias = index.value(destination).unwrap();
                        let sinks = match alias.category() {
                            Category::Context => {
                                let packets: Vec<Particle<usize>> =
                                    crate::partition(alias.context(), index)
                                        .iter()
                                        .map(|join| Particle::from(join.as_slice()))
                                        .chain(latent.iter().cloned())
                                        .collect();

                                Wave::from(packets.as_slice())
                            }
                            _ => Wave::<usize>::monochromatic(Particle::fundamental(destination)),
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
                    Some(Particle::fundamental(index.alias(value).unwrap())),
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
