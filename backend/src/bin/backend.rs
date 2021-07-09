use diana_actix_web::{
    actix_web::{App, HttpServer},
    create_graphql_server,
};
use backend::get_opts;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configurer = create_graphql_server(get_opts()).expect("Failed to set up configurer!");

    HttpServer::new(move || App::new().configure(configurer.clone()))
        .bind("0.0.0.0:9000")?
        .run()
        .await
}
