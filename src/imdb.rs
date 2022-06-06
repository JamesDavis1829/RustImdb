pub mod imdb_scraper {
    use rocket::http::private::SmallVec;
    use serde_json::Value;
    use serde::Serialize;
    use serde::Deserialize;

    use scraper::Html;
    use scraper::Selector;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ImdbMovie {
        name: String,
        rating: i64,
        plot: String
    }

    pub async fn search(term: String) -> Result<Value, String> {
        let changed_term = term.replace(" ", "_");
        
        let first_term = match term.chars().nth(0) {
            Some(c) => c,
            None => {
                return Err(String::from("Invalid search term"))
            }
        };

        let url = format!("https://v2.sg.media-imdb.com/suggestion/{}/{}.json",first_term, changed_term);
        let resp = reqwest::get(url).await;
        let resp = match resp {
            Ok(res) => res,
            Err(e) => {
                println!("{:?}", e);
                return Err(String::from("Url failure."))
            }
        };

        let parsed = match resp.json::<Value>().await {
            Ok(has_map) => has_map,
            Err(e) => {
                println!("{:?}", e);
                return Err(String::from("Parse failure."))
            }
        };

        Ok(parsed)
    }

    pub async fn get_movie_data(movie_id: String) -> Result<ImdbMovie, String> {
        let resp = reqwest::get(format!("https://www.imdb.com/title/{}/", movie_id)).await;

        let resp = match resp {
            Ok(res) => res,
            Err(e) => {
                println!("{:?}", e);
                return Err(String::from("Could not find that movie."))
            }
        };

        let resp_text = match resp.text().await {
            Ok(text) => text,
            Err(e) => {
                println!("{:?}", e);
                return Err(String::from("Could not parse the IMDB response"))
            }
        };

        let movie = convert_imdb_html_to_movie(resp_text);
        Ok(movie)
    }

    fn convert_imdb_html_to_movie(html_document: String) -> ImdbMovie {
        let document = Html::parse_document(html_document.as_str());

        let title_selector = match Selector::parse("[data-testid=\"hero-title-block__title\"]") {
            Ok(s) => s,
            Err(_) => Selector { selectors: SmallVec::from_vec(vec! [])} 
        };

        let title = match document.select(&title_selector).next() {
            Some(item) => item.inner_html(),
            None => String::from(""),
        };

        let plot_selector = match Selector::parse("[data-testid=\"plot-l\"]") {
            Ok(s) => s,
            Err(_) => Selector { selectors: SmallVec::from_vec(vec! [])} 
        };

        let plot = match document.select(&plot_selector).next() {
            Some(item) => item.inner_html(),
            None => String::from(""),
        };

        ImdbMovie { 
            name: title,
            rating: 0,
            plot: plot
        }
    }
}