// use component::graph::matrix::Related as Graph;
// use component::graph::state::State;

// use std::hash::Hash;

// use serde::{Deserialize, Serialize};

// use itertools::Either;

// pub fn transition<T: Default + Hash + Eq + Clone + Serialize + for<'a> Deserialize<'a>>(
//     _graph: &Graph<State<T>>,
//     _state: &State<T>,
// ) -> State<T> {
//     States::initialize(
//         states
//             .stream()
//             .flat_map(|state| {
//                 let mut transitions = graph.transitions(state).into_iter();
//                 match transitions.next() {
//                     Some(transition) => {
//                         Either::Left(std::iter::once(transition).chain(transitions))
//                     }
//                     None => Either::Right(std::iter::once(state.clone())),
//                 }
//             })
//             .collect::<Vec<State<T>>>()
//             .as_slice(),
//     )
//     todo!()
// }
