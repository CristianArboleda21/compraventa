use actix_web::{ web, post, HttpResponse };
use chrono::Utc;
use mongodb::{ Database, Collection, bson::{Document, doc} };
use rand::Rng;

use crate::models::models::{ Ventas, VentasPost };
use crate::services::get::get_products::products;

#[post("/registroVenta")]
pub async fn register_sale(client: web::Data<mongodb::Client>, data: web::Json<VentasPost>) -> HttpResponse {

    let db: Database = client.database("tienda_online");
    let ventas: Collection<Document> = db.collection("ventas");
    let productos: Collection<Document> = db.collection("productos");

    let mut random = rand::thread_rng();
    let codigo_venta = random.gen_range(1000..9999);

    let new_venta = Ventas {
        productos: data.productos.clone(),
        codigo_venta,
        fecha_venta: bson::DateTime::from_chrono(Utc::now()),
        total_venta: data.total_venta
    };

    let doc_ventas = bson::to_document(&new_venta).unwrap();
    
    let mut resta: i32 = 0;
    
    for prod in data.productos.clone() {
        
        let producto = match productos.find_one(doc! {"nombre" : prod.nombre}, None).await {
            Ok(Some(doc)) => { doc }
            Ok(None) => {
                return HttpResponse::InternalServerError().json("Producto no encontrado");
            }
            Err(e) => {
                return HttpResponse::InternalServerError().json(e.to_string());
            }
        };

        let cantidad_inventario = producto.get_i32("cantidad").unwrap();
        
        resta = cantidad_inventario - prod.cantidad; 
        
    }
    
    if resta >= 0 {

        match ventas.insert_one(doc_ventas, None).await {
            Ok(result) => {

                let id_sale = match result.inserted_id.as_object_id() {
                    Some(id) => id,
                    None => {
                        return HttpResponse::InternalServerError().json("Error obteniendo el id de la venta");
                    }
                };

                let sale = match ventas.find_one(doc! {"_id" : id_sale}, None).await {
                    Ok(Some(doc)) => { doc }
                    Ok(None) => {
                        return HttpResponse::InternalServerError().json("Venta no encontrado");
                    }
                    Err(e) => {
                        return HttpResponse::InternalServerError().json(e.to_string());
                    }
                };

                let list_product = match sale.get_array("productos") {
                    Ok(array) => { array }
                    Err(e) => {
                        return HttpResponse::InternalServerError().json(e.to_string());
                    }
                };

                for product in list_product {

                    let doc_producto = product.as_document().unwrap();

                    let nombre = doc_producto.get_str("nombre").unwrap();
                    let cantidad_venta = doc_producto.get_i32("cantidad").unwrap();

                    let prod = match productos.find_one(doc! {"nombre" : nombre}, None).await {
                        Ok(Some(product)) => { product }
                        Ok(None) => {
                            return HttpResponse::InternalServerError().json("Producto no encontrado");
                        }
                        Err(e) => {
                            return HttpResponse::InternalServerError().json(e.to_string());
                        }
                    };

                    let cantidad_inventario = prod.get_i32("cantidad").unwrap();
                    let new_cantidad_inventario = cantidad_inventario - cantidad_venta;

                    let _ = productos.update_one(
                        doc! {"nombre" : nombre},
                        doc! {"$set" : { "cantidad" : new_cantidad_inventario } },
                        None
                    ).await;

                }

                HttpResponse::Ok().json("Venta creada")
            }
            Err(e) => {
                HttpResponse::InternalServerError().json(e.to_string())
            }
        }
        
    } else { 
        
        HttpResponse::BadRequest().json("Excede la cantidad del inventario")
        
    }
    
    

}