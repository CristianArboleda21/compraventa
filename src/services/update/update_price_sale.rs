use actix_web::{ web, put, HttpResponse, HttpRequest };
use mongodb::{ Database, Collection, bson::{Document, doc} };
use bson::oid::ObjectId;

use crate::models::models::ActualizacionPrecioVenta;

#[put("/actualizarPrecioVenta/{id}")]
pub async fn update_price_sale(client: web::Data<mongodb::Client>, req: HttpRequest, data: web::Json<ActualizacionPrecioVenta>) -> HttpResponse {
    
    let db: Database = client.database("tienda_online");
    let productos: Collection<Document> = db.collection("productos");

    let id_producto: ObjectId = match req.match_info().get("id").unwrap().parse() {
        Ok(id) => { id }
        Err(_) => {
            return HttpResponse::BadRequest().json("Error obteniendo el id del producto")
        }
    };
    
    match productos.update_one(
        doc! { "_id" : id_producto },
        doc! { "$set" : { "precio_venta" : data.precio_venta } },
        None
    ).await {
        Ok(result) => {

            if result.modified_count == 0 {
                HttpResponse::NotModified().json("Precio de venta del producto no fue modificado")
            } else {
                HttpResponse::Ok().json("Precio de venta del producto actualizado")
            }

        }
        Err(e) => {
            HttpResponse::BadRequest().json("Error al actualizar precio de venta del producto")
        }
    }
    
}