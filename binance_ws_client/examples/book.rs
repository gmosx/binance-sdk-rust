use binance_ws_client::{api::subscribe::Levels, Client};
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let mut client = Client::connect_market_data().await.expect("cannot connect");

    client.subscribe_depth("btcusdt", Levels::L10).await;

    let mut depth_events = client.depth_events.unwrap();

    while let Some(msg) = depth_events.next().await {
        dbg!(&msg);
    }
}
