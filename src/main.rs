extern crate crabler;

use crabler::*;
use async_std::future;

#[derive(WebScraper)]
#[on_response(response_handler)]
#[on_html(".tag_row", entry_handler)]

const ENTRY_PREFIX: &'static str = "https://chaturbate.com/tags/";

struct Scraper {}

impl Scraper {
    fn on_response(&self, response: Response) -> Result<()> {
        if let Some(destination) = response.download_destination {
            if response.status == 200 {
                warn!("Finished downloading {} -> {}", response.url, destination);
            }
        }

        Ok(())
    }

    async fn entry_handler(&self, mut response: Response, row: Element) -> Result<()> {
        // What to do with each tag_row entry ?
        // Store in .csv

        Ok(())
    }
}


#[async_std::main]
async fn main() -> Result<()> {
    // Add "?page = {#}" until # = 10


}