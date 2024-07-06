use actix_web::{ web, get, HttpResponse };
use mongodb::{ Database, Collection, bson::{Document, doc} };
use futures::TryStreamExt;
use tera::{Context, Tera};


#[get("/Ventas")]
pub async fn sales(client: web::Data<mongodb::Client>, tera: web::Data<Tera>) -> HttpResponse {

    let db: Database = client.database("tienda_online");
    let ventas: Collection<Document> = db.collection("ventas");
    let mut context = Context::new();

    let mut lista_ventas = [].to_vec();

    match ventas.find(None, None).await {
        Ok(mut ventas) => {

            while let Some(result) = ventas.try_next().await.expect("error") {
                lista_ventas.push(result)
            }

            context.insert("ventas", &lista_ventas);
            let resp = tera.render("indicadores/indicadores_ventas.html", &context).unwrap();
            HttpResponse::Ok().body(resp)

        }
        Err(e) => {
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }

}