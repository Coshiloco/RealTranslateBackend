#[macro_use] extern crate rocket;
use once_cell::sync::Lazy;
use std::time::Instant;

static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);



#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
