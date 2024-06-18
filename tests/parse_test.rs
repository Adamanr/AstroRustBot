#[cfg(test)]
mod test {
    use std::fs;
    use scraper::{Html, Selector};
    use tg_astro_bot::parse::parser::{get_element};

    #[tokio::test]
    async fn get_element_p(){
        let fragment = match fs::read_to_string("./tests/testdata/test.html") {
            Ok(html) => Html::parse_fragment(html.as_str()),
            Err(_) => panic!("error get testdata/test.html")
        };

        let div_selector = Selector::parse("div.post-content").expect("error parse div.post-content");
        let div = fragment.select(&div_selector).next().expect("error select fragment");

        let content = get_element(div, "p");
        assert_eq!(content, vec!["Новости!", "Что-то точно будет хорошее!"]);
    }

    #[tokio::test]
    async fn get_element_empty(){
        let fragment = match fs::read_to_string("./tests/testdata/test.html") {
            Ok(html) => Html::parse_fragment(html.as_str()),
            Err(_) => panic!("error get testdata/test.html")
        };

        let div_selector = Selector::parse("div.post-content").expect("error parse div.post-content");
        let div = fragment.select(&div_selector).next().expect("error select fragment");

        let content = get_element(div, "t");
        let t:Vec<&str> = Vec::new();

        assert_eq!(content, t);
    }
}