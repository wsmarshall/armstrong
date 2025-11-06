use crate::routes::{health_check, subscribe};
use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::PgConnection;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    //wrap connection in smart pointer
    let connection = web::Data::new(connection);
    //capture 'connection' from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            //get a pointer copy and attach it with application state
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
