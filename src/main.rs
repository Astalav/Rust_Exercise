#[macro_use] extern crate rocket;

mod endpoints;
mod data;
mod util;

use endpoints::most_common_month;
use endpoints::propability_by_month;
use data::initializer;


#[launch]
fn rocket() -> _ {
    match initializer::load() {
        Ok(_) => (),
        Err(error) => panic!("Problem while initializing the Application: {:?}", error),
    }

    rocket::build()
        .mount("/hurricanes", routes![most_common_month::execute, propability_by_month::execute])
}