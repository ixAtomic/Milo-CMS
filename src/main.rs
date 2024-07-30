use presentation::collection::index;

mod application;
mod presentation;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let endpoints = ["/", "/test"];
    let mut builder = rocket::build();

    for endpoint in endpoints {
        builder = builder.mount(endpoint, routes![index]);
    }
    builder
}
