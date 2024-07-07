use actix_web::{ web, post, HttpResponse };
use mongodb::{ Database, Collection, bson::{Document, doc} };
use chrono::Utc;
use rand::Rng;

use crate::models::models::{Compras, ComprasPost, Productos};

#[post("/registroCompra")]
pub async fn register_purchase(client: web::Data<mongodb::Client>, data: web::Json<ComprasPost>) -> HttpResponse {

    let db: Database = client.database("tienda_online");
    let productos: Collection<Document> = db.collection("productos");
    let compras: Collection<Document> = db.collection("compras");
    
    let mut random = rand::thread_rng();
    let codigo_random = random.gen_range(1000..9999);

    match productos.find_one(doc! {"nombre" : data.nombre.clone()}, None).await {
        Ok(Some(result)) => {

            let nombre: String = match result.get_str("nombre").unwrap().parse() {
                Ok(nombre) => nombre,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(e.to_string());
                }
            };
            
            let codigo = result.get_i32("codigo").unwrap();
            let cantidad = result.get_i32("cantidad").unwrap();
            let suma_cantidad = cantidad + data.cantidad;

            let _ = productos.update_one(
                doc! {"nombre" : data.nombre.clone()},
                doc! { "$set" : { "cantidad" : suma_cantidad } },
                None
            ).await;

            let total_compra = data.precio_unidad * data.cantidad;

            let new_compras = Compras {
                nombre,
                codigo,
                cantidad: data.cantidad,
                fecha_compra: bson::DateTime::from_chrono(Utc::now()),
                precio_unidad: data.precio_unidad,
                total_compra,
            };

            let doc_compras: Document = bson::to_document(&new_compras).unwrap();

            let _ = compras.insert_one(doc_compras, None).await;

            HttpResponse::Ok().json("Se actualizo la cantidad de productos en el inventario y se registro la compra")
        }
        Ok(None) => {

            let new_product = Productos {
                nombre: data.nombre.clone(),
                codigo: codigo_random,
                cantidad: 0,
                precio_venta: 0
            };

            let doc_product: Document = bson::to_document(&new_product).unwrap();
            
            match productos.insert_one(doc_product, None).await {
                Ok(result) => {
                    
                    let id_product = match result.inserted_id.as_object_id() {
                        Some(id) => id,
                        None => {
                            return HttpResponse::InternalServerError().json("Error obteniendo el id del producto");
                        }
                    };

                    let product = match productos.find_one(doc! {"_id" : id_product}, None).await {
                        Ok(Some(doc)) => { doc }
                        Ok(None) => {
                            return HttpResponse::InternalServerError().json("Producto no encontrado");
                        }
                        Err(e) => {
                            return HttpResponse::InternalServerError().json(e.to_string());
                        }
                    };
                    
                    let nombre: String = match product.get_str("nombre").unwrap().parse() {
                        Ok(nombre) => nombre,
                        Err(e) => {
                            return HttpResponse::InternalServerError().json(e.to_string());
                        }
                    };

                    let codigo = product.get_i32("codigo").unwrap();

                    let cantidad = product.get_i32("cantidad").unwrap();
                    let suma_cantidad = cantidad + data.cantidad;

                    let _ = productos.update_one(
                        doc! {"_id" : id_product },
                        doc! { "$set" : { "cantidad" : suma_cantidad } },
                        None
                    ).await;

                    let total_compra = data.precio_unidad * data.cantidad;

                    let new_compras = Compras {
                        nombre,
                        codigo,
                        cantidad: data.cantidad,
                        fecha_compra: bson::DateTime::from_chrono(Utc::now()),
                        precio_unidad: data.precio_unidad,
                        total_compra,
                    };

                    let doc_compras: Document = bson::to_document(&new_compras).unwrap();

                    let _ = compras.insert_one(doc_compras, None).await;
                    
                    HttpResponse::Ok().json("Producto creado y compra registrada")
                }
                Err(e) => {
                    HttpResponse::InternalServerError().json(e.to_string())
                }
            }

        }
        Err(e) => {
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }

}