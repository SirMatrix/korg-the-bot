mod events;
mod commands;

use commands::GENERAL_GROUP;
//use commands::MODCOMS_GROUP;


use events::Handler;

use serenity::{
    model::{channel::Message, gateway::Ready, gateway::Activity},
    prelude::*,
    framework::standard::macros::{command, group},
    framework::standard::{StandardFramework, CommandResult},
};




#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);
        //.group(MODCOMS_GROUP);


    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.

    let mut client = Client::new(&include_str!("bot_token.txt"))
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");


    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}