use diesel::r2d2::ConnectionManager;
use diesel::{r2d2::Pool, SqliteConnection};
use poise::{
    serenity_prelude as serenity,
    PrefixFrameworkOptions,
};

struct Data {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

mod commands;
mod models;
mod schema;

fn establish_connection(database_url: &str) -> Pool<ConnectionManager<SqliteConnection>> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Couldn't build connection pool")
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let pool = establish_connection(&database_url);

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        // Options for poise framework
        .options(poise::FrameworkOptions {
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            commands: vec![commands::quote(), commands::new_person(), commands::people(), commands::get_quotes(), commands::get_quotes_id(), commands::all_quotes(), commands::drop_quote()],
            ..Default::default()
        })
        // Poise code on bot ready state
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                println!("Bot login at {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { pool })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
