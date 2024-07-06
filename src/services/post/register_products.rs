use actix_web::{ web, post, HttpResponse };
use mongodb::{ Database, Collection, bson::{Document, doc} };
use rand::Rng;

use crate::models::models::{Productos, ProductoPost};

#[post("/RegistroProducto")]
pub async fn register_product(client: web::Data<mongodb::Client>, data: web::Json<ProductoPost>) -> HttpResponse {
    
    let db: Database = client.database("tienda_online");
    let productos: Collection<Document> = db.collection("productos");
    
    let mut random = rand::thread_rng();
    let codigo = random.gen_range(1000..9999);
    
    let new_product = Productos {
        nombre: data.nombre.clone(),
        codigo,
        cantidad: 0,
        precio_venta: 0
    };

    let doc_product: Document = bson::to_document(&new_product).unwrap();
    
    match productos.find_one(doc! {"nombre" : data.nombre.clone()}, None).await { 
        Ok(Some(_)) => {
            return HttpResponse::Ok().json("Este producto ya existe")
        }
        Ok(None) => {
            
            match productos.insert_one(doc_product, None).await { 
                Ok(_) => {
                    return HttpResponse::Ok().json("Producto creado")
                }
                Err(_) => {
                    return HttpResponse::InternalServerError().json("No se pudo crear el producto")
                }
            }
            
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json("Error encontrando el producto")
        }
    }
    
}