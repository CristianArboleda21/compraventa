use serde::{Serialize, Deserialize};
use bson::DateTime;


#[derive(Serialize, Deserialize, Clone)]
pub struct Products {
    pub name: String,
    pub code: i32,
    pub amount: i32,
    pub price_sale: i32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProductPost {
    pub name: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Purchase {
    pub name: String,
    pub code: i32,
    pub amount: i32,
    pub date_purchase: DateTime,
    pub price_unit: i32,
    pub total_purchase: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PurchasePost {
    pub name: String,
    pub amount: i32,
    pub price_unit: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SalesProduct {
    pub name: String,
    pub code: i32,
    pub price: i32,
    pub amount: i32,
    pub total: i32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Sales {
    pub products: Vec<SalesProduct>,
    pub code_sale: i32,
    pub date_sale: DateTime,
    pub total_sale: i32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SalesPost {
    pub products: Vec<SalesProduct>,
    pub total_sale: i32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Invetory {
    pub name: String,
    pub code: i32,
    pub amount: i32,
    pub price_sale: i32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdatePriceSale {
    pub price_sale: i32
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    pub date_init: String,
    pub date_end: String
}
#[derive(Serialize, Deserialize, Clone)]
pub struct IndicatorPurchase {
    pub name: String,
    pub code: i32,
    pub amount: i32,
    pub total_purchase: i32,
    pub date_purchase: DateTime
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IndicatorSale {
    pub products: String,
    pub code: i32,
    pub amount: i32,
    pub total_purchase: i32,
    pub date_purchase: DateTime
}
