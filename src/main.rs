use futures::pin_mut;
use futures::{future, stream::StreamExt};
use log::info;
use nostr_sdk::prelude::*;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_stream::wrappers::BroadcastStream;

#[tokio::main]
async fn main() {
    env_logger::init();
    let keys = Keys::generate();
    let nostr_client = Client::new(&keys);
    let mut fd = File::create("lud16.txt").await.unwrap();
    let relays: Vec<String> = reqwest::get("https://api.nostr.watch/v1/online")
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    for url in relays {
        nostr_client
            .add_relay(url.clone(), None)
            .await
            .expect(&format!("{} connects", url));
        nostr_client.connect().await;
        info!("connected {}", url);

        let filter = Filter::new()
            .kind(Kind::Metadata)
            .since(Timestamp::now() - Duration::from_secs(15552000)); // minus six months
        nostr_client.subscribe(vec![filter.clone()]).await;
        info!("subscribed {:?}", filter);

        let stream = BroadcastStream::new(nostr_client.notifications())
            .filter_map(|x| future::ready(x.ok()))
            .take_while(|x| {
                future::ready(!matches!(
                    x,
                    RelayPoolNotification::Message(_, RelayMessage::EndOfStoredEvents(_))
                ))
            })
            .filter_map(|x| async {
                match x {
                    RelayPoolNotification::Event(_, e) => Some(e),
                    _ => None,
                }
            })
            .filter_map(|e| async move {
                match e.kind {
                    Kind::Metadata => {
                        let metadata =
                            Metadata::from_json(e.content).unwrap_or(Metadata::default());
                        metadata.lud16
                    }
                    _ => None,
                }
            })
            .filter(|l| future::ready(l.ends_with("walletofsatoshi.com")));

        pin_mut!(stream);

        while let Some(lud16) = stream.next().await {
            let msg = format!("{}\n", lud16);
            fd.write_all(msg.as_bytes()).await.unwrap();
        }

        nostr_client.disconnect().await.unwrap();
        info!("disconnected {}", url);
    }
}
