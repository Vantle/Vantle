use observe::trace;
use record::debug;

use arena::{Aliased, Indexed};
use attribute::{Categorized, Contextualized};
use component::graph::attribute::{Attribute as Data, Category, Value};
use component::graph::relation::{Constructor, Related};
use component::graph::state::particle::Particle;
use component::graph::state::wave::Wave;
use relation::Relate;
use valued::Valued as Arena;

type Result<T> = std::result::Result<T, arena::error::Error>;

#[trace(channels = [core])]
pub fn relate<T: Value>(
    attribute: &Data<T>,
    index: &Arena<Data<T>>,
) -> Result<Related<Wave<usize>>> {
    debug!("==> Relating {:#?} contexts", attribute);

    let mut relations = Constructor::<Wave<usize>>::default();

    related(None, attribute.context(), index, &mut relations)?;

    debug!("<== Related {:#?} contexts", attribute);
    Ok(Related::from(&relations))
}

#[trace(channels = [core])]
fn related<T: Value>(
    latent: Option<&Particle<usize>>,
    level: &[Data<T>],
    index: &Arena<Data<T>>,
    relations: &mut Constructor<Wave<usize>>,
) -> Result<()> {
    let partitions = partition(level, index)?;

    for partition in &partitions {
        let particulates = particulate(partition, index)?;
        process(latent, &particulates, index, relations)?;
        recurse(&particulates, index, relations)?;
    }

    Ok(())
}

#[trace(channels = [core])]
fn process<T: Value>(
    latent: Option<&Particle<usize>>,
    particulates: &[Vec<usize>],
    index: &Arena<Data<T>>,
    relations: &mut Constructor<Wave<usize>>,
) -> Result<()> {
    for (position, particulate) in particulates.iter().enumerate() {
        for &element in particulate {
            let value = index.value(element)?;

            if !matches!(value.category(), Category::Context) {
                continue;
            }

            let sources = sources(value, latent, index)?;
            connect(&sources, particulates, position, latent, index, relations)?;
        }
    }

    Ok(())
}

#[trace(channels = [core])]
fn sources<T: Value>(
    value: &Data<T>,
    latent: Option<&Particle<usize>>,
    index: &Arena<Data<T>>,
) -> Result<Wave<usize>> {
    let packets = crate::partition(value.context(), index)?
        .iter()
        .map(|join| Particle::from(join.as_slice()))
        .chain(latent.iter().copied().cloned())
        .collect::<Vec<Particle<usize>>>();

    Ok(Wave::from(packets.as_slice()))
}

#[trace(channels = [core])]
fn connect<T: Value>(
    sources: &Wave<usize>,
    particulates: &[Vec<usize>],
    position: usize,
    latent: Option<&Particle<usize>>,
    index: &Arena<Data<T>>,
    relations: &mut Constructor<Wave<usize>>,
) -> Result<()> {
    for target in particulates.iter().skip(position + 1) {
        let (contexts, attributes) = classify(target, latent, index)?;

        for context in contexts {
            relations.relate(sources, &context);
        }

        if !attributes.is_empty() {
            let sinks = Wave::from([Particle::from(attributes.as_slice())].as_slice());
            relations.relate(sources, &sinks);
        }
    }

    Ok(())
}

#[trace(channels = [core])]
fn classify<T: Value>(
    target: &[usize],
    latent: Option<&Particle<usize>>,
    index: &Arena<Data<T>>,
) -> Result<(Vec<Wave<usize>>, Vec<usize>)> {
    let mut contexts = Vec::new();
    let mut attributes = Vec::new();

    for &element in target {
        let value = index.value(element)?;
        match value.category() {
            Category::Context => {
                let packets = crate::partition(value.context(), index)?
                    .iter()
                    .map(|join| Particle::from(join.as_slice()))
                    .chain(latent.iter().copied().cloned())
                    .collect::<Vec<Particle<usize>>>();
                contexts.push(Wave::from(packets.as_slice()));
            }
            _ => attributes.push(element),
        }
    }

    Ok((contexts, attributes))
}

#[trace(channels = [core])]
fn recurse<T: Value>(
    particulates: &[Vec<usize>],
    index: &Arena<Data<T>>,
    relations: &mut Constructor<Wave<usize>>,
) -> Result<()> {
    for particulate in particulates {
        for &destination in particulate {
            let value = index.value(destination)?;

            if !matches!(value.category(), Category::Group) {
                continue;
            }

            let alias = index.alias(value)?;
            let particle = Particle::from([alias].as_slice());
            related(Some(&particle), value.context(), index, relations)?;
        }
    }

    Ok(())
}

#[trace(channels = [core])]
pub fn partition<T: Value>(
    attributes: &[Data<T>],
    index: &Arena<Data<T>>,
) -> Result<Vec<Vec<usize>>> {
    debug!("==> Partitioning {:#?}", attributes);
    let mut partitions = Vec::new();

    for value in attributes {
        if value.category() == &Category::Partition {
            partitions.push(Vec::default());
        } else {
            let alias = index.alias(value)?;
            if let Some(last) = partitions.last_mut() {
                last.push(alias);
            } else {
                partitions.push(vec![alias]);
            }
        }
    }

    debug!("<== Partitioned {:#?} for {:#?}", partitions, attributes);
    Ok(partitions)
}

#[trace(channels = [core])]
pub fn particulate<T: Value>(
    partition: &[usize],
    index: &Arena<Data<T>>,
) -> Result<Vec<Vec<usize>>> {
    debug!("==> Particulating {:#?}", partition);
    let mut particulates = Vec::new();
    let mut current = Vec::new();

    for &item in partition {
        let value = index.value(item)?;
        match value.category() {
            Category::Void => {
                if !current.is_empty() {
                    particulates.push(current);
                    current = Vec::new();
                }
            }
            _ => current.push(item),
        }
    }

    if !current.is_empty() {
        particulates.push(current);
    }

    debug!("<== Particulated {:#?} from {:#?}", particulates, partition);
    Ok(particulates)
}

#[trace(channels = [core])]
pub fn signal<T: Value>(module: &Data<T>, arena: &Arena<Data<T>>) -> Result<Wave<usize>> {
    let partitions = partition(module.context(), arena)?;

    let particles = partitions
        .iter()
        .map(|partition| Particle::from(partition.as_slice()))
        .collect::<Vec<Particle<usize>>>();

    Ok(Wave::from(particles.as_slice()))
}
