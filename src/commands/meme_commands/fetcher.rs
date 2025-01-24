use scraper::selectable::Selectable;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Meme {
    pub url: String,
    pub text: String,
    pub send: bool,
}

async fn fetch_page(url: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    Ok(body)
}

fn parse_memes(body: &str) -> Vec<Meme> {
    let document = Html::parse_document(body);
    // println!("{:?}", document);
    let meme_selector = Selector::parse("div.base-unit.clearfix").unwrap();
    let img_selector = Selector::parse("img.base-img").unwrap();
    let mut memes = Vec::new();

    for element in document.select(&meme_selector) {
        // let res = element.select(&img_selector).count();
        if let Some(img_element) = element.select(&img_selector).next() {
            let url = "https:".to_string() + img_element.value().attr("src").unwrap();
            let text = img_element.value().attr("alt").unwrap();
            memes.push(Meme {
                url: url.to_string(),
                text: text.to_string(),
                send: false,
            });
        }
    }

    memes
}

pub async fn fetch_memes(memes_urls: Vec<String>) -> Result<Vec<Meme>, Box<dyn std::error::Error>> {
    let delay = 2;
    let initial_page = 1;
    let last_page = 5;
    let mut memes = Vec::new();

    for meme_url in memes_urls {
        // let source = "https://imgflip.com/meme/162372564/Domino-Effect";
        let source = meme_url;
        for i in initial_page..=last_page {
            let url = format!("{}?page={}", source, i);
            println!("{}", url);
            let body = fetch_page(&url).await?;
            memes.extend(parse_memes(&body));
            std::thread::sleep(std::time::Duration::from_secs(delay));
        }
    }

    Ok(memes)
}
