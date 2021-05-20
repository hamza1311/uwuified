#[macro_use]
extern crate rocket;
use rocket::figment::Figment;
use std::net::IpAddr;
use rocket::response::NamedFile;
use std::io;

#[get("/uwu?<q>")]
pub fn uwuify(q: &str) -> String {
    uwuifier::uwuify_str_sse(q)
}

#[get("/")]
pub async fn index() -> Result<NamedFile, io::Error> {
    NamedFile::open("static/index.html").await
}

#[launch]
fn rocket() -> _ {
    let figment = Figment::from(rocket::Config::default())
        .merge((
            "port",
            std::env::var("PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .as_str()
                .parse::<u16>()
                .expect("invalid port supplied"),
        ))
        .merge((
            "address",
            std::env::var("HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string())
                .as_str()
                .parse::<IpAddr>()
                .expect("invalid host supplied"),
        ));
    rocket::custom(figment)
        .mount("/", routes![index])
        .mount("/api", routes![uwuify])
}
