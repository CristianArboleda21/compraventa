use actix_web::{ web, post, HttpResponse };
use mongodb::{ Database, Collection, bson::{Document, doc} };
use rand::Rng;

use crate::models::models::{Products, ProductPost};

#[post("/registroProducto")]
pub async fn register_product(client: web::Data<mongodb::Client>, data: web::Json<ProductPost>) -> HttpResponse {
    
    let db: Database = client.database("tienda_online");
    let products: Collection<Document> = db.collection("products");
    
    let mut random = rand::thread_rng();
    let code = random.gen_range(1000..9999);
    
    let new_product = Products {
        name: data.name.clone(),
        code,
        amount: 0,
        price_sale: 0
    };

    let doc_product: Document = bson::to_document(&new_product).unwrap();
    
    match products.find_one(doc! {"nombre" : data.name.clone()}, None).await { 
        Ok(Some(_)) => {
            return HttpResponse::Ok().json("Este producto ya existe")
        }
        Ok(None) => {
            
            match products.insert_one(doc_product, None).await { 
                Ok(_) => {
                    return HttpResponse::Ok().json("Producto creado")
                }
                Err(_) => {
                    return HttpResponse::InternalServerError().json("No se pudo crear el producto")
                }
            }
            
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json("Error encontrando el producto")
        }
    }
    
}