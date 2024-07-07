use mongodb::{ Database, Collection, bson::{Document, doc} };
use actix_web::{ web, get, HttpResponse };
use bson::Bson;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use serde_json::json;
use tera::{Context, Tera};

use crate::models::models::{ Params, IndicatorPurchase };
use crate::routes::routes::indicadores_inversion;

#[get("/indicadores/inversion/compras")]
pub async fn purchases(client: web::Data<mongodb::Client>, query: web::Query<Params>, tera: web::Data<Tera> ) -> HttpResponse {

    let db: Database = client.database("tienda_online");
    let purchases: Collection<Document> = db.collection("purchases");
    let mut context : Context = Context::new();

    //2024-06-01T00:00:00Z
    //2024-06-15T00:00:00Z
    
    let mut filter_bson : Bson = bson::to_bson(&doc! {}).unwrap();

    if query.date_init.clone() != "" && query.date_end.clone() == "" {

        let init_date: DateTime<Utc> = match query.date_init.clone().parse() {
            Ok(param) => param,
            Err(_) => return HttpResponse::InternalServerError().json("Formato de fecha incorrecta")
        };
        
        let doc : Document = doc! { "date_purchase" : { "$gte": init_date } };
        filter_bson = bson::to_bson(&doc).unwrap();
        

    } else if query.date_init.clone() == "" && query.date_end.clone() != ""{

        let end_date: DateTime<Utc> = match query.date_end.clone().parse() {
            Ok(param) => param,
            Err(_) => return HttpResponse::InternalServerError().json("Formato de fecha incorrecta")
        };
        
        let doc : Document = doc! { "date_purchase" : { "$lte": end_date } };
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

        let doc : Document = doc! { "date_purchase" : { "$gte": init_date, "$lte" : end_date } };
        filter_bson = bson::to_bson(&doc).unwrap();

    };

    let mut filter : Document = doc! {
        "$match" : filter_bson

    };

    let mut pipeline: Vec<Document> = [
        filter,
        doc! {
            "$project" : {
                "name" : "$name",
                "code" : "$code",
                "amount" : "$amount",
                "date_purchase" : "$date_purchase",
                "price_unit" : "$price_unit",
                "total_purchase" : "$total_purchase",
            }
        }

    ].to_vec();

    let mut list_purchases = vec![];
    let mut total_investment : i32 = 0;

    match purchases.aggregate(pipeline, None).await {
        Ok(mut purchase) => {
            
            while let Some(result) = purchase.try_next().await.expect("error") {

                let total_purchase = match result.clone().get_i32("total_purchase") {
                    Ok(total_purchase) => total_purchase,
                    Err(_) => { 
                        return HttpResponse::InternalServerError().json("campo total_purchase con formato incorrecto")
                    }
                };
                
                let name = result.get_str("name").unwrap().to_string();
                let code = result.get_i32("code").unwrap();
                let amount = result.get_i32("amount").unwrap();
                let date = result.get_datetime("date_purchase").unwrap().clone();

                let indicador_compra = IndicatorPurchase {
                    name,
                    code,
                    amount,
                    total_purchase,
                    date_purchase: date
                };
                
                total_investment += total_purchase;
                list_purchases.push(indicador_compra)

            }
            
            HttpResponse::Ok().json(json!(
                {
                    "total_investment" : total_investment,
                    "purchases" : list_purchases
                }
            ))
            
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}