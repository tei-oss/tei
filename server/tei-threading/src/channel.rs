use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    Mutex,
};

pub struct UnboundedChannel<T> {
    pub tx: UnboundedSender<T>,
    pub rx: Mutex<UnboundedReceiver<T>>,
}

#[must_use]
pub fn channel_unbounded<T>() -> UnboundedChannel<T> {
    let (tx, rx) = mpsc::unbounded_channel();
    UnboundedChannel {
        tx,
        rx: Mutex::new(rx),
    }
}
