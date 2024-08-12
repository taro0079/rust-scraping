use scraper::selectable::Selectable;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://news.yahoo.co.jp/topics/top-picks")?.text()?;
    let docuement = scraper::Html::parse_document(&body);
    let title_selector = scraper::Selector::parse("li.newsFeed_item").unwrap();
    let href_selector = scraper::Selector::parse("a.newsFeed_item_link").unwrap();
    let elements = docuement.select(&title_selector);
    for element in elements {
        if let Some(article_element) = element.select(&href_selector).next() {
            let title_selector = scraper::Selector::parse("div.newsFeed_item_title").unwrap();
            let text = article_element
                .select(&title_selector)
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap();
            let link = article_element.value().attr("href").unwrap_or("No link");

            println!("Title: {} \nLink: {}\n", text, link);
        }
    }

    Ok(())
}
