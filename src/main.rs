#[macro_use] extern crate rocket;

mod endpoints;
mod data;
mod util;

use endpoints::propability_by_month;
use endpoints::year_and_month_with_most_hurricanes;
use data::initializer;


#[launch]
fn rocket() -> _ {
    match initializer::load() {
        Ok(_) => (),
        Err(error) => panic!("Problem while initializing the Application: {:?}", error),
    }

    rocket::build()
        .mount("/hurricanes", routes![
            propability_by_month::execute, 
            year_and_month_with_most_hurricanes::execute,
        ])
}