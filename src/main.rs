use crabler::CrablerError;
use druid::widget::prelude::*;
use druid::widget::{Align, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Lens, WindowDesc, WidgetExt};

mod scrape;

#[derive(Clone, Data, Lens)]
struct Init {
    tag: String,
}


fn build_ui() -> impl Widget<Init> {
    // Indicating tags are being collected
    let title = Label::new("Getting tags...");

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
        .with_child(title)
        .with_child(search);
    
    Align::centered(layout)
}

#[async_std::main]
async fn main() -> Result<(), CrablerError> {
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
    
    scrape::one_scrape().await
}


    