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
#[prefixes("emoji", "em")]
#[commands(santa, nice)]
#[description = "Sends emoji's into chat"]
#[default_command(nice)]
struct Emoji;

#[command]
#[description = "Sends Santa emoji"]
fn santa(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, ":santa_tone1:")?;
    Ok(())
}

#[command]
#[description = "You already know what it is"]
fn nice(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, ":eggplant: :sweat_drops:")?;
    Ok(())
}
