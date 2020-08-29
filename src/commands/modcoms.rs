use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    framework::standard::macros::{command, group},
    framework::standard::{StandardFramework, CommandResult},
};

//File for anything moderator or admin related


#[group]
#[commands]
struct Modcoms;

