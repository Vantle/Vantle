use stream::Update;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct Client {
    receiver: Receiver<Update>,
}

impl Client {
    #[must_use]
    pub fn connect(_source: &str) -> (Self, Sender<Update>) {
        let (sender, receiver) = channel(1024);
        (Self { receiver }, sender)
    }

    pub fn recv(&mut self) -> Option<Update> {
        self.receiver.try_recv().ok()
    }
}
