use crate::utils::embeds::{embed_add_details, get_bot_icon_attachment};
use crate::{Context, Error, ErrorType};
use poise::CreateReply;
use poise::serenity_prelude::{CreateEmbed, Timestamp, User};

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
    // Sadly no way to actually get the bio of a user :(
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
    let member = guild.member(ctx.as_ref(), user.id).await?;
    let guild_join_date = member.joined_at.map_or(
        // We don't fail here, because this isn't a big enough of a problem
        "Unknown".to_string(),
        |joined_at| joined_at.format("%B %e, %Y").to_string(),
    );
    let mut roles = member.roles(ctx.as_ref()).ok_or_else(|| {
        ErrorType::NotFound("No guild found to get guild specific details".to_string())
    })?;
    roles.sort();
    let embed = CreateEmbed::default()
        .title(format!("{display_name}'s User Details"))
        .description(format!("Details about the Discord user {display_name}."))
        .timestamp(Timestamp::now())
        .thumbnail(member.avatar_url().unwrap_or_else(|| user.avatar_url().unwrap_or_default()));
    let embed = embed_add_details(ctx, embed);
    let attachment = get_bot_icon_attachment();
    let reply = CreateReply::default().embed(embed).attachment(attachment);
    ctx.send(reply).await?;
    Ok(())
}
