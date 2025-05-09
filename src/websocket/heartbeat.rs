use tokio_tungstenite::tungstenite::{Message, Result, Error};
use futures_util::SinkExt;

pub async fn send_heartbeat(
    write: &mut (impl SinkExt<Message, Error = Error> + Unpin),
) -> Result<()> {
    write.send(Message::Ping(vec![])).await?;
    Ok(())
} 