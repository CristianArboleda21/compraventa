use actix_web::{web, get, HttpResponse, HttpRequest};
use bson::oid::ObjectId;
use mongodb::{ Database, Collection, bson::{Document, doc} };


#[get("/ventaById/{id}")]
pub async fn sale_by_id(client: web::Data<mongodb::Client>, req: HttpRequest) -> HttpResponse {

    let db: Database = client.database("tienda_online");
    let ventas: Collection<Document> = db.collection("ventas");

    let id_venta: ObjectId = match req.match_info().get("id").unwrap().parse() {
        Ok(id) => { id }
        Err(_) => {
            return HttpResponse::BadRequest().json("Error en el id que envio")
        }
    };
    
    match ventas.find_one(doc! {"_id" : id_venta}, None).await {
        Ok(Some(ventas)) => {
            HttpResponse::Ok().json(ventas)
        }
        Ok(None) => {
            HttpResponse::NotFound().json("Esta venda no existe")
        }
        Err(e) => {
            HttpResponse::InternalServerError().json("Error buscando la venta")
        }
    }

}