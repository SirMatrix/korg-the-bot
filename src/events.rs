use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::{channel::Message, gateway::Ready, gateway::Activity},
};



pub struct Handler;


#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

    }

}

