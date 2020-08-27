
use serenity::client::Context;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::*;
use tokio_core::reactor::Handle;
use std::collections::HashMap;
use serenity::framework::Framework;






const MOD_ROLE: RoleId = RoleId(692794426413547530);
const GUILD_ID: GuildId = GuildId(692401689323503637);



struct MyFramework{
    commands: HashMap<String, Box<Fn(Message, Vec<String>)>>,
}


impl Framework for MyFramework {
    fn dispatch(&mut self, _: Context, msg: Message, tokio_handle: &Handle){
        let args = msg.content.split_whitespace();
        let command = match args.advance() {
            Some(command) => {
                if !command.starts_with('~') {return;}
            },
            None => return,
        };
        let command = match self.commands.get(&command){
            Some(command) => command, None => return,
        };
        tokio_handle.spawn_fn(move || {(command)(msg,args); Ok() });
    }

}

struct Handler;


impl EventHandler for Handler {
}



use serenity::Client;


fn main(){


    let mut client = Client::new(&include_str!("bot_token.txt"),Handler).expect("Could not create new client.");

    client.start().expect("Could not start client.");

}