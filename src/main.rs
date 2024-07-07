use actix_web::{web, App, HttpServer};
use mongodb::{ Client, options::{ClientOptions, ResolverConfig} };
use dotenvy::dotenv;
use std::env;
use tera::Tera;

mod archivos_estaticos;
use archivos_estaticos::leer_archivo_estatico;

mod routes;
use crate::routes::routes::{index, registros, registrar_compra, registrar_venta, 
                            registro_productos, indicadores, indicadores_inversion, 
                            indicadores_ventas, inventario, precio_venta };
mod services;
use crate::services::post::register_products::register_product;
use crate::services::post::register_purchase::register_purchase;
use crate::services::post::register_sale::register_sale;
use crate::services::update::update_price_sale::update_price_sale;
use crate::services::get::inventory::inventory;
use crate::services::get::get_products::products;
use crate::services::get::get_purschases::purchases;
use crate::services::get::get_sales::sales;
use crate::services::get::get_sale_by_id::sale_by_id;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().ok();
    let url = env::var("DB_URL").expect("error");
    let port: u16 = env::var("PORT").expect("error").to_owned().parse().unwrap();

    let options = ClientOptions::parse_with_resolver_config(&url, ResolverConfig::cloudflare())
        .await.expect("Error in options");

    let client = Client::with_options(options).expect("Error create client");

    HttpServer::new(move || {

        let tera: Tera = Tera::new("templates/**/*").unwrap();

        App::new()
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(tera))
            .service(leer_archivo_estatico)
            .service(register_product)
            .service(register_purchase)
            .service(register_sale)
            .service(inventory)
            .service(update_price_sale)
            .service(products)
            .service(purchases)
            .service(sales)
            .service(sale_by_id)
            
            .service(index)
            .service(registros)
            .service(registrar_compra)
            .service(registrar_venta)
            .service(inventario)
            .service(registro_productos)
            .service(precio_venta)
            .service(indicadores)
            .service(indicadores_inversion)
            .service(indicadores_ventas)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
