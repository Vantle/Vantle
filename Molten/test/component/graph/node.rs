#[cfg(test)]
mod tests {
    use component::graph::node::Node;
    use component::graph::traits::node::{Queryable, Translatable};

    #[test]
    fn query() {
        let state_a: Node<String> = Node::from(&["a", "b", "c"][..]);
        let state_b: Node<String> = Node::from(&["a", "b"][..]);
        let state_c: Node<String> = Node::from(&["d", "e"][..]);

        // Test subset: state_b should be a subset of state_a
        assert!(state_b.subset(&state_a).is_some());
        assert!(state_a.subset(&state_b).is_none());

        // Test superset: state_a should be a superset of state_b
        assert!(state_a.superset(&state_b).is_some());
        assert!(state_b.superset(&state_a).is_none());

        // Test joint: state_a and state_b should have common elements
        assert!(state_a.joint(&state_b).is_some());
        assert!(state_a.joint(&state_c).is_none());

        // Test disjoint: state_a and state_c should have no common elements
        assert!(state_a.disjoint(&state_c).is_some());
        assert!(state_a.disjoint(&state_b).is_none());

        // Test isomorphic: state should be isomorphic to itself
        assert!(state_a.isomorphic(&state_a).is_some());
        assert!(state_a.isomorphic(&state_b).is_none());
    }

    #[test]
    fn translate() {
        let state_a: Node<String> = Node::from(&["a", "b", "c"][..]);
        let state_b: Node<String> = Node::from(&["a", "b"][..]);
        let state_c: Node<String> = Node::from(&["d", "e"][..]);

        // Test join: should always return Some
        let join_result = state_a.join(&state_b);
        assert!(join_result.is_some());

        // Test intersect: should return Some when there are common elements
        let intersect_result = state_a.intersect(&state_b);
        assert!(intersect_result.is_some());

        // Test intersect with disjoint sets: should return None
        let intersect_empty = state_a.intersect(&state_c);
        assert!(intersect_empty.is_none());

        // Test diverge: should return Some when there are differences
        let diverge_result = state_a.diverge(&state_b);
        assert!(diverge_result.is_some());

        // Test diverge when subset: should return None
        let diverge_empty = state_b.diverge(&state_a);
        assert!(diverge_empty.is_none());
    }

    #[test]
    fn construct() {
        let empty_state = Node::<String>::default();
        let single_state: Node<String> = Node::from(&["test"][..]);
        let multiple_state: Node<String> = Node::from(&["a", "b", "a", "c", "b", "a"][..]);

        // Test that states are created correctly
        assert!(empty_state.isomorphic(&empty_state).is_some());
        assert!(single_state.isomorphic(&single_state).is_some());
        assert!(multiple_state.isomorphic(&multiple_state).is_some());

        // Test that different states are not isomorphic
        assert!(empty_state.isomorphic(&single_state).is_none());
        assert!(single_state.isomorphic(&multiple_state).is_none());

        // Test empty state properties
        assert!(empty_state.disjoint(&single_state).is_some());
        assert!(empty_state.joint(&single_state).is_none());

        // Demonstrate generic From implementation versatility
        let int_state: Node<i32> = Node::from(&[1, 2, 3, 1, 2][..]);
        let char_state: Node<char> = Node::from(&['a', 'b', 'c', 'a'][..]);

        assert!(int_state.isomorphic(&int_state).is_some());
        assert!(char_state.isomorphic(&char_state).is_some());
    }
}
