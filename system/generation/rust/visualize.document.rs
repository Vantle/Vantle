macro_rules! gather {
    ($($module:ident),* $(,)?) => {{
        let mut groups = Vec::new();
        $(groups.extend($module::cards());)*
        groups
    }};
}

fn main() -> miette::Result<()> {
    html::execute(|arguments| {
        let groups = gather![
            particle_component_document,
            wave_component_document,
            relation_component_document,
            attribute_component_document,
            syntax_document,
            translator_component_document,
            snapshot_document,
            query_document,
            arena_system_document,
            arena_observation_document,
            graph_document,
            constructor_document,
            relation_system_document,
            index_document,
            breadth_document,
            particle_state_document,
            wave_state_document,
            translator_symbolic_document,
            renderer_document,
            arena_symbolic_document,
            partition_document,
            group_document,
            context_document,
            module_document,
            attribute_symbolic_document,
            traversal_document,
            evaluate_document,
            evaluate_observation_document,
            translate_document,
            scale_document,
            observation_document,
            lifecycle_document,
            channel_document,
            expression_observation_document,
            expression_math_document,
            vector_document,
            quaternion_document,
            color_document,
            proportion_document,
            render_document,
            level_document,
            span_document,
            hierarchy_document,
            filtering_document,
            peer_document,
            simple_document,
            complex_document,
            library_document,
            returns_document,
            similarity_document,
        ];

        html::generate(
            arguments,
            visualize::page(&arguments.root, groups, |body, group| {
                card::cases(card::source(body, group), group, |returns, unexpected| {
                    let actual = unexpected?;
                    let divergences = difference::compare(returns, actual);
                    if divergences.is_empty() {
                        return None;
                    }
                    json::diff(returns, &divergences, 61).ok()
                })
            }),
        )
    })
}
