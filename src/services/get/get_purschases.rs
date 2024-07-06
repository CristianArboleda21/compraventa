use actix_web::{ web, get, HttpResponse };
use mongodb::{ Database, Collection, bson::{Document, doc} };
use futures::TryStreamExt;
use tera::{Context, Tera};


#[get("/Compras")]
pub async fn compras(client: web::Data<mongodb::Client>, tera: web::Data<Tera>) -> HttpResponse {

    let db: Database = client.database("tienda_online");
    let compras: Collection<Document> = db.collection("compras");
    let mut context = Context::new();

    let mut lista_compras = [].to_vec();

    match compras.find(None, None).await {
        Ok(mut compras) => {

            while let Some(result) = compras.try_next().await.expect("error") {
                lista_compras.push(result)
            }

            context.insert("compras", &lista_compras);
            let resp = tera.render("indicadores/indicadores_inversion.html", &context).unwrap();
            HttpResponse::Ok().body(resp)

        }
        Err(e) => {
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }

}