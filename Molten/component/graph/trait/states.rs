pub trait Stateful<T> {
    fn scale(&self, basis: &T) -> Option<usize>;
    fn product(&self, beta: &Self) -> Self;

    fn union(&self, beta: &Self) -> Option<&Self>;
    fn intersection(&self, test: &Self) -> Option<&Self>;

    fn elimination(&self, test: &Self) -> Option<&Self>;
    fn divergence(&self, test: &Self) -> Option<&Self>;

    fn subset(&self, test: &Self) -> Option<&Self>;
    fn superset(&self, test: &Self) -> Option<&Self>;

    fn disjoint(&self, test: &Self) -> Option<&Self>;
    fn equivalence(&self, test: &Self) -> Option<&Self>;
}
