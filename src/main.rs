use crabler::*;

const ENTRY_PREFIX: &'static str = "https://chaturbate.com/tags/";

#[derive(WebScraper)]
#[on_response(response_handler)]
#[on_html(".tag_row .tag", name_handler)]
#[on_html(".tag_row .viewer", viewer_handler)]
#[on_html(".tag_row .rooms", room_handler)]

struct Scraper {}

impl Scraper {
    // add more comments
    async fn response_handler(&self, response: Response) -> Result<()> {
        println!("Status {}", response.status);
        Ok(())
    }

// WORK HERE
    async fn name_handler(&self, response: Response, tag_name: Element) -> Result<()> {
        if let Some(s) = tag_name.tag("a") {
            //get names
            println!("{}", )

        }

        Ok(())
    }
    async fn viewer_handler(&self, response: Response, tag_views: Element) -> Result<()> {
        if let Some(span) = tag_row.tag("span") {
            //get views
        }

        Ok(())
    }
    async fn room_handler(&self, response: Response, tag_room: Element) -> Result<()> {
        if let Some(rooms) = tag_row.text {
            
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