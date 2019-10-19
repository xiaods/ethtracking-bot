use std::env;

use futures::StreamExt;
use telegram_bot::*;

mod geteth;

async fn geteth_message(api: Api, message: Message) -> Result<(), Error> {
    api.send(message.text_reply(geteth::geteth_message())).await?;

    Ok(())
}


async fn get_tracking(api: Api, message: Message) -> Result<(), Error> {
    match message.kind {
        MessageKind::Text { ref data, .. } => match data.as_str() {
            "/geteth" => geteth_message(api, message).await?,
            _ => (),
        },
        _ => (),
    };

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            get_tracking(api.clone(), message).await?;
        }
    }
    Ok(())
}
