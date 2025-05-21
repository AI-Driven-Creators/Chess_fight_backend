use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::{Error, Message, Result};

pub async fn send_heartbeat(
    write: &mut (impl SinkExt<Message, Error = Error> + Unpin),
) -> Result<()> {
    write.send(Message::Ping(vec![])).await?;
    Ok(())
}
