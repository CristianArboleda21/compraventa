use actix_web::{ web, put, HttpResponse, HttpRequest };
use mongodb::{ Database, Collection, bson::{Document, doc} };
use bson::oid::ObjectId;

use crate::models::models::UpdatePriceSale;

#[put("/actualizarPrecioVenta/{id}")]
pub async fn update_price_sale(client: web::Data<mongodb::Client>, req: HttpRequest, data: web::Json<UpdatePriceSale>) -> HttpResponse {
    
    let db: Database = client.database("tienda_online");
    let products: Collection<Document> = db.collection("products");

    let id_product: ObjectId = match req.match_info().get("id").unwrap().parse() {
        Ok(id) => { id }
        Err(_) => {
            return HttpResponse::BadRequest().json("Error obteniendo el id del producto")
        }
    };
    
    match products.update_one(
        doc! { "_id" : id_product },
        doc! { "$set" : { "price_sale" : data.price_sale } },
        None
    ).await {
        Ok(result) => {

            if result.modified_count == 0 {
                HttpResponse::NotModified().json("Precio de venta del producto no fue modificado")
            } else {
                HttpResponse::Ok().json("Precio de venta del producto actualizado")
            }

        }
        Err(_) => {
            HttpResponse::BadRequest().json("Error al actualizar precio de venta del producto")
        }
    }
    
}