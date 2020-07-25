// TODO: めっちゃ適当
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
