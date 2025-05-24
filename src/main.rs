use log;
use pretty_env_logger;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use teloxide::types::UpdateKind;
use teloxide::types::{
    InlineQuery, InlineQueryResult, InlineQueryResultArticle, InputMessageContent,
    InputMessageContentText,
};
use teloxide::{prelude::*, update_listeners::webhooks};

fn new_rng(user_id: i64, query: &str) -> StdRng {
    // Truncate time to 30 minutes
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let t30 = now - (now % (30 * 60));
    let mut hasher = Sha256::new();
    hasher.update(&user_id.to_le_bytes());
    hasher.update(&t30.to_le_bytes());
    hasher.update(query.as_bytes());
    let hash = hasher.finalize();
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&hash);
    StdRng::from_seed(seed)
}

fn pia(query: &str, rng: &mut StdRng) -> String {
    let prefix = if rng.next_u64() % 8 == 0 {
        "Pia!▼(ｏ ‵-′)ノ★ "
    } else {
        "Pia!<(=ｏ ‵-′)ノ☆ "
    };
    format!("{}{}", prefix, query)
}

fn divine(query: &str, rng: &mut StdRng) -> String {
    let mut result = format!("所求事项: {}\n结果: ", query);
    let o = rng.next_u64() % 16;
    let omen = if o >= 9 {
        Some("吉")
    } else if o < 7 {
        Some("凶")
    } else {
        None
    };
    if let Some(omen) = omen {
        let m = rng.next_u64() % 1024;
        let mult = match m {
            0 => "极小",
            1..=10 => "超小",
            11..=55 => "特小",
            56..=175 => "甚小",
            176..=385 => "小",
            386..=637 => "",
            638..=847 => "大",
            848..=967 => "甚大",
            968..=1012 => "特大",
            1013..=1022 => "超大",
            _ => "极大",
        };
        result.push_str(mult);
        result.push_str(omen);
    } else {
        result.push_str("尚可");
    }
    result
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting Pythia Gata Bot with webhook...");

    let bot = Bot::from_env();

    let addr = ([127, 0, 0, 1], 8443).into();
    let url = "Your HTTPS ngrok URL here. Get it by `ngrok http 8443`"
        .parse()
        .unwrap();
    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    teloxide::repl_with_listener(
        bot,
        |bot: Bot, update: Update| async move {
            if let UpdateKind::InlineQuery(inline_query) = update.kind {
                handle_inline_query(&bot, &inline_query).await?;
            }
            respond(())
        },
        listener,
    )
    .await;
}

async fn handle_inline_query(bot: &Bot, query: &InlineQuery) -> ResponseResult<()> {
    let user_id = query.from.id.0 as i64;
    let query_text = &query.query;
    let locale = query.from.language_code.as_deref().unwrap_or("zh");
    let mut rng = new_rng(user_id, query_text);
    let (divine_title, pia_title) = match locale {
        "zh" => ("求签", "Pia"),
        _ => ("Divination", "Pia"),
    };
    let divine_result = InlineQueryResultArticle::new(
        "divine",
        divine_title,
        InputMessageContent::Text(InputMessageContentText::new(divine(query_text, &mut rng))),
    );
    let pia_result = InlineQueryResultArticle::new(
        "pia",
        pia_title,
        InputMessageContent::Text(InputMessageContentText::new(pia(query_text, &mut rng))),
    );
    bot.answer_inline_query(
        &query.id,
        vec![
            InlineQueryResult::Article(divine_result),
            InlineQueryResult::Article(pia_result),
        ],
    )
    .await?;
    Ok(())
}
