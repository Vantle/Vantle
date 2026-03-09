use similarity::nearest;

fn suggest(target: String, candidates: Vec<String>) -> Option<String> {
    nearest(&target, &candidates)
}
