use actix_web::{ web, get, HttpResponse };
use mongodb::{ Database, Collection, bson::{Document, doc} };
use futures::TryStreamExt;
use tera::{Context, Tera};

use crate::models::models::Invetory;

#[get("/inventario/productosInventario")]
pub async fn inventory(client: web::Data<mongodb::Client>, tera: web::Data<Tera>) -> HttpResponse {
    
    let db: Database = client.database("tienda_online");
    let products: Collection<Document> = db.collection("products");
    let mut context = Context::new();
    
    let mut list_products = [].to_vec();
    
    match products.find(None, None).await { 
        Ok(mut products) => {
            
            while let Some(result) = products.try_next().await.expect("error") {

                let name: String = match result.get_str("name").unwrap().parse() {
                    Ok(name) => name,
                    Err(e) => {
                        return HttpResponse::InternalServerError().json(e.to_string());
                    }
                };
                
                let code = result.get_i32("code").unwrap();
                let amount = result.get_i32("amount").unwrap();
                let price_sale = result.get_i32("price_sale").unwrap();
                
                let product = Invetory {
                    name,
                    code,
                    amount,
                    price_sale
                };
                
                list_products.push(product);
            }
            
            context.insert("products", &list_products);
            let resp = tera.render("inventario/productos_inventario.html", &context).unwrap();
            HttpResponse::Ok().body(resp)
            
        }
        Err(_) => {
            HttpResponse::InternalServerError().json("Error obteniendo los productos para el inventario")
        }
    }
    
}