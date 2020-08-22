
mod potd;
mod apod;

fn main() {
    let conf = potd::parse_config(String::from("/Users/strotti/.potd/nasa_potd.toml"));
    let a = apod::Apod::new(conf.nasa.api_key);
    let response = a.retrieve_picture_url();
    println!("Hello, world! {}!", response.hdurl.unwrap());
}
