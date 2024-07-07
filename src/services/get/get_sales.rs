use mongodb::{ Database, Collection, bson::{Document, doc} };
use actix_web::{ web, get, HttpResponse };
use bson::Bson;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use serde_json::json;

use crate::models::models::{ Params, IndicatorSale };
use crate::routes::routes::indicadores_inversion;

#[get("/indicadores/ventasProductos")]
pub async fn sales(client: web::Data<mongodb::Client>, query: web::Query<Params>) -> HttpResponse {

    let db: Database = client.database("tienda_online");
    let sales: Collection<Document> = db.collection("sales");

    //2024-06-01T00:00:00Z
    //2024-06-15T00:00:00Z

    let mut filter_bson : Bson = bson::to_bson(&doc! {}).unwrap();

    if query.date_init.clone() != "" && query.date_end.clone() == "" {

        let init_date: DateTime<Utc> = match query.date_init.clone().parse() {
            Ok(param) => param,
            Err(_) => return HttpResponse::InternalServerError().json("Formato de fecha incorrecta")
        };

        let doc : Document = doc! { "date_sale" : { "$gte": init_date } };
        filter_bson = bson::to_bson(&doc).unwrap();


    } else if query.date_init.clone() == "" && query.date_end.clone() != ""{

        let end_date: DateTime<Utc> = match query.date_end.clone().parse() {
            Ok(param) => param,
            Err(_) => return HttpResponse::InternalServerError().json("Formato de fecha incorrecta")
        };

        let doc : Document = doc! {"date_sale" : { "$lte": end_date }};
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

        let doc : Document = doc! {"date_sale" : { "$gte": init_date, "$lte" : end_date }};
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
                "products" : "$products",
                "code_sale" : "$code_sale",
                "date_sale" : "$date_sale",
                "total_sale" : "$total_sale",
            }
        }
    ].to_vec();
    

    let mut list_sales = vec![];
    let mut new_total_sales : i32 = 0;

    match sales.aggregate(pipeline, None).await {
        Ok(mut sales) => {

            while let Some(result) = sales.try_next().await.expect("error") {
                
                let total_sale = result.get_i32("total_sale").unwrap();
                new_total_sales += total_sale;
                list_sales.push(result)
            }

            HttpResponse::Ok().json(json!(
                {
                    "total_sales" : new_total_sales,
                    "sales" : list_sales
                }
            ))

        }
        Err(e) => {
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}