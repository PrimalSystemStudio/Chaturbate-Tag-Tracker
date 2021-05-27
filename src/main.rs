use druid::widget::prelude::*;
use druid::widget::{Align, Flex, Label, TextBox};
use druid::{AppLauncher, Application, Data, Lens, WindowDesc, WidgetExt};
use tokio::time::{self, Duration};
use crabler::*;
use std::fs;
use crabler::*;
use chrono::prelude::*;
use pyo3::prelude::*;
use tokio::time::{self, Duration};

mod scrape;

#[derive(Clone, Data, Lens)]
struct Init {
    tag: String,
}


fn build_ui() -> impl Widget<Init> {
    // Search box
    let l_search = Label::new("Search: ");
    let tb_search = TextBox::new()
        .with_placeholder("Enter tag to search")
        .lens(Init::tag);
    let search = Flex::row()
        .with_child(l_search)
        .with_child(tb_search);

    // Describe layout of UI
    let layout = Flex::column()
        .with_child(search);
    
    Align::centered(layout)
}

#[tokio::main]
async fn main() -> Result<()> {

    // Describe the main window
    let main_window = WindowDesc::new(build_ui())
        .title("Chaturbate Tag Tracker")
        .window_size((400.0, 400.0));

    // Create starting app state
    let init_state = Init {
        tag: String::from("#"),
    };

    // Start application
    AppLauncher::with_window(main_window)
        .launch(init_state)
        .expect("Failed to launch application");
    
    // Run the scraper while application is open
    while Application::try_global().is_some() {
        // Wait for the application to start
        let mut wait = time::interval(Duration::from_secs(15));
        let mut scrape_interval = time::interval(Duration::from_secs(3600));
        wait.tick().await;
        wait.tick().await;

        println!("Scraper starting");
        // Run the scraper to get info from chaturbate.com/tags
        let scraper = scrape::Scraper {};
        scraper.run(Opts::new().with_urls(scrape::ENTRY_PREFIX.to_vec()).with_threads(10)).await?;
        scrape_interval.tick().await;
    }

}
    