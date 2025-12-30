use event::{Sink, Source};
use hypergraph::Label;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Phase {
    #[default]
    Idle,
    Hovering {
        label: Label,
        position: (f32, f32),
    },
    Selected {
        label: Label,
        position: (f32, f32),
    },
}

pub struct State {
    pub position: Option<(f32, f32)>,
    pub hovered: Option<Label>,
    pub selected: Option<Label>,
    sinks: Vec<Box<dyn Sink<Phase>>>,
}

impl State {
    #[must_use]
    pub fn new() -> Self {
        Self {
            position: None,
            hovered: None,
            selected: None,
            sinks: Vec::new(),
        }
    }

    #[expect(clippy::cast_possible_truncation)]
    pub fn moved(&mut self, x: f64, y: f64) {
        self.position = Some((x as f32, y as f32));
    }

    pub fn exited(&mut self) {
        self.position = None;
        self.hovered = None;
    }

    pub fn select(&mut self) {
        self.selected = self.hovered;
    }

    pub fn deselect(&mut self) {
        self.selected = None;
    }

    #[must_use]
    pub fn phase(&self) -> Phase {
        let position = self.position.unwrap_or((0.0, 0.0));

        match (self.selected, self.hovered) {
            (Some(label), _) => Phase::Selected { label, position },
            (None, Some(label)) => Phase::Hovering { label, position },
            _ => Phase::Idle,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl Source<Phase> for State {
    fn interact(&mut self, sink: Box<dyn Sink<Phase>>) {
        self.sinks.push(sink);
    }

    fn act(&mut self, event: &Phase) {
        for sink in &mut self.sinks {
            sink.react(event);
        }
    }
}
