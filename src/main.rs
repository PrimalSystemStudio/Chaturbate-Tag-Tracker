use async_std::task;
use crabler::*;
use chrono::prelude::*;
use pyo3::prelude::*;
use std::fs;
use std::time::Duration;

const ENTRY_PREFIX: [&'static str; 10] = [
    "https://chaturbate.com/tags/?page=1",
    "https://chaturbate.com/tags/?page=2",
    "https://chaturbate.com/tags/?page=3",
    "https://chaturbate.com/tags/?page=4",
    "https://chaturbate.com/tags/?page=5",
    "https://chaturbate.com/tags/?page=6",
    "https://chaturbate.com/tags/?page=7",
    "https://chaturbate.com/tags/?page=8",
    "https://chaturbate.com/tags/?page=9",
    "https://chaturbate.com/tags/?page=10"];

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

    async fn tag_handler(&self, _: Response, el: Element) -> Result<()> {
        // Get all elements nested in .tag_row
        let tag_data = el.children();
        let tag_tag = &tag_data[0];

        // Get tag name, views and rooms 
        if let Some(tag_name) = tag_tag.children()[0].text() {
            if let Some(tag_views) = tag_data[1].text() {
                if let Some(tag_rooms) = tag_data[2].text() {
                    // Get current time
                    let utcnow = Utc::now();
                    let date = utcnow.format("%Y-%m-%d").to_string();
                    let time = utcnow.format("%H:%M").to_string();

                    // println!("Tag: {} has {} viewers and {} rooms on {} at {}", tag_name, tag_views, tag_rooms, date, time);
                    
                    
                    let tag_info = (tag_name, tag_views.parse().unwrap(), tag_rooms.parse().unwrap(), date, time);

                    // Pass tag data to algorithms.py for addition into the database
                    let success = py_data(tag_info);
                    println!("{:?}", success);
                    Ok(())
                } else {
                    Err(CrablerError::BodyParsing("Error, no rooms".to_string()))
                }
            } else {
                Err(CrablerError::BodyParsing("Error, no viewers".to_string()))
            }
        } else {
            Err(CrablerError::BodyParsing("Error, no name".to_string()))
        }
    }
}

// Use algorithms.py to store the data in sqlite file
fn py_data(data: (String, String, String, String, String)) -> PyResult<()> {
    // Turn algorithms file to string
    let al_string = fs::read_to_string("src/algorithms.py")
        .expect("Error reading file.");

    // Pass data to algorithms program
    Python::with_gil(|py| {
        let algorithm = PyModule::from_code(py, &al_string, "algorithms.py", "algorithms")?;
        let _ = algorithm.call1("add", data)?;
        Ok(())
    })

}

#[async_std::main]
// Run scraper to get info from chaturbate.com/tags
pub async fn main() -> Result<()> {
    loop {
        let start = Utc::now().format("%H:%M:%S").to_string();
        println!("Started at {}", start);
        let scraper = Scraper {};
        scraper.run(Opts::new().with_urls(ENTRY_PREFIX.to_vec()).with_threads(10)).await?;
        let end = Utc::now().format("%H:%M:%S").to_string();
        println!("Ended at {}", end);
        task::sleep(Duration::from_secs(3600)).await;
        let restart = Utc::now().format("%H:%M:%S").to_string();
        println!("Restarted at {}", restart);
    }
}

