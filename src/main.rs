use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::Client;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content == "!ping" {
            let _ = msg.channel_id.say(&context, "Pong!");
        }
    }
}

let mut client = Client::new("my token here").event_handler(Handler).await?;

client.start().await?;