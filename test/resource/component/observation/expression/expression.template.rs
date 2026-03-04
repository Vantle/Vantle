use channel::Channel;
use expression::Expression;

fn parse(input: String) -> bool {
    ::parse::parse(&input).is_ok()
}

fn evaluate(input: String, channels: Vec<String>) -> bool {
    let parsed = ::parse::parse(&input).unwrap();
    let channels: Vec<Channel> = channels
        .into_iter()
        .map(|name| Channel { name, weight: 1 })
        .collect();
    parsed.evaluate(&channels)
}

fn reject(input: String) -> bool {
    ::parse::parse(&input).is_err()
}

fn any(input: String) -> bool {
    matches!(::parse::parse(&input).unwrap(), Expression::Any)
}
