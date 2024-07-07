use actix_web::{ web, get, HttpResponse };
use mongodb::{ Database, Collection, bson::{Document, doc} };
use futures::TryStreamExt;


#[get("/productos")]
pub async fn products(client: web::Data<mongodb::Client>) -> HttpResponse {

    let db: Database = client.database("tienda_online");
    let products: Collection<Document> = db.collection("products");

    let mut lista_products = [].to_vec();

    match products.find(None, None).await {
        Ok(mut products) => {

            while let Some(result) = products.try_next().await.expect("error") {
                lista_products.push(result)
            }
            
            HttpResponse::Ok().json(lista_products)

        }
        Err(_) => {
            HttpResponse::InternalServerError().json("Error obteniendo los productos")
        }
    }

}