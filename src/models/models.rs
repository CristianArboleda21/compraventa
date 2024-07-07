use serde::{Serialize, Deserialize};
use bson::DateTime;


#[derive(Serialize, Deserialize, Clone)]
pub struct Productos {
    pub nombre: String,
    pub codigo: i32,
    pub cantidad: i32,
    pub precio_venta: i32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProductoPost {
    pub nombre: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Compras {
    pub nombre: String,
    pub codigo: i32,
    pub cantidad: i32,
    pub fecha_compra: DateTime,
    pub precio_unidad: i32,
    pub total_compra: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ComprasPost {
    pub nombre: String,
    pub cantidad: i32,
    pub precio_unidad: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VentasProducto {
    pub nombre: String,
    pub codigo: i32,
    pub precio: i32,
    pub cantidad: i32,
    pub total: i32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ventas {
    pub productos: Vec<VentasProducto>,
    pub codigo_venta: i32,
    pub fecha_venta: DateTime,
    pub total_venta: i32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VentasPost {
    pub productos: Vec<VentasProducto>,
    pub total_venta: i32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Invetario {
    pub nombre: String,
    pub codigo: i32,
    pub cantidad: i32,
    pub precio_venta: i32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActualizacionPrecioVenta {
    pub precio_venta: i32
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    pub date_init: String,
    pub date_end: String
}
#[derive(Serialize, Deserialize, Clone)]
pub struct IndicadorCompra {
    pub nombre: String,
    pub codigo: i32,
    pub cantidad: i32,
    pub total_compra: i32,
    pub fecha_compra: DateTime
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IndicadorVenta {
    pub productos: String,
    pub codigo: i32,
    pub cantidad: i32,
    pub total_compra: i32,
    pub fecha_compra: DateTime
}
