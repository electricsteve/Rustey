use crate::{Context, Error, ErrorType};
use poise::serenity_prelude::User;

#[poise::command(prefix_command, slash_command, subcommands("user"), subcommand_required)]
pub async fn moderation(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn user(ctx: Context<'_>, user: User) -> Result<(), Error> {
    // General info
    let display_name = user.display_name();
    let user_name = user.name.as_str();
    let discord_join_date = user.id.created_at().format("%B %e, %Y").to_string();
    // about me here, IDK how to get it yet
    // Guild specific info
    let guild = if let Some(guild) = ctx.guild() {
        guild.clone()
    } else {
        // We intentionally fail if no specific guild details found, because this command is only meant to be run in that context.
        return Err(ErrorType::NotFound(
            "No guild found to get guild specific details".to_string(),
        )
        .into());
    };
    let guild_join_date = guild.member(ctx.as_ref(), user.id).await?.joined_at.map_or(
        // We don't fail here, because this isn't a big enough of a problem
        "Unknown".to_string(),
        |joined_at| joined_at.format("%B %e, %Y").to_string(),
    );
    // roles
    Ok(())
}
