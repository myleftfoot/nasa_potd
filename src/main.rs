use error_chain::error_chain;
use exitcode;
use notify_rust::Notification;
use std::env;
mod apod;
mod potd;
use env_logger;
use ansi_term::Colour;

error_chain! {
    foreign_links {
        Error(ureq::Error);
    }
}

fn main() {
    env_logger::init();
    let mut home_folder = env::current_dir().unwrap().to_path_buf();
    home_folder.push(".potd");
    home_folder.push("nasa_potd.toml");
    
    let conf = potd::parse_config(home_folder.display().to_string());
    let a = apod::Apod::new(conf.nasa.api_key);
    
    let response = a.retrieve_potd_info();
    match response {
        Ok(r) => {
            let url = r.hdurl.unwrap_or(String::from(""));
            println!("NASA picture of the day.\n{}", url);
            if r.media_type.unwrap_or(String::from("")) == "image" {
                let mut rt = tokio::runtime::Runtime::new().unwrap();
                let future = potd::download_image(url, conf.destination_folder);
                let _r = rt.block_on(future);

                let _x = Notification::new()
                    .summary("Nasa Picture of the Day")
                    .appname("nasa_potd")
                    .body(&r.title.unwrap_or(String::from("").to_string()))
                    .icon("firefox")
                    .show();
            }
        }
        Err(e) => {
            let msg = format!("Error retriving picture of the day. {}", e);
            log::error!("{}", msg);
            eprintln!("{}", Colour::Red.paint(msg));
            std::process::exit(exitcode::UNAVAILABLE);
        }
    }
}
