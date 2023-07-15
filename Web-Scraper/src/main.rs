use scraper::{Html, Selector};
use reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // make a GET request
    let resp = reqwest::blocking::get("http://example.com")?;
    // get the HTML from the response
    let body = resp.text()?;

    // parse the body as HTML
    let parsed_html = Html::parse_document(&body);

    // create a selector for `h1` tags
    let h1_selector = Selector::parse("h1").unwrap();

    // loop over each `h1` tag in the HTML
    for element in parsed_html.select(&h1_selector) {
        let h1_text = element.text().collect::<Vec<_>>();
        println!("{:?}", h1_text);
    }

    Ok(())
}
