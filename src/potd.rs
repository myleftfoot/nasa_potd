use serde_derive::Deserialize;
use std::fs::File;
use std::io::copy;
use std::io::prelude::*;
use ureq;
use ureq::Error;
use url::Url;

#[derive(Deserialize)]
pub struct Config {
    pub destination_folder: String,
    pub nasa: Nasa,
}

#[derive(Deserialize)]
pub struct Nasa {
    pub api_key: String,
}

pub fn parse_config(config_file: String) -> Config {
    let mut file_content = String::new();
    File::open(&config_file)
        .and_then(|mut f| f.read_to_string(&mut file_content))
        .unwrap();

    toml::from_str(&mut file_content).unwrap()
}

pub async fn download_image(url: String, destination: String) -> Result<(), Error> {
    let u = Url::parse(&url.to_string()).unwrap();
    let response = ureq::get(&url).call();

    if !response.ok() {
        return Err(Error::BadStatus);
    }
    let mut dest = {
        let fname = u
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        let fname = format!("{}/{}", destination, fname);
        File::create(fname)?
    };

    copy(&mut response.into_reader(), &mut dest)?;
    Ok(())
}
