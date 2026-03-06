use expression::Expression;
use layer::Backpressure;
use parse::parse;
use tracing::level_filters::LevelFilter;
use url::Url;

pub struct Stream {
    pub url: Url,
    pub level: LevelFilter,
    pub channels: Expression,
}

pub enum Sink {
    Log {
        stream: Stream,
        ansi: bool,
    },
    Chrome(Stream),
    Grpc {
        stream: Stream,
        backpressure: Backpressure,
    },
    Http(Stream),
}

#[must_use]
pub fn stdout() -> Sink {
    Sink::Log {
        stream: Stream {
            url: Url::parse("log://1").expect("static url"),
            level: LevelFilter::INFO,
            channels: Expression::Any,
        },
        ansi: true,
    }
}

pub fn resolve(address: &str) -> error::Result<Sink> {
    let url = Url::parse(address).map_err(|source| error::Error::Parse {
        address: address.to_string(),
        source,
    })?;

    let channels = channels(&url).map_err(|source| error::Error::Channels {
        expression: url
            .query_pairs()
            .find(|(k, _)| k == "channels")
            .map_or_else(String::new, |(_, v)| v.to_string()),
        source,
    })?;

    let stream = Stream {
        level: level(&url),
        channels,
        url,
    };

    match stream.url.scheme() {
        "log" => Ok(Sink::Log {
            ansi: ansi(&stream.url),
            stream,
        }),
        "chrome" => Ok(Sink::Chrome(stream)),
        "grpc" => Ok(Sink::Grpc {
            backpressure: backpressure(&stream.url),
            stream,
        }),
        "http" | "https" => Ok(Sink::Http(stream)),
        scheme => Err(error::Error::Scheme {
            scheme: scheme.to_string(),
        }
        .into()),
    }
}

#[must_use]
pub fn normalize(address: &str) -> String {
    if address.contains("://") {
        return address.to_string();
    }

    let path = std::path::Path::new(address);
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().map_or_else(|_| path.to_path_buf(), |cwd| cwd.join(path))
    };

    format!("log:///{}", absolute.display())
}

fn level(url: &Url) -> LevelFilter {
    url.query_pairs()
        .find(|(k, _)| k == "level")
        .and_then(|(_, v)| v.parse::<LevelFilter>().ok())
        .unwrap_or(LevelFilter::INFO)
}

fn ansi(url: &Url) -> bool {
    url.query_pairs()
        .find(|(k, _)| k == "ansi")
        .and_then(|(_, v)| v.parse::<bool>().ok())
        .unwrap_or(false)
}

fn channels(url: &Url) -> Result<Expression, expression::Sourced> {
    url.query_pairs()
        .find(|(k, _)| k == "channels")
        .map_or_else(|| Ok(Expression::Any), |(_, v)| parse(&v))
}

fn backpressure(url: &Url) -> Backpressure {
    url.query_pairs().find(|(k, _)| k == "backpressure").map_or(
        Backpressure::default(),
        |(_, v)| {
            let value = v.as_ref();
            if value == "block" {
                return Backpressure::Block;
            }
            if let Some(capacity) = value
                .strip_prefix("drop(")
                .and_then(|s| s.strip_suffix(')'))
                .and_then(|s| s.parse::<usize>().ok())
            {
                return Backpressure::Drop(capacity);
            }
            Backpressure::default()
        },
    )
}
