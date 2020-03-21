mod api;
mod tracker;

use actix_web::web;

pub fn configure(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/tracker")
                .service(web::resource("/announce").route(web::get().to(tracker::handle_announce))),
        )
        .service(web::scope("/api"));
}
