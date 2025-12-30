use channel::Channel;

fn parse(input: String) -> Vec<(String, u8)> {
    match Channel::parse(&input) {
        Ok(channels) => channels
            .iter()
            .map(|c| (c.name.clone(), c.weight))
            .collect(),
        Err(_) => vec![],
    }
}

fn duplicate(input: String) -> bool {
    Channel::parse(&input).is_err()
}

fn serialize(channels: Vec<(String, u8)>) -> String {
    let parsed: Vec<Channel> = channels
        .into_iter()
        .map(|(name, weight)| Channel { name, weight })
        .collect();
    Channel::serialize(&parsed)
}

fn matches(channels: Vec<String>, filter: Vec<String>) -> bool {
    let parsed: Vec<Channel> = channels
        .into_iter()
        .map(|name| Channel { name, weight: 1 })
        .collect();
    let refs: Vec<&str> = filter.iter().map(String::as_str).collect();
    Channel::matches(&parsed, &refs)
}
