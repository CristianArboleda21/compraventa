use actix_web::{ web, get, HttpResponse };
use mongodb::{ Database, Collection, bson::{Document, doc} };
use futures::TryStreamExt;


#[get("/Productos")]
pub async fn products(client: web::Data<mongodb::Client>) -> HttpResponse {

    let db: Database = client.database("tienda_online");
    let productos: Collection<Document> = db.collection("productos");

    let mut lista_productos = [].to_vec();

    match productos.find(None, None).await {
        Ok(mut productos) => {

            while let Some(result) = productos.try_next().await.expect("error") {
                lista_productos.push(result)
            }
            
            HttpResponse::Ok().json(lista_productos)

        }
        Err(e) => {
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }

}