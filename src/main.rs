use tokio::sync::mpsc;
mod actors;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel::<actors::Message>(1);
    let tx_one = tx.clone();
    tokio::spawn(async move {
        for _ in 0..5 {
            let buy_actor = actors::BuyOrder::new(5.5, "BYND".to_owned(),tx_one.clone());
            buy_actor.send().await;
        }
        drop(tx_one);
    });
    tokio::spawn(async move {
        for _ in 0..5 {
            let buy_actor = actors::BuyOrder::new(5.5,"PLTR".to_owned(),tx.clone());
            buy_actor.send().await;
        }
        drop(tx);
    });
    let actor = actors::OrderBookActor::new(rx, 20.0);
    actor.run().await;
}
