use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, channel::Embed},
    model::id::GuildId,
    prelude::*,
framework::standard::macros::{command, group},
framework::standard::{StandardFramework, CommandResult,Args},
};

const  Guildname: GuildId = GuildId(692401689323503637);

#[group]
#[commands(about, ping, poll)]

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
async fn poll( ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let title = args.single::<String>();
    let choice1 = args.single::<String>();
    let choice2 = args.single::<String>();
    let choice3 = args.single::<String>();
    let choice4 = args.single::<String>();

    let _embed = msg.channel_id.send_message(&ctx.http,  |m|{
        m.embed(|e|{
            e.title(title.unwrap());
            e.field(":regional_indicator_a:", choice1.unwrap(), false);
            e.field(":regional_indicator_b:", choice2.unwrap(), false);
            e.field(":regional_indicator_c:", choice3.unwrap(), false);
            e.field(":regional_indicator_d:", choice4.unwrap(), false);
            return e;
        });
        return m;
    }).await?;

    Ok(())
}

