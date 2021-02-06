use crabler::*;

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
        // Print all tag info
        if let tag_data = el.children() {
            let tag_tag = &tag_data[0];
            let tag_name = &tag_tag.children()[0];
            let tag_views = &tag_data[1];
            let tag_rooms = &tag_data[2];
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