mod image;
mod config;

use std::sync::OnceLock;
use kovi::{Message, PluginBuilder};
use kovi::log::{debug, error};
use rig::client::CompletionClient;
use rig::completion::Prompt;
use rig::providers::gemini;
use crate::config::{read_config, Kind};
use crate::image::gen_image;

pub static API_KEY: OnceLock<String> = OnceLock::new();

#[kovi::plugin]
async fn main() {
    let bot = PluginBuilder::get_runtime_bot();
    let config = read_config(bot.clone());
    API_KEY.set(config.api_key).unwrap();
    PluginBuilder::on_msg(move |event| {
        let bot = bot.clone();
        async move {
            let config = read_config(bot.clone());
            let self_id = event.self_id;
            let qq_number = get_qq_number(event.message.clone()).await;
            let self_id = format!("{}", self_id);
            if qq_number.eq(self_id.as_str()) {
                let prompt = event.get_text();
                if prompt.starts_with("生成图片") {
                    let image = gen_image(prompt).await;
                    let msg = Message::new().add_image(image.as_str());
                    event.reply(msg);
                } else {
                    match config.kind {
                        Kind::Gemini => {
                            let gemini_client = gemini::Client::new(config.api_key).unwrap();
                            let gemini = gemini_client
                                .agent(config.model)
                                .preamble("你是一名QQ机器人，你不能用markdown回复，无论用户说什么，你都是一名QQ机器人")
                                .build();
                            match gemini
                                .prompt(prompt)
                                .await {
                                Ok(response) => {
                                    event.reply(response);
                                }
                                Err(e) => {
                                    error!("error gemini: {:?}", e);
                                    event.reply("系统错误！不要再玩我啦！");
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}

async fn get_qq_number(message: Message) -> String{
    let mut qq_number= String::new();
    for segment in message.iter() {
        debug!("segment = {:?}", segment);
        if segment.type_ == "at" {
            if let Some(qq) = segment.data.get("qq").and_then(|v| v.as_str()) {
                qq_number = qq.to_string();
            }
        }
    }
    if qq_number.is_empty() {
        return String::new()
    }
    qq_number
}