use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::Client;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        if msg.content == "!ping" {
            let _ = msg.channel_id.say(&context, "Pong!");
        }
    }
}


fn main(){


    let mut client = Client::new(&include_str!("bot_token.txt"),Handler).expect("Could not create new client.");

    client.start().expect("Could not start client.");

}