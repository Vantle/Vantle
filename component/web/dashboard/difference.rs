#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Segment {
    Index(usize),
    Key(String),
}

#[derive(Clone, Debug)]
pub struct Divergence {
    pub expected: serde_json::Value,
    pub actual: serde_json::Value,
}

#[must_use]
pub fn compare(
    expected: &serde_json::Value,
    actual: &serde_json::Value,
) -> Vec<(Vec<Segment>, Divergence)> {
    let mut result = Vec::new();
    walk(expected, actual, &mut Vec::new(), &mut result);
    result
}

fn walk(
    expected: &serde_json::Value,
    actual: &serde_json::Value,
    path: &mut Vec<Segment>,
    result: &mut Vec<(Vec<Segment>, Divergence)>,
) {
    match (expected, actual) {
        (serde_json::Value::Object(expected), serde_json::Value::Object(actual)) => {
            for (key, value) in expected {
                descend(path, Segment::Key(key.clone()), |path| {
                    match actual.get(key) {
                        Some(other) => walk(value, other, path, result),
                        None => emit(result, path, value, &serde_json::Value::Null),
                    }
                });
            }
            for (key, value) in actual {
                if expected.contains_key(key) {
                    continue;
                }
                descend(path, Segment::Key(key.clone()), |path| {
                    emit(result, path, &serde_json::Value::Null, value);
                });
            }
        }
        (serde_json::Value::Array(expected), serde_json::Value::Array(actual)) => {
            let length = expected.len().max(actual.len());
            for index in 0..length {
                descend(path, Segment::Index(index), |path| {
                    match (expected.get(index), actual.get(index)) {
                        (Some(value), Some(other)) => walk(value, other, path, result),
                        (Some(value), None) => {
                            emit(result, path, value, &serde_json::Value::Null);
                        }
                        (None, Some(value)) => {
                            emit(result, path, &serde_json::Value::Null, value);
                        }
                        (None, None) => unreachable!(),
                    }
                });
            }
        }
        _ if expected == actual => {}
        _ => emit(result, path, expected, actual),
    }
}

fn descend(path: &mut Vec<Segment>, segment: Segment, visit: impl FnOnce(&mut Vec<Segment>)) {
    path.push(segment);
    visit(path);
    path.pop();
}

fn emit(
    result: &mut Vec<(Vec<Segment>, Divergence)>,
    path: &[Segment],
    expected: &serde_json::Value,
    actual: &serde_json::Value,
) {
    result.push((
        path.to_vec(),
        Divergence {
            expected: expected.clone(),
            actual: actual.clone(),
        },
    ));
}
