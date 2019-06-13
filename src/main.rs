use std::env::var;
use std::net::{IpAddr, Ipv4Addr};

use telegram_types::bot::inline_mode::{
    AnswerInlineQuery, InlineQueryResult, InlineQueryResultArticle, InputMessageContent,
    InputTextMessageContent, ResultId,
};
use telegram_types::bot::types::{ParseMode, Update, UpdateContent};

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use env_logger;
use uuid::Uuid;

fn get_result_id() -> ResultId {
    // UUID is a 128 bits number and can be represented as 32 hexadecimal digits.
    let mut uuid = [b'!'; 32];

    // It must be a valid ASCII string.
    Uuid::new_v4().to_simple().encode_lower(&mut uuid);

    // If there is an error forming strings from UUID bytes, something has gone terribly wrong.
    // We might as well panic there.
    ResultId(std::str::from_utf8(&uuid).unwrap().to_owned())
}

fn form_result() -> InlineQueryResult<'static> {
    let id = get_result_id();

    let title = "求签".to_string();
    let message = "大凶".to_string();
    let description = "求签".to_string();

    InlineQueryResult::Article(InlineQueryResultArticle {
        id,
        title: title.into(),
        input_message_content: InputMessageContent::Text(InputTextMessageContent {
            message_text: message.into(),
            parse_mode: Some(ParseMode::HTML),
            disable_web_page_preview: Some(true),
        }),
        reply_markup: None,
        url: None,
        hide_url: None,
        description: if description.is_empty() {
            None
        } else {
            Some(description.into())
        },
        thumb_url: None,
        thumb_width: None,
        thumb_height: None,
    })
}

fn handler(u: web::Json<Update>) -> HttpResponse {

    match &u.content {
        UpdateContent::InlineQuery(m) => {
            let mut results = Vec::new();
            results.push(form_result());

            HttpResponse::Ok().json(AnswerInlineQuery {
                inline_query_id: m.id.clone(),
                results: results.into(),
                cache_time: None,
                is_personal: None,
                next_offset: None,
                switch_pm_text: None,
                switch_pm_parameter: None,
            })
        },
        _ => HttpResponse::Ok().finish()
    }

}

fn main() -> std::io::Result<()> {
    let host: IpAddr = var("HOST")
        .ok()
        .and_then(|host| host.parse().ok())
        .unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    let port: u16 = var("PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or(8080);

    env_logger::init();

    HttpServer::new(|| App::new()
        .wrap(Logger::default())
        .service(web::resource("/").to(handler)))
        .bind((host, port))?
        .run()
}
