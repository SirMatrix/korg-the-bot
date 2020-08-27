use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::Client;

use serenity::{
    model::{channel::Message, gateway::Ready, guild::Member},
    model::id::{GuildId, RoleId, UserId},
    prelude::*,
};

const MOD_ROLE: RoleId = RoleId(692794426413547530);

struct Handler;

impl Handler {
    fn GetRole(&self, ctx: &Context, member: &Member) -> bool{
        if member.user.read().bot{return false};
        let access = member.roles.iter().any(|role| role.0==MOD_ROLE);

    }






}




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