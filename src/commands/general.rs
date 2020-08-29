use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, channel::Embed},
    prelude::*,
framework::standard::macros::{command, group},
framework::standard::{StandardFramework, CommandResult},
};

#[group]
#[commands(about, ping, guildinfo)]

struct General;



#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "My name is Korg. I am a bot made  by Sir Matrix in rust for polling and other needs!").await?;

    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "pong!").await?;

    Ok(())
}

#[command]
async fn guildinfo( ctx: &Context, msg: &Message) -> CommandResult {

    let embed = msg.channel_id.send_message(&ctx.http,  |m|{
        m.embed(|e|{
            e.title("Test Embed Title!");
            e.description("This is some cool embed stuff");
            return e;
        });
        return m;
    }).await?;

    Ok(())
}

