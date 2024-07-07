use actix_web::{ web, post, HttpResponse };
use mongodb::{ Database, Collection, bson::{Document, doc} };
use chrono::Utc;
use rand::Rng;

use crate::models::models::{Purchase, PurchasePost, Products};

#[post("/registroCompra")]
pub async fn register_purchase(client: web::Data<mongodb::Client>, data: web::Json<PurchasePost>) -> HttpResponse {

    let db: Database = client.database("tienda_online");
    let products: Collection<Document> = db.collection("products");
    let purchases: Collection<Document> = db.collection("purchases");
    
    let mut random = rand::thread_rng();
    let code_random = random.gen_range(1000..9999);

    match products.find_one(doc! {"nombre" : data.name.clone()}, None).await {
        Ok(Some(result)) => {

            let name: String = match result.get_str("name").unwrap().parse() {
                Ok(name) => name,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(e.to_string());
                }
            };
            
            let code = result.get_i32("code").unwrap();
            let amount = result.get_i32("amount").unwrap();
            let sum_amount = amount + data.amount;

            let _ = products.update_one(
                doc! {"name" : data.name.clone()},
                doc! { "$set" : { "amount" : sum_amount } },
                None
            ).await;

            let total_purchase = data.price_unit * data.amount;

            let new_purchase = Purchase {
                name,
                code,
                amount: data.amount,
                date_purchase: bson::DateTime::from_chrono(Utc::now()),
                price_unit: data.price_unit,
                total_purchase,
            };

            let doc_purchase: Document = bson::to_document(&new_purchase).unwrap();

            let _ = purchases.insert_one(doc_purchase, None).await;

            HttpResponse::Ok().json("Se actualizo la amount de products en el inventario y se registro la compra")
        }
        Ok(None) => {

            let new_product = Products {
                name: data.name.clone(),
                code: code_random,
                amount: 0,
                price_sale: 0
            };

            let doc_product: Document = bson::to_document(&new_product).unwrap();
            
            match products.insert_one(doc_product, None).await {
                Ok(result) => {
                    
                    let id_product = match result.inserted_id.as_object_id() {
                        Some(id) => id,
                        None => {
                            return HttpResponse::InternalServerError().json("Error obteniendo el id del producto");
                        }
                    };

                    let product = match products.find_one(doc! {"_id" : id_product}, None).await {
                        Ok(Some(doc)) => { doc }
                        Ok(None) => {
                            return HttpResponse::InternalServerError().json("Producto no encontrado");
                        }
                        Err(e) => {
                            return HttpResponse::InternalServerError().json(e.to_string());
                        }
                    };
                    
                    let name: String = match product.get_str("name").unwrap().parse() {
                        Ok(name) => name,
                        Err(e) => {
                            return HttpResponse::InternalServerError().json(e.to_string());
                        }
                    };

                    let code = product.get_i32("code").unwrap();

                    let amount = product.get_i32("amount").unwrap();
                    let sum_amount = amount + data.amount;

                    let _ = products.update_one(
                        doc! {"_id" : id_product },
                        doc! { "$set" : { "amount" : sum_amount } },
                        None
                    ).await;

                    let total_purchase = data.price_unit * data.amount;

                    let new_purchase = Purchase {
                        name,
                        code,
                        amount: data.amount,
                        date_purchase: bson::DateTime::from_chrono(Utc::now()),
                        price_unit: data.price_unit,
                        total_purchase,
                    };

                    let doc_purchase: Document = bson::to_document(&new_purchase).unwrap();

                    let _ = purchases.insert_one(doc_purchase, None).await;
                    
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