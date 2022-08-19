use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use chrono::offset::Utc;
use chrono::SecondsFormat;
use log::{error, info};
use serenity::async_trait;
use serenity::http::{CacheHttp, Typing};
use serenity::model::channel::Message;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::prelude::RoleId;
use serenity::prelude::*;

use crate::transactions_per_secound::{get_tps, get_tps_string};

mod transactions_per_secound;

struct Handler {
    is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!tps") {
            let typing = Typing::start(ctx.http.clone(), msg.channel_id.0).unwrap();

            let tps = get_tps();
            let mut message = "error - try again!".to_string();
            if tps > 0 {
                message = get_tps_string(tps);
            }

            if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                eprintln!("Error sending message: {:?}", why);
            }
            let _ = typing.stop();
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");

        let ctx = Arc::new(ctx);

        if !self.is_loop_running.load(Ordering::Relaxed) {
            let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    set_status_to_current_time(Arc::clone(&ctx2), _guilds.clone()).await;
                    tokio::time::sleep(Duration::from_secs(env::var("LOOP_UPDATE_SLEEP").unwrap_or("60".to_string()).parse::<u64>().unwrap_or(60))).await;
                }
            });

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}

async fn set_status_to_current_time(ctx: Arc<Context>, _guilds: Vec<GuildId>) {
    let current_time = Utc::now();
    let formatted_time = current_time.to_rfc3339_opts(SecondsFormat::Secs, true);

    let tps = get_tps();
    //SET Name
    for guild in _guilds.clone() {
        match guild.edit_nickname(&ctx.http, Some(&*get_tps_string(tps))).await {
            Ok(_) => { info!("Changed Bot nickname!") }
            Err(_) => { error!("Unable to change bot nickname!") }
        }
    }

    //GET colors
    let mut red_role_id: RoleId = Default::default();
    let mut green_role_id: RoleId = Default::default();
    for guild in _guilds.clone() {
        let roles = guild.roles(&ctx).await;
        for (_role_id, role) in roles.expect("no guild roles found!") {
            if role.name.contains("tickers-red") {
                red_role_id = role.id;
            }
            if role.name.contains("tickers-green") {
                green_role_id = role.id;
            }
        }
    }

    //Change Bot-Color
    let threshold = env::var("TPS_THRESHOLD").unwrap_or("2000".to_string()).parse::<i64>().unwrap();
    for guild in _guilds.clone() {
        if tps > threshold {
            guild.member(&ctx.http, &ctx.cache.current_user_id()).await.unwrap().remove_role(&ctx, red_role_id).await.unwrap();
            guild.member(&ctx.http, &ctx.cache.current_user_id()).await.unwrap().add_role(&ctx, green_role_id).await.unwrap();
        } else {
            guild.member(&ctx.http, &ctx.cache.current_user_id()).await.unwrap().remove_role(&ctx, green_role_id).await.unwrap();
            guild.member(&ctx.http, &ctx.cache.current_user_id()).await.unwrap().add_role(&ctx, red_role_id).await.unwrap();
        }
    }


    ctx.set_activity(Activity::playing(&formatted_time)).await;
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("--- BOT STARTED ---");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
        })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}