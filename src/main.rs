use std::env;

use futures::StreamExt;
use telegram_bot::*;
mod geteth;

use tokio::timer::delay;
use std::time::Duration;


async fn geteth_message(api: Api, message: Message) -> Result<(), Error> {
    api.send(message.text_reply(geteth::geteth_message())).await?;

    Ok(())
}

async fn getbtc_message(api: Api, message: Message) -> Result<(), Error> {
    api.send(message.text_reply(getbtc::getbtc_message())).await?;

    Ok(())
}


async fn get_tracking(api: Api, message: Message) -> Result<(), Error> {
    match message.kind {
        MessageKind::Text { ref data, .. } => match data.as_str() {
            "/geteth" => geteth_message(api, message).await?,
            "/getbtc" => getbtc_message(api, message).await?,
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
    let mut cur_d = geteth::geteth_ts();
    println!("cur_d {}", cur_d);
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        println!("{:?}", update);
        if let UpdateKind::Message(message) = update.kind {
            get_tracking(api.clone(), message).await?;
        }

        let when = tokio::clock::now() + Duration::from_millis(2000);
        delay(when).await;
        let new_d = geteth::geteth_ts();
        let chat = ChatId::new(61031);
        // get new ticker from source
        if new_d != cur_d {
            api.spawn(chat.text(geteth::geteth_message()));
            cur_d = new_d;
        } else {
            api.spawn(chat.text("PING ME by every 2 seconds"));
        }
    }
    Ok(())
}
