use proto::observation as wire;

#[must_use]
pub fn update(update: stream::Update) -> wire::Update {
    let payload = match update {
        stream::Update::Span(s) => wire::update::Payload::Span(span(s)),
        stream::Update::Event(e) => wire::update::Payload::Event(event(e)),
        stream::Update::Snapshot(s) => wire::update::Payload::Snapshot(snapshot(s)),
    };
    wire::Update {
        payload: Some(payload),
    }
}

#[must_use]
pub fn span(span: stream::Span) -> wire::Span {
    let event = match span.lifecycle {
        stream::Lifecycle::Begin(begin) => wire::span::Event::Begin(wire::Begin {
            timestamp: begin.timestamp,
            fields: begin.fields.into_iter().map(field).collect::<Vec<_>>(),
        }),
        stream::Lifecycle::End(end) => wire::span::Event::End(wire::End {
            timestamp: end.timestamp,
        }),
    };

    wire::Span {
        id: Some(wire::Identifier {
            trace: span.id.trace,
            span: span.id.span,
            parent: span.id.parent,
        }),
        metadata: Some(metadata(span.metadata)),
        channels: span.channels.into_iter().map(channel).collect::<Vec<_>>(),
        event: Some(event),
    }
}

#[must_use]
pub fn channel(ch: stream::channel::Channel) -> wire::Channel {
    wire::Channel {
        name: ch.name,
        weight: u32::from(ch.weight),
    }
}

#[must_use]
pub fn event(event: stream::Event) -> wire::Event {
    wire::Event {
        parent: event.parent,
        metadata: Some(metadata(event.metadata)),
        channels: event.channels.into_iter().map(channel).collect::<Vec<_>>(),
        timestamp: event.timestamp,
        fields: event.fields.into_iter().map(field).collect::<Vec<_>>(),
    }
}

#[must_use]
pub fn snapshot(snapshot: stream::Snapshot) -> wire::Snapshot {
    wire::Snapshot {
        timestamp: snapshot.timestamp,
        state: snapshot.state,
        trigger: snapshot.trigger,
    }
}

#[must_use]
pub fn metadata(metadata: stream::Metadata) -> wire::Metadata {
    wire::Metadata {
        target: metadata.target,
        name: metadata.name,
        level: level(metadata.level) as i32,
    }
}

#[must_use]
pub fn level(level: stream::Level) -> wire::Level {
    match level {
        stream::Level::Trace => wire::Level::Trace,
        stream::Level::Debug => wire::Level::Debug,
        stream::Level::Info => wire::Level::Info,
        stream::Level::Warn => wire::Level::Warn,
        stream::Level::Error => wire::Level::Error,
    }
}

#[must_use]
pub fn field(field: stream::Field) -> wire::Field {
    let value = match field.value {
        stream::Value::Signed(v) => wire::field::Value::Signed(v),
        stream::Value::Unsigned(v) => wire::field::Value::Unsigned(v),
        stream::Value::Boolean(v) => wire::field::Value::Boolean(v),
        stream::Value::Text(v) => wire::field::Value::Text(v),
        stream::Value::Serialized(v) => wire::field::Value::Serialized(v),
    };
    wire::Field {
        name: field.name,
        value: Some(value),
    }
}
