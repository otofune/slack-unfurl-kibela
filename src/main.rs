use async_std::task;
mod kibela;

fn main() {
    task::block_on(async {
        let client = kibela::client::new(
            std::env::var("SYAKUSI_KIBELA_TEAM_NAME").unwrap(),
            std::env::var("SYAKUSI_KIBELA_ACCESS_TOKEN").unwrap(),
        );
        let note = client.note("/notes/662").await.unwrap();
        let comment = client.comment("629").await.unwrap();
        println!("{:#?}", note);
        println!("{:#?}", comment);
    })
}
