use std::sync::mpsc::Sender;
use crate::agent::exchange::Exchange;
use crate::application::Application;

pub struct Agent {
    pub app: Application,
    pub exchanges: Vec<Exchange>,
    pub exchange_tx: Sender<Exchange>,

    pub thread_id: uuid::Uuid,
    pub query_id: uuid::Uuid,

    /// Indicate whether the request was answered.
    ///
    /// This is used in the `Drop` handler, in order to track cancelled answer queries.
    pub complete: bool,
}