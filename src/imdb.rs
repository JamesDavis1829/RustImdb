pub mod imdb_scraper {
    use serde_json::Value;
    use serde::Serialize;
    use serde::Deserialize;

    use scraper::Html;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ImdbMovie {
        name: String
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

        let movie = ImdbMovie { name: String::from("Test Movie") };
        Ok(movie)
    }
}