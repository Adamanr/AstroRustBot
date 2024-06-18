pub mod parser {
    use std::ops::Add;
    use chrono::{Datelike, Duration, Local};
    use reqwest::{Error};
    use scraper::{ElementRef, Html, Selector};
    use crate::maps::{MONTH, ZODIAC};

    async fn get_html(zodiac: String, next: bool) -> Result<String, Error> {
        let now = if next {
            Local::now().add(Duration::days(1))
        } else { Local::now()};
        
        let zodiac = ZODIAC.get(zodiac.as_str()).expect("error get zodiac");
        let mth =  MONTH.get(&now.month()).expect("error get month");

        let url = format!("https://astrozodiac.net/goroskop-na-{}-{mth}-{zodiac}", now.day());

        match reqwest::get(url).await {
            Ok(req) => req.text().await,
            Err(err) => Err(err)
        }
    }

    pub async fn parse_html(zodiac: String, next: bool) -> Option<String>{
        let fragment = match get_html(zodiac, next).await {
            Ok(html) => Html::parse_fragment(html.as_str()),
            Err(_) => return None
        };

        let div_selector = Selector::parse("div.post-content").expect("error parse div.post-content");
        let div = fragment.select(&div_selector).next().expect("error select div fragment");

        let contents = get_element(div, "p");
        let titles = get_element(div, "h2");

        let mut result = String::new();
        for i in 0..contents.len() {
            let content = contents.get(i).expect("error get content");

            if i == 0 {
                result.push_str(format!("Общий гороскоп\n{content}\n\n").as_str());
                continue;
            }

            let title = titles.get(i-1).expect("error get title");

            result.push_str(format!("{title}\n{content}\n\n").as_str());
        }

        Some(result)
    }

    pub fn get_element<'a>(div: ElementRef<'a>, tag: &str) -> Vec<&'a str> {
        let p_selector = Selector::parse(tag).expect("error parse selector");

        div.select(&p_selector)
            .flat_map(|e| e.text().collect::<Vec<&str>>())
            .collect::<Vec<&str>>()
    }
}