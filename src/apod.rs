use ureq;
use serde_json;
use serde_derive::Deserialize;
const URL: &str = "https://api.nasa.gov/planetary/apod";

#[derive(Deserialize)]
pub struct NasaResponse {
    pub copyright: Option<String>,
    pub explanation: String,
    pub hdurl: Option<String>,
    pub media_type: String,
    pub title: String
}

pub struct Apod {
    pub api_key: String,
    pub nasa_url: String,
}

impl Apod {
    pub fn new(key: String) -> Apod {
        Apod {
            api_key: key,
            nasa_url: String::from(URL),
        }
    }
}

impl Apod {
    pub fn retrieve_picture_url(&self) -> NasaResponse {
        let url = format!("{}?api_key={}", self.nasa_url, self.api_key);
        let response = ureq::get(&url).call();
        let body = response.into_string().unwrap();
        let nasa_response: NasaResponse = serde_json::from_str(&body).unwrap();
        return nasa_response;
    }
}
