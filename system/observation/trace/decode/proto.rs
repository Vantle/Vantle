use proto::observation as wire;

#[must_use]
pub fn update(update: wire::Update) -> Option<stream::Update> {
    match update.payload? {
        wire::update::Payload::Span(s) => Some(stream::Update::Span(span(s)?)),
        wire::update::Payload::Event(e) => Some(stream::Update::Event(event(e)?)),
        wire::update::Payload::Snapshot(s) => Some(stream::Update::Snapshot(snapshot(s))),
    }
}

#[must_use]
pub fn span(span: wire::Span) -> Option<stream::Span> {
    let id = span.id?;
    let meta = span.metadata?;
    let lifecycle = match span.event? {
        wire::span::Event::Begin(b) => stream::Lifecycle::Begin(stream::Begin {
            timestamp: b.timestamp,
            fields: b.fields.into_iter().filter_map(field).collect::<Vec<_>>(),
        }),
        wire::span::Event::End(e) => stream::Lifecycle::End(stream::End {
            timestamp: e.timestamp,
        }),
    };

    Some(stream::Span {
        id: stream::Identifier {
            trace: id.trace,
            span: id.span,
            parent: id.parent,
        },
        metadata: metadata(meta),
        channels: span.channels.into_iter().map(channel).collect::<Vec<_>>(),
        lifecycle,
    })
}

#[must_use]
pub fn event(event: wire::Event) -> Option<stream::Event> {
    Some(stream::Event {
        parent: event.parent,
        metadata: metadata(event.metadata?),
        channels: event.channels.into_iter().map(channel).collect::<Vec<_>>(),
        timestamp: event.timestamp,
        fields: event
            .fields
            .into_iter()
            .filter_map(field)
            .collect::<Vec<_>>(),
    })
}

#[must_use]
pub fn snapshot(snapshot: wire::Snapshot) -> stream::Snapshot {
    stream::Snapshot {
        timestamp: snapshot.timestamp,
        state: snapshot.state,
        trigger: snapshot.trigger,
    }
}

#[must_use]
pub fn metadata(meta: wire::Metadata) -> stream::Metadata {
    stream::Metadata {
        target: meta.target,
        name: meta.name,
        level: level(meta.level),
    }
}

#[must_use]
pub fn level(level: i32) -> stream::Level {
    match wire::Level::try_from(level).unwrap_or(wire::Level::Undefined) {
        wire::Level::Trace => stream::Level::Trace,
        wire::Level::Debug => stream::Level::Debug,
        wire::Level::Undefined | wire::Level::Info => stream::Level::Info,
        wire::Level::Warn => stream::Level::Warn,
        wire::Level::Error => stream::Level::Error,
    }
}

#[must_use]
pub fn channel(ch: wire::Channel) -> stream::channel::Channel {
    stream::channel::Channel {
        name: ch.name,
        weight: ch.weight.try_into().unwrap_or(1),
    }
}

#[must_use]
pub fn field(field: wire::Field) -> Option<stream::Field> {
    let value = match field.value? {
        wire::field::Value::Signed(v) => stream::Value::Signed(v),
        wire::field::Value::Unsigned(v) => stream::Value::Unsigned(v),
        wire::field::Value::Boolean(v) => stream::Value::Boolean(v),
        wire::field::Value::Text(v) => stream::Value::Text(v),
        wire::field::Value::Serialized(v) => stream::Value::Serialized(v),
    };
    Some(stream::Field {
        name: field.name,
        value,
    })
}
