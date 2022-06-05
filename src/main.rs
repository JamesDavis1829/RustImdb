#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};

mod imdb;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/imdb/search/<term>")]
async fn get_imdb_search(term: String) -> String {
    let results = imdb::imdb_scraper::search(term.clone()).await;

    match results {
        Ok(parsed) => return format!("{:?}", parsed),
        Err(e) => return format!("Error {:?}", e)
    }
}

#[get("/imdb/movie/<movie_id>")]
async fn get_imdb_movie(movie_id: String) -> String {
    let results = imdb::imdb_scraper::get_movie_data(movie_id).await;
    match results {
        Ok(movie) => return format!("{:?}", movie),
        Err(e) => return format!("Error {:?}", e)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index, 
        delay, 
        get_imdb_search,
        get_imdb_movie
    ])
}