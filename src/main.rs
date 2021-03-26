use crabler::CrablerError;
use druid::Data;

mod scrape;

#[derive(Clone, Data)]

#[async_std::main]
async fn main() -> Result<(), CrablerError> {
    scrape::one_scrape().await
}