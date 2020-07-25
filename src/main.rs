mod kibela;
mod slack;
mod types;

use kibela::url::{parse_url, UrlType as KibelaUrlType};
use slack::{
    events::{Event, EventCallback, Link},
    types::{Unfurl, UnfurlBody},
};
use std::collections::HashMap;
use types::Result;

fn collapse_whitespace(from: &str) -> String {
    let s: Vec<&str> = from.split_whitespace().collect();
    s.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn must_collapse() {
        assert_eq!(collapse_whitespace("abc\n\ndef   ghi"), "abc def ghi")
    }
}

#[derive(Clone)]
pub struct State {
    kibela_client: kibela::client::Client,
    slack_client: slack::client::Client,
}

async fn process_links(
    client: &kibela::client::Client,
    links: Vec<Link>,
) -> Result<HashMap<String, UnfurlBody>> {
    let mut bodies: HashMap<String, UnfurlBody> = HashMap::new();
    for link in links {
        let result = match parse_url(&link.url) {
            Some(KibelaUrlType::Note {
                path,
                comment_id: Some(comment_id),
            }) => {
                let note = client.note(path).await;
                let comment = client.comment(comment_id).await;
                match (note, comment) {
                    (Ok(note), Ok(comment)) => {
                        bodies.insert(link.url.to_string(), UnfurlBody::Attachment {
                            author_link: comment.author.url.to_string(),
                            author_name: format!("@{}", comment.author.account),
                            title: format!("「{}」へのコメント", note.title),
                            title_link: link.url.to_string(),
                            // ブラウザーがホワイトスペースを詰める Kibe.la サイトでの挙動にあわせる
                            text: collapse_whitespace(comment.summary.as_str()),
                            ts: comment.published_at.timestamp_millis().to_string(),
                            footer: "Kibela".to_string(),
                            footer_icon: "https://cdn.kibe.la/assets/shortcut_icon-99b5d6891a0a53624ab74ef26a28079e37c4f953af6ea62396f060d3916df061.png".to_string(),
                        });
                        Ok(())
                    }
                    // FIXME: 地獄なのでなんとかできないか
                    (Err(e1), Err(e2)) => {
                        Err(format!("retruns error {}, {}", e1.to_string(), e2.to_string()).into())
                    }
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e),
                }
            }
            Some(KibelaUrlType::Note {
                path,
                comment_id: None,
            }) => {
                let note = client.note(path).await;
                match note {
                    Ok(note) => {
                        bodies.insert(link.url.to_string(), UnfurlBody::Attachment {
                            author_link: note.author.url.to_string(),
                            author_name: format!("@{}", note.author.account),
                            title: note.title,
                            title_link: link.url.to_string(),
                            // ブラウザーがホワイトスペースを詰める Kibe.la サイトでの挙動にあわせる
                            text: collapse_whitespace(note.summary.as_str()),
                            ts: note.published_at.timestamp_millis().to_string(),
                            footer: "Kibela".to_string(),
                            footer_icon: "https://cdn.kibe.la/assets/shortcut_icon-99b5d6891a0a53624ab74ef26a28079e37c4f953af6ea62396f060d3916df061.png".to_string(),
                        });
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            }
            _ => Err("not match".into()),
        };
        if let Err(e) = result {
            dbg!(e);
        }
    }
    Ok(bodies)
}

#[async_std::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    tide::log::start();

    let kibela_client = kibela::client::new(
        std::env::var("SYAKUSI_KIBELA_TEAM_NAME").unwrap(),
        std::env::var("SYAKUSI_KIBELA_ACCESS_TOKEN").unwrap(),
    );
    let slack_client = slack::client::new(std::env::var("SYAKUSI_SLACK_BOT_USER_TOKEN").unwrap());
    let state = State {
        kibela_client,
        slack_client,
    };

    let mut app = tide::with_state(state);
    app.at("/")
        .post(|mut req: tide::Request<State>| async move {
            let event: Event = req.body_json().await.map_err(|err| {
                dbg!(&err);
                err
            })?;
            let state = req.state();
            match event {
                Event::UrlVerification { challenge } => Ok(challenge),
                Event::EventCallback {
                    event:
                        EventCallback::LinkShared {
                            channel,
                            links,
                            message_ts,
                        },
                } => {
                    // いつか並列化したいかもしれない
                    let bodies = process_links(&state.kibela_client, links).await;
                    match bodies {
                        Ok(bodies) => {
                            let u = Unfurl {
                                channel,
                                // > The chat.unfurl method requires a token, the message identifier (message_ts), ...
                                // https://api.slack.com/reference/messaging/link-unfurling#attaching_content
                                ts: message_ts,
                                unfurls: bodies,
                            };
                            if let Err(e) = state.slack_client.unfurl(u).await {
                                dbg!(e);
                            };
                        }
                        Err(e) => {
                            dbg!(e);
                        }
                    }

                    Ok("Ok".into())
                }
            }
        });
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
