//Upload to Github after consulting Vi on a name

use std::fs;
use crabler::*;
use chrono::prelude::*;
use pyo3::prelude::*;

const ENTRY_PREFIX: &'static str = "https://chaturbate.com/tags/";

// Use WebScraper trait to get each item with the ".tag_row" class
#[derive(WebScraper)]
#[on_response(response_handler)]
#[on_html(".tag_row", tag_handler)]

struct Scraper {}

impl Scraper {
    // Print webpage status
    async fn response_handler(&self, response: Response) -> Result<()> {
        println!("Status {}", response.status);
        Ok(())
    }

    async fn tag_handler(&self, response: Response, el: Element) -> Result<()> {
        // Import algorithms.py
        let gil = Python::acquire_gil();
        let py = gil.python();
        let al_string = fs::read_to_string("algorithms.py")
            .expect("Error reading file.");
        let algorithms = PyModule::from_code(py, &al_string, "algorithms.py", "algorithms");

        // Define tag parameters
        let tag_data = el.children();
        let tag_tag = &tag_data[0];
        if let Some(tag_name) = tag_tag.children()[0].text() {
            if let Some(tag_views) = tag_data[1].text() {
                if let Some(tag_rooms) = &tag_data[2].text() {
                    // Get current time
                    let datetime = Utc::now().to_string();
                    
                    // Pass tag data to algorithms(add)
                    let success = algorithms.call(py, "add", (tag_name, tag_views, tag_rooms, datetime), None).unwrap();
                    println!("{:?}", success)
                }
            }
        }

        Ok(())
    }
}


#[async_std::main]
async fn main() -> Result<()> {
    // Add "?page=#" until # = 10
    let scraper = Scraper {};
    scraper.run(Opts::new().with_urls(vec![ENTRY_PREFIX])).await
}

