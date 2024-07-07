use mongodb::{ Database, Collection, bson::{Document, doc} };
use actix_web::{ web, get, HttpResponse };
use bson::Bson;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use serde_json::json;

use crate::models::models::{ Params, IndicadorCompra };
use crate::routes::routes::indicadores_inversion;

#[get("/indicadores/ventasCompras")]
pub async fn ventas(client: web::Data<mongodb::Client>, query: web::Query<Params>) -> HttpResponse {

    let db: Database = client.database("tienda_online");
    let ventas: Collection<Document> = db.collection("ventas");

    //2024-06-01T00:00:00Z
    //2024-06-15T00:00:00Z

    let mut filter_bson : Bson = bson::to_bson(&doc! {}).unwrap();

    if query.date_init.clone() != "" && query.date_end.clone() == "" {

        let init_date: DateTime<Utc> = match query.date_init.clone().parse() {
            Ok(param) => param,
            Err(_) => return HttpResponse::InternalServerError().json("Formato de fecha incorrecta")
        };

        let doc : Document = doc! {"fecha_venta" : { "$gte": init_date }};
        filter_bson = bson::to_bson(&doc).unwrap();


    } else if query.date_init.clone() == "" && query.date_end.clone() != ""{

        let end_date: DateTime<Utc> = match query.date_end.clone().parse() {
            Ok(param) => param,
            Err(_) => return HttpResponse::InternalServerError().json("Formato de fecha incorrecta")
        };

        let doc : Document = doc! {"fecha_venta" : { "$lte": end_date }};
        filter_bson = bson::to_bson(&doc).unwrap();

    } else if query.date_init.clone() != "" && query.date_end.clone() != "" {
        let init_date: DateTime<Utc> = match query.date_init.clone().parse() {
            Ok(param) => param,
            Err(_) => return HttpResponse::InternalServerError().json("Formato de fecha incorrecta")
        };

        let end_date: DateTime<Utc> = match query.date_end.clone().parse() {
            Ok(param) => param,
            Err(_) => return HttpResponse::InternalServerError().json("Formato de fecha incorrecta")
        };

        let doc : Document = doc! {"fecha_venta" : { "$gte": init_date, "$lte" : end_date }};
        filter_bson = bson::to_bson(&doc).unwrap();

    };

    let mut filter : Document = doc! {
        "$match" : filter_bson
    };

    let mut pipeline: Vec<Document> = [
        filter,
        doc! {
            "$project" : {
                "_id" : "$_id",
                "productos" : "$productos",
                "codigo_venta" : "$codigo_venta",
                "fecha_venta" : "$fecha_venta",
                "total_venta" : "$total_venta",
            }
        }
    ].to_vec();
    

    let mut lista_ventas = vec![];
    let mut new_total_ventas : i32 = 0;

    match ventas.aggregate(pipeline, None).await {
        Ok(mut blogs) => {

            while let Some(result) = blogs.try_next().await.expect("error") {
                
                let total_ventas = result.get_i32("total_venta").unwrap();
                new_total_ventas += total_ventas;
                lista_ventas.push(result)
            }

            HttpResponse::Ok().json(json!(
                {
                    "total_ventas" : new_total_ventas,
                    "ventas" : lista_ventas
                }
            ))

        }
        Err(e) => {
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}