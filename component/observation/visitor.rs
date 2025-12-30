use stream::{Field, Value, channel, coerce};

#[derive(Default)]
pub struct Visitor {
    pub channels: Option<String>,
    pub fields: Vec<Field>,
}

impl Visitor {
    pub fn channels(&self) -> Result<Vec<channel::Channel>, channel::Error> {
        channel::Channel::parse(self.channels.as_deref().unwrap_or(""))
    }
}

impl tracing::field::Visit for Visitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "channels" {
            self.channels = Some(value.to_string());
        } else {
            self.fields.push(Field {
                name: field.name().to_string(),
                value: Value::Text(value.to_string()),
            });
        }
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        let text = format!("{value:?}");

        if field.name() == "channels" {
            self.channels = Some(text.trim_matches('"').to_string());
            return;
        }

        self.fields.push(coerce(field.name(), text));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.fields.push(Field {
            name: field.name().to_string(),
            value: Value::Signed(value),
        });
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.fields.push(Field {
            name: field.name().to_string(),
            value: Value::Unsigned(value),
        });
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.fields.push(Field {
            name: field.name().to_string(),
            value: Value::Boolean(value),
        });
    }
}
