use crate::Envelope;
use tokio::sync::broadcast;

pub struct EventBus {
    tx: broadcast::Sender<Envelope>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    pub fn emit(&self, envelope: Envelope) {
        let _ = self.tx.send(envelope);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Envelope> {
        self.tx.subscribe()
    }
}
