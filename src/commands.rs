use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn quote(
    ctx: Context<'_>,
    #[description = "Who said it?"] quotee: String,
    #[description = "What did they say?"] quote: String,
) -> Result<(), Error> {
    let pool = &ctx.data().pool;
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let response = format!("\"{}\"\n \\- {}", quote, quotee);
    ctx.say(response).await?;
    Ok(())
}
