mod kibela;
mod slack;
mod types;


use kibela::{
    client::Client,
    url::{parse_url,UrlType as KibelaUrlType},
};
use slack::{
    events::{Event, EventCallback},
    types::{Unfurl, UnfurlBody},
};

fn build_client() -> Client {
    kibela::client::new(
        std::env::var("SYAKUSI_KIBELA_TEAM_NAME").unwrap(),
        std::env::var("SYAKUSI_KIBELA_ACCESS_TOKEN").unwrap(),
    )
}

fn collapse_whitespace(from: &str) -> String {
    let s: Vec<&str> = from.split_whitespace().collect();
    s.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn must_collapse() {
        assert_eq!(collapse_whitespace("aaa\n\naaa"), "aaa aaa")
    }
}

#[derive(Clone)]
pub struct State<T>
where
    T: 'static,
{
    field: T,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();

    let client = build_client();
    let state = State::<Client> { field: client };
    let mut app = tide::with_state(state);
    app.at("/").post(|mut req: tide::Request<State<Client>>| async move {
        let event: Event = req.body_json().await.map_err(|err| {
            dbg!(&err);
            err
        })?;
        let client = &req.state().field;
        match event {
            Event::UrlVerification { challenge } => {
                dbg!(&challenge);
                Ok(challenge)
            }
            Event::EventCallback {
                event: EventCallback::LinkShared { channel, links },
            } => {
                let mut unfurl = Unfurl {
                    channel,
                    ts: "aa".to_string(),
                    unfurls: vec![],
                };
                for link in links {
                    let body = match parse_url(&link.url) {
                        Some(KibelaUrlType::Note { path, comment_id: Some(comment_id) }) => {
                            let note = client.note(path).await;
                            let comment = client.comment(comment_id).await;
                            match (note, comment) {
                                (Ok(note), Ok(comment)) => Ok(UnfurlBody::Attachment {
                                    author_link: comment.author.url.to_string(),
                                    author_name: format!("@{}", comment.author.account),
                                    title: format!("「{}」へのコメント", note.title),
                                    title_link: link.url.to_string(),
                                    // ブラウザーがホワイトスペースを詰める Kibe.la サイトでの挙動にあわせる
                                    text: collapse_whitespace(comment.summary.as_str()),
                                    ts: comment.published_at.timestamp_millis().to_string(),
                                    footer: "Kibela".to_string(),
                                    footer_icon: "https://cdn.kibe.la/assets/shortcut_icon-99b5d6891a0a53624ab74ef26a28079e37c4f953af6ea62396f060d3916df061.png".to_string(),
                                }),
                                // FIXME: 地獄なのでなんとかできないか
                                (Err(e1), Err(e2)) => {
                                    Err(format!("retruns error {}, {}", e1.to_string(), e2.to_string()).into())
                                },
                                (Err(e), _) => {
                                    Err(e)
                                },
                                (_, Err(e)) => {
                                    Err(e)
                                },
                            }
                        },
                        Some(KibelaUrlType::Note { path, comment_id: None }) => {
                            let note = client.note(path).await;
                            match note {
                                Ok(note) =>
                                    Ok(UnfurlBody::Attachment {
                                        author_link: note.author.url.to_string(),
                                        author_name: format!("@{}", note.author.account),
                                        title: note.title,
                                        title_link: link.url.to_string(),
                                        // ブラウザーがホワイトスペースを詰める Kibe.la サイトでの挙動にあわせる
                                        text: collapse_whitespace(note.summary.as_str()),
                                        ts: note.published_at.timestamp_millis().to_string(),
                                        footer: "Kibela".to_string(),
                                        footer_icon: "https://cdn.kibe.la/assets/shortcut_icon-99b5d6891a0a53624ab74ef26a28079e37c4f953af6ea62396f060d3916df061.png".to_string(),
                                    })
                                ,
                                Err(e) => Err(e)
                            }
                        },
                        _ => Err("not match".into()),
                    };
                    match body {
                        Ok(body) => {
                            unfurl.unfurls.push(body);
                        },
                        Err(e) => {
                            dbg!(e);
                        }
                    }
                };
                Ok("Ok".into())
            }
        }
    });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
