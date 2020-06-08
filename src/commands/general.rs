use serenity::{
    framework::standard::{
        help_commands,
        macros::{command, group, help},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{
        channel::Message,
        id::UserId,
    },
    prelude::*,
};
use std::collections::HashSet;

#[help]
#[strikethrough_commands_tip_in_guild(None)]
#[strikethrough_commands_tip_in_dm(None)]
fn help(
    ctx: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    help_commands::with_embeds(ctx, msg, args, help_options, groups, owners)
}

#[group]
#[commands(bark, say)]
#[description = "General commands"]
struct General;

#[command]
#[description = "Make the bot woof"]
fn bark(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Woof")?;
    Ok(())
}

#[command]
#[description = "Replies with whatever is passed to it as an argument"]
fn say(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    msg.reply(&ctx, args.rest())?;
    Ok(())
}
