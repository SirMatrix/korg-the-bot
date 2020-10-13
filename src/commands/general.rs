use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, channel::Embed},
    model::id::GuildId,
    prelude::*,
framework::standard::macros::{command, group},
framework::standard::{StandardFramework, CommandResult,Args},
};
use serenity::model::guild::Target::Emoji;
use serenity::model::channel::{Reaction, ReactionType};
use serenity::model::channel::ReactionType::{Custom, Unicode};


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
    if args.len() <= 1 {
        msg.channel_id.say(&ctx.http, "Please add more than just a title.").await?;
    }else {
        let embed = msg.channel_id.send_message(&ctx.http,  |m|{
            m.embed(|e|{
                if args.len() == 2 {
                    e.title(title.unwrap());
                    e.field(":regional_indicator_a:", choice1.unwrap(), false);

                }
                else if args.len() == 3 {
                    e.title(title.unwrap());
                    e.field(":regional_indicator_a:", choice1.unwrap(), false);
                    e.field(":regional_indicator_b:", choice2.unwrap(), false);


                }else if  args.len() == 4 {
                    e.title(title.unwrap());
                    e.field(":regional_indicator_a:", choice1.unwrap(), false);
                    e.field(":regional_indicator_b:", choice2.unwrap(), false);
                    e.field(":regional_indicator_c:", choice3.unwrap(), false);


                }else if args.len() == 5{
                    e.title(title.unwrap());
                    e.field(":regional_indicator_a:", choice1.unwrap(), false);
                    e.field(":regional_indicator_b:", choice2.unwrap(), false);
                    e.field(":regional_indicator_c:", choice3.unwrap(), false);
                    e.field(":regional_indicator_d:", choice4.unwrap(), false);

                }
                return e;
            });
            return m;


        }).await?;
    }
    if args.len() == 2 {
        msg.react(&ctx.http, Unicode("🇦".to_string())).await?;
    }else if args.len() == 3 {
        msg.react(&ctx.http, Unicode("🇦".to_string())).await?;
        msg.react(&ctx.http, Unicode("🇧".to_string())).await?;
    }else if args.len() == 4 {
        msg.react(&ctx.http, Unicode("🇦".to_string())).await?;
        msg.react(&ctx.http, Unicode("🇧".to_string())).await?;
        msg.react(&ctx.http, Unicode("🇨".to_string())).await?;
    }else if args.len() == 5{
        msg.react(&ctx.http, Unicode("🇦".to_string())).await?;
        msg.react(&ctx.http, Unicode("🇧".to_string())).await?;
        msg.react(&ctx.http, Unicode("🇨".to_string())).await?;
        msg.react(&ctx.http, Unicode("🇩".to_string())).await?;
    }



    Ok(())
}

