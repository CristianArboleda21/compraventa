use actix_web::{web, get, HttpResponse, HttpRequest};
use bson::oid::ObjectId;
use mongodb::{ Database, Collection, bson::{Document, doc} };


#[get("/ventaById/{id}")]
pub async fn sale_by_id(client: web::Data<mongodb::Client>, req: HttpRequest) -> HttpResponse {

    let db: Database = client.database("tienda_online");
    let sales: Collection<Document> = db.collection("sales");

    let id_sale: ObjectId = match req.match_info().get("id").unwrap().parse() {
        Ok(id) => { id }
        Err(_) => {
            return HttpResponse::BadRequest().json("Error en el id que envio")
        }
    };
    
    match sales.find_one(doc! {"_id" : id_sale}, None).await {
        Ok(Some(sales)) => {
            HttpResponse::Ok().json(sales)
        }
        Ok(None) => {
            HttpResponse::NotFound().json("Esta venta no existe")
        }
        Err(e) => {
            HttpResponse::InternalServerError().json("Error buscando la venta")
        }
    }

}