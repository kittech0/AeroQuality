use crate::app::App;
use crate::error_handler::BoxResult;

mod app;
mod error_handler;

const APP_ID: &str = "io.github.kittech0.airqg";

#[tokio::main]
async fn main() -> BoxResult<()> {
    // Create a new application

    let app = App::new();

    app.run()
}
