#[macro_use]
extern crate rocket;
use rocket::figment::Figment;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let figment = Figment::from(rocket::Config::default()).merge((
        "port",
        std::env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .as_str()
            .parse::<u16>()
            .expect("invalid port supplied"),
    ));
    rocket::custom(figment).mount("/", routes![index])
}
