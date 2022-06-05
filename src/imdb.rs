pub mod imdb_scraper {
    // use rocket::serde::Deserialize;
    // use rocket::serde::Serialize;
    use serde_json::Value;

    #[derive(Debug)]
    pub struct ImdbMovie {
        name: String
    }

    // #[derive(Serialize, Deserialize, Debug)]
    // #[allow(non_snake_case)]
    // pub struct ImdbImageSearchResult {
    //     height: i64,
    //     imageUrl: String,
    //     width: i64
    // }

    // #[derive(Serialize, Deserialize, Debug)]
    // pub struct ImdbSearchResultEntry {
    //     id: String,
    //     l: String,
    //     q: String,
    //     rank: i64,
    //     s: String,
    //     i: ImdbImageSearchResult,
    //     v: Vec<ImdbSearchResultEntry>,
    //     vt: i64,
    //     y: i64
    // }

    // #[derive(Serialize, Deserialize, Debug)]
    // pub struct  ImdbSearchResults {
    //     d: Vec<ImdbImageSearchResult>,
    //     q: String,
    //     v: i64
    // }

    pub async fn search(term: String) -> Result<Value, String> {
        let changed_term = term.replace(" ", "_");
        let url = format!("https://v2.sg.media-imdb.com/suggestion/t/{}.json", changed_term);
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
        let movie = ImdbMovie { name: String::from("Test Movie") };
        Ok(movie)
    }
}