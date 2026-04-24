use actix_web::web;

use crate::handler::payment_handler;

pub fn payment_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::resource("/payment")
            .route(web::post().to(payment_handler::create_payment_handler))
            .route(web::get().to(payment_handler::get_all_payments_handler))
          .service(
            web::resource("/{uuid}")
                .route(web::get().to(payment_handler::get_payment_by_uuid_handler))
                .route(web::put().to(payment_handler::update_payment_handler))

                .service(
                    web::resource("/refund")
                       .route(web::patch().to(payment_handler::refund_payment_handler))
        )
          )
    );
}