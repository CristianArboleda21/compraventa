use actix_web::{ web, post, HttpResponse };
use chrono::Utc;
use mongodb::{ Database, Collection, bson::{Document, doc} };
use rand::Rng;

use crate::models::models::{ Sales, SalesPost };

#[post("/registroVenta")]
pub async fn register_sale(client: web::Data<mongodb::Client>, data: web::Json<SalesPost>) -> HttpResponse {

    let db: Database = client.database("tienda_online");
    let sales: Collection<Document> = db.collection("sales");
    let products: Collection<Document> = db.collection("products");

    let mut random = rand::thread_rng();
    let code_sale = random.gen_range(1000..9999);

    let new_sale = Sales {
        products: data.products.clone(),
        code_sale,
        date_sale: bson::DateTime::from_chrono(Utc::now()),
        total_sale: data.total_sale
    };

    let doc_sales = bson::to_document(&new_sale).unwrap();
    
    let mut subtract: i32 = 0;
    
    for prod in data.products.clone() {
        
        let product = match products.find_one(doc! {"name" : prod.name}, None).await {
            Ok(Some(doc)) => { doc }
            Ok(None) => {
                return HttpResponse::InternalServerError().json("Producto no encontrado");
            }
            Err(e) => {
                return HttpResponse::InternalServerError().json(e.to_string());
            }
        };

        let amount_inventory = product.get_i32("amount").unwrap();
        
        subtract = amount_inventory - prod.amount; 
        
    }
    
    if subtract >= 0 {

        match sales.insert_one(doc_sales, None).await {
            Ok(result) => {

                let id_sale = match result.inserted_id.as_object_id() {
                    Some(id) => id,
                    None => {
                        return HttpResponse::InternalServerError().json("Error obteniendo el id de la venta");
                    }
                };

                let sale = match sales.find_one(doc! {"_id" : id_sale}, None).await {
                    Ok(Some(doc)) => { doc }
                    Ok(None) => {
                        return HttpResponse::InternalServerError().json("Venta no encontrado");
                    }
                    Err(e) => {
                        return HttpResponse::InternalServerError().json(e.to_string());
                    }
                };

                let list_product = match sale.get_array("products") {
                    Ok(array) => { array }
                    Err(e) => {
                        return HttpResponse::InternalServerError().json(e.to_string());
                    }
                };

                for product in list_product {

                    let doc_product = product.as_document().unwrap();

                    let name = doc_product.get_str("name").unwrap();
                    let amount_sale = doc_product.get_i32("amount").unwrap();

                    let prod = match products.find_one(doc! {"name" : name}, None).await {
                        Ok(Some(product)) => { product }
                        Ok(None) => {
                            return HttpResponse::InternalServerError().json("Producto no encontrado");
                        }
                        Err(e) => {
                            return HttpResponse::InternalServerError().json(e.to_string());
                        }
                    };

                    let amount_inventory = prod.get_i32("amount").unwrap();
                    let new_amount_inventory = amount_inventory - amount_sale;

                    let _ = products.update_one(
                        doc! {"name" : name},
                        doc! {"$set" : { "amount" : new_amount_inventory } },
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