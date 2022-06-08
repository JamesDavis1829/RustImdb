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
        rating: f32,
        plot: String
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ImdbSearchResultImage {
        height: Option<i64>,
        imageUrl: String,
        width: Option<i64>
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ImdbSearchResultEntry{
        i: ImdbSearchResultImage,
        id: String,
        l: String,
        s: String,
        q: Option<String>,
        rank: Option<i32>
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ImdbSearchResults {
        d: Vec<ImdbSearchResultEntry>
    }

    pub async fn search(term: String) -> Result<ImdbSearchResults, String> {
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

        let parsed = match resp.json::<ImdbSearchResults>().await {
            Ok(results) => results,
            Err(e) => {
                println!("{:?}", e);
                return Err(String::from("Parse failure."))
            }
        };

        //It appears that movies begin with IDs that have tt so filter so it's only movies and not like a "Top 10"
        let filtered_results: Vec<ImdbSearchResultEntry> = parsed.d.into_iter().filter(|val| val.id.starts_with("tt")).collect();

        Ok(ImdbSearchResults {
            d: filtered_results
        })
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

        let title_selector = create_selector("[data-testid=\"hero-title-block__title\"]");
        let title = match document.select(&title_selector).next() {
            Some(item) => item.inner_html(),
            None => String::from(""),
        };

        let plot_selector = create_selector("[data-testid=\"plot-l\"]");
        let plot = match document.select(&plot_selector).next() {
            Some(item) => item.inner_html(),
            None => String::from(""),
        };

        let rating_selector = create_selector("[data-testid=\"hero-rating-bar__aggregate-rating__score\"] > span");
        let rating = match document.select(&rating_selector).next() {
            Some(item) => item.inner_html(),
            None => String::from("")
        };

        let rating = match rating.parse::<f32>() {
            Ok(num) => num,
            Err(_) => -1.0
        };

        ImdbMovie { 
            name: title,
            rating: rating,
            plot: plot
        }
    }

    fn create_selector(pattern: &str) -> Selector {
        return match Selector::parse(pattern) {
            Ok(s) => s,
            Err(_) => Selector { selectors: SmallVec::from_vec(vec! [])}
        }
    }
}