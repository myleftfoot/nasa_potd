use serde_derive::Deserialize;
use serde_json;
use ureq;
use ureq::Error;

const URL: &str = "https://api.nasa.gov/planetary/apod";

#[derive(Deserialize)]
pub struct NasaResponse {
    pub copyright: Option<String>,
    pub explanation: Option<String>,
    pub hdurl: Option<String>,
    pub media_type: Option<String>,
    pub title: Option<String>,
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
    pub fn retrieve_potd_info(&self) -> Result<NasaResponse, Error> {
        let url = format!("{}?api_key={}", self.nasa_url, self.api_key);
        let response = ureq::get(&url).call();
        if !response.ok() {
            return Err(Error::BadStatus);
        }
        let body = response.into_string().unwrap();
        log::debug!("{:#?}", body);
        let nasa_response: NasaResponse = serde_json::from_str(&body).unwrap();

        Ok(nasa_response)
    }
}
