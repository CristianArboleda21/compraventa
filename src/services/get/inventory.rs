use actix_web::{ web, get, HttpResponse };
use mongodb::{ Database, Collection, bson::{Document, doc} };
use futures::TryStreamExt;
use tera::{Context, Tera};

use crate::models::models::Invetario;

#[get("/inventario/productosInventario")]
pub async fn inventory(client: web::Data<mongodb::Client>, tera: web::Data<Tera>) -> HttpResponse {
    
    let db: Database = client.database("tienda_online");
    let productos: Collection<Document> = db.collection("productos");
    let mut context = Context::new();
    
    let mut lista_productos = [].to_vec();
    
    match productos.find(None, None).await { 
        Ok(mut productos) => {
            
            while let Some(result) = productos.try_next().await.expect("error") {

                let nombre: String = match result.get_str("nombre").unwrap().parse() {
                    Ok(nombre) => nombre,
                    Err(e) => {
                        return HttpResponse::InternalServerError().json(e.to_string());
                    }
                };
                
                let codigo = result.get_i32("codigo").unwrap();
                let cantidad = result.get_i32("cantidad").unwrap();
                let precio_venta = result.get_i32("precio_venta").unwrap();
                
                let producto = Invetario {
                    nombre,
                    codigo,
                    cantidad,
                    precio_venta
                };
                
                lista_productos.push(producto);
            }
            
            context.insert("productos", &lista_productos);
            let resp = tera.render("inventario/productos_inventario.html", &context).unwrap();
            HttpResponse::Ok().body(resp)
            
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
    
}