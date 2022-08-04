use std::collections::HashMap;

use carlog::*;
use futures::StreamExt;
use telegram_bot::*;

use crate::config::WolgramConfig;
use crate::error::WolgramResult;
use crate::wol::perform_wol;
use crate::WolgramError;

async fn process_command(
    message: Message,
    message_text: String,
    devices: &HashMap<String, String>,
    api: &Api,
) -> WolgramResult {
    let words: Vec<&str> = message_text.split(' ').collect();
    match words[0] {
        "/list" => {
            let mut list_text = "No devices have been configured yet.".to_string();
            if devices.len() > 0 {
                list_text = "Available devices are:".to_string();
                for (key, val) in devices {
                    let part = format!("{}\n{} - {}", list_text, key, val);
                    list_text = part;
                }
            }
            if let Err(err) = api.send(message.text_reply(list_text)).await {
                return Err(WolgramError::TelegramBotError(err));
            }
        }
        "/wake" => {
            if words.len() < 2 {
                if let Err(err) = api.send(message.text_reply("Command usage: /wake <name>")).await {
                    return Err(WolgramError::TelegramBotError(err));
                }
            } else {
                let name = words[1];
                let address = devices.get(name);
                match address {
                    Some(addr) => {
                        carlog!(
                            "Waking ",
                            format!("Trying to wake device {} with address {}", name, addr),
                            CargoColor::Yellow
                        );
                        let wol_msg = match perform_wol(addr) {
                            Ok(()) => {
                                carlog!(
                                    "Success ",
                                    format!("Sent WOL for device {} with address {}", name, addr),
                                    CargoColor::Green
                                );
                                format!("Successfully sent wake on lan request for device {} ({})", name, addr)
                            },
                            Err(err) => {
                                carlog!(
                                    "Error ",
                                    format!("Failed to send WOL for device {} with address {}. Error: {}", name, addr, err),
                                    CargoColor::Red
                                );
                                format!("Failed to send wake on lan request for device {} ({}). Error: {}", name, addr, err)
                            },
                        };
                        if let Err(err) = api.send(message.text_reply(wol_msg)).await {
                            return Err(WolgramError::TelegramBotError(err));
                        }
                    }
                    None => {
                        if let Err(err) = api.send(message.text_reply(format!("Device {} not found", name))).await {
                            return Err(WolgramError::TelegramBotError(err));
                        }
                    }
                }
            }
        }
        _ => {
            if let Err(err) = api.send(message.text_reply(format!(
                "Wolgram help:\n /help - Shows this message\n /list - Shows a list of all wakeable devices\n /wake <name> - Sends a wake request to the named device"
            ))).await {
                return Err(WolgramError::TelegramBotError(err));
            }
        }
    }
    Ok(())
}

pub async fn start_bot_loop(config: WolgramConfig) -> WolgramResult {
    let api = Api::new(config.api_key);
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = match update {
            Ok(res) => res,
            Err(err) => {
                return Err(WolgramError::TelegramBotError(err));
            }
        };
        if let UpdateKind::Message(msg) = update.kind {
            let sender = msg.chat.id();
            let sender = sender.into();
            if !config.chat_ids.contains(&sender) {
                carlog!(
                    "Unauthorized message ",
                    format!("chat_id={}", sender),
                    CargoColor::Red
                );
                continue;
            }
            if let MessageKind::Text { ref data, .. } = msg.kind {
                carlog_info!("Inbound message", format!("chat_id={} {}", sender, data));
                if data.starts_with('/') {
                    process_command(msg.clone(), data.clone(), &config.devices, &api).await?;
                }
            }
        }
    }
    Ok(())
}
