use actix_web::{get, HttpResponse, web};
use tera::{Context, Tera};

// ruta panel inicial
#[get("/")]
async fn index(tera: web::Data<Tera>) -> HttpResponse {

    let mut context = Context::new();
    let resp_html = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(resp_html)

}

// Registros
#[get("/Registros")]
async fn registros(tera: web::Data<Tera>) -> HttpResponse {

    let mut context = Context::new();
    let resp = tera.render("registros/registros.html", &context).unwrap();
    HttpResponse::Ok().body(resp)
    
}

#[get("/Registros/RegistrarCompra")]
async fn registrar_compra(tera: web::Data<Tera>) -> HttpResponse {

    let mut context = Context::new();
    let resp = tera.render("registros/registro_compra.html", &context).unwrap();
    HttpResponse::Ok().body(resp)

}

#[get("/Registros/RegistrarVenta")]
async fn registrar_venta(tera: web::Data<Tera>) -> HttpResponse {

    let mut context = Context::new();
    let resp = tera.render("registros/registro_venta.html", &context).unwrap();
    HttpResponse::Ok().body(resp)

}

// Inventario

// Ruta para registrar un producto en el inventario
#[get("/Inventario")]
async fn inventario(tera: web::Data<Tera>) -> HttpResponse {

    let mut context = Context::new();
    let resp = tera.render("inventario/inventario.html", &context).unwrap();
    HttpResponse::Ok().body(resp)

}

#[get("/Inventario/RegistroProd")]
async fn registro_productos(tera: web::Data<Tera>) -> HttpResponse {

    let mut context = Context::new();
    let resp = tera.render("registros/registro_producto.html", &context).unwrap();
    HttpResponse::Ok().body(resp)

}

#[get("/Inventario/PrecioVenta")]
async fn precio_venta(tera: web::Data<Tera>) -> HttpResponse {

    let mut context = Context::new();
    let resp = tera.render("inventario/precio_venta.html", &context).unwrap();
    HttpResponse::Ok().body(resp)

}


// Indicadores
#[get("/Indicadores")]
async fn indicadores(tera: web::Data<Tera>) -> HttpResponse {

    let mut context = Context::new();
    let resp = tera.render("indicadores/indicadores.html", &context).unwrap();
    HttpResponse::Ok().body(resp)

}

#[get("/Indicadores/Inversion")]
async fn indicadores_inversion(tera: web::Data<Tera>) -> HttpResponse {

    let mut context = Context::new();
    let resp = tera.render("indicadores/indicadores_inversion.html", &context).unwrap();
    HttpResponse::Ok().body(resp)

}

#[get("/Indicadores/Ventas")]
async fn indicadores_ventas(tera: web::Data<Tera>) -> HttpResponse {

    let mut context = Context::new();
    let resp = tera.render("indicadores/indicadores_ventas.html", &context).unwrap();
    HttpResponse::Ok().body(resp)

}