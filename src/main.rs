#[macro_use] extern crate rocket;

mod endpoints;

use endpoints::most_common_month;
use endpoints::propability_by_month;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/hurricanes", routes![most_common_month::execute, propability_by_month::execute])
}