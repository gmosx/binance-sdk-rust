use async_stream::stream;
use serde::Deserialize;

use crate::client::Client;

pub const SUBSCRIBE_METHOD: &str = "SUBSCRIBE";

pub type SubscribeParams = Vec<String>;

/// The order book levels (i.e. depth) to subscribe to).
/// Valid <levels> are 5, 10, or 20
pub enum Levels {
    L5 = 5,
    L10 = 10,
    L20 = 20,
}

#[derive(Debug, Deserialize)]
pub struct DepthEvent {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: u64,
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
}

impl Client {
    // <https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md#partial-book-depth-streams>
    pub async fn subscribe_depth(&mut self, symbol: &str, levels: Levels) {
        let topic = format!("{symbol}@depth{}@1000ms", levels as u8);

        self.call(SUBSCRIBE_METHOD, vec![topic])
            .await
            .expect("cannot send request");

        let mut messages_receiver = self
            .broadcast
            .clone()
            .expect("client not connected")
            .subscribe();

        let depth_events = stream! {
            while let Ok(msg) = messages_receiver.recv().await {
                if let Ok(msg) = serde_json::from_str::<DepthEvent>(&msg) {
                    yield msg
                }
            }
        };

        let depth_events = Box::pin(depth_events);

        self.depth_events = Some(depth_events);
    }
}
