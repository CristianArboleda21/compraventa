use mongodb::{ Database, Collection, bson::{Document, doc} };
use actix_web::{ web, get, HttpResponse };
use bson::Bson;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use serde_json::json;
use tera::{Context, Tera};

use crate::models::models::{ Params, IndicadorCompra };
use crate::routes::routes::indicadores_inversion;

#[get("/indicadores/inversion/compras")]
pub async fn compras(client: web::Data<mongodb::Client>, query: web::Query<Params>, tera: web::Data<Tera> ) -> HttpResponse {

    let db: Database = client.database("tienda_online");
    let compras: Collection<Document> = db.collection("compras");
    let mut context : Context = Context::new();

    //2024-06-01T00:00:00Z
    //2024-06-15T00:00:00Z
    
    let mut filter_bson : Bson = bson::to_bson(&doc! {}).unwrap();

    if query.date_init.clone() != "" && query.date_end.clone() == "" {

        let init_date: DateTime<Utc> = match query.date_init.clone().parse() {
            Ok(param) => param,
            Err(_) => return HttpResponse::InternalServerError().json("Formato de fecha incorrecta")
        };
        
        let doc : Document = doc! {"fecha_compra" : { "$gte": init_date }};
        filter_bson = bson::to_bson(&doc).unwrap();
        

    } else if query.date_init.clone() == "" && query.date_end.clone() != ""{

        let end_date: DateTime<Utc> = match query.date_end.clone().parse() {
            Ok(param) => param,
            Err(_) => return HttpResponse::InternalServerError().json("Formato de fecha incorrecta")
        };
        
        let doc : Document = doc! {"fecha_compra" : { "$lte": end_date }};
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

        let doc : Document = doc! {"fecha_compra" : { "$gte": init_date, "$lte" : end_date }};
        filter_bson = bson::to_bson(&doc).unwrap();

    };

    let mut filter : Document = doc! {
        "$match" : filter_bson

    };

    let mut pipeline: Vec<Document> = [
        filter,
        doc! {
            "$project" : {
                "nombre" : "$nombre",
                "codigo" : "$codigo",
                "cantidad" : "$cantidad",
                "fecha_compra" : "$fecha_compra",
                "precio_unidad" : "$precio_unidad",
                "total_compra" : "$total_compra",
            }
        }

    ].to_vec();

    let mut lista_compras = vec![];
    let mut total_inversion : i32 = 0;

    match compras.aggregate(pipeline, None).await {
        Ok(mut blogs) => {
            
            while let Some(result) = blogs.try_next().await.expect("error") {

                let total_compra = match result.clone().get_i32("total_compra") {
                    Ok(total_compra) => total_compra,
                    Err(_) => { 
                        return HttpResponse::InternalServerError().json("campo total_compra con formato incorrecto")
                    }
                };
                
                let nombre = result.get_str("nombre").unwrap().to_string();
                let codigo = result.get_i32("codigo").unwrap();
                let cantidad = result.get_i32("cantidad").unwrap();
                let fecha = result.get_datetime("fecha_compra").unwrap().clone();
                
                let indicador_compra = IndicadorCompra {
                    nombre,
                    codigo, 
                    cantidad,
                    total_compra,
                    fecha_compra: fecha
                };
                
                total_inversion += total_compra;
                lista_compras.push(indicador_compra)

            }
            
            HttpResponse::Ok().json(json!(
                {
                    "total_inversion" : total_inversion,
                    "compras" : lista_compras
                }
            ))
            
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}