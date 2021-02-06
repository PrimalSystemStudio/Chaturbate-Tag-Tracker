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
        if let Some(tag_data) = el.children() {
            let tag_name = tag_data.first().text();
            print("{}", tag_name)
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