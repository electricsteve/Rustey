use crate::component::Component;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use serenity::all::FullEvent;
use std::sync::Arc;

pub fn component() -> Box<Component> {
    Box::new(Component {
        id: "todo".to_string(),
        commands: vec![todo],
        event_handler: Arc::new(Handler)
    })
}

struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn dispatch(&self, _context: &serenity::all::Context, event: &FullEvent) {
        if let FullEvent::Ready { data_about_bot: _data_about_bot, .. } = event {
            println!("Todo component loaded!");
        }
    }
}

/// Check your own to-do list
#[poise::command(slash_command, prefix_command)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let user_id = ctx.author().id;
    let data = ctx.data();
    let todo_list = {
        let hash_map = data.todo_map.lock().unwrap();
        let list = hash_map.get(&user_id);
        if let Some(list) = list {
            list.clone()
        } else {
            Vec::new()
        }
    };
    if todo_list.is_empty() {
        ctx.say("There is currently nothing on your to-do list.").await?;
    } else {
        let formatted_list: String = todo_list.join("\n- ");
        ctx.say(format!("These are the items on your to-do list:\n- {}", formatted_list)).await?;
    }
    Ok(())
}

#[poise::command(prefix_command, slash_command, subcommands("list", "add"), subcommand_required)]
pub async fn todo(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn add(ctx: Context<'_>, content: String) -> Result<(), Error> {
    let user_id = ctx.author().id;
    let data = ctx.data();
    {
        let mut hash_map = data.todo_map.lock().unwrap();
        let num_votes = hash_map.entry(user_id).or_default();
        num_votes.push(content.clone());
    }
    ctx.say(format!("Successfully added `{content}` to your to-do list!")).await?;
    Ok(())
}