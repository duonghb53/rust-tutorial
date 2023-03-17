#![allow(unused)]
fn main() {
    let f = 3.3_f64;
    let g = -3.3_f64;

    assert_eq!(f.round(), 3.0);
    assert_eq!(g.round(), -3.0);
    let i: u32 = 10;
    println!("{i}");
    let mut var = String::from("DuongHB");
    let data = SalesPromotionProductCodeRowForProductCode::new(None);

    //println!("{:#?}", data.detail.as_ref().map(|d|d.product_code_id));
    //println!("{:?}", data.detail.as_ref().and_then(|f|Some(f.product_code_id.clone())));
    println!(
        "{:?}",
        data.detail.as_ref().map(|f| f.product_code_id.clone())
    );
    println!(
        "{:?}",
        data.detail
            .as_ref()
            .map(|f| f.period_sales_amount.clone())
            .flatten()
    );
    let a = "Chạy được Emacs rồi".to_string();
    println!("{a}");
}

#[derive(Debug, Clone)]
pub struct SalesPromotionProductCodeRowForProductCode {
    detail: Option<SalesPromotionProductCodeProductCodeRow>,
}

#[derive(Debug, Clone)]
pub struct SalesPromotionProductCodeProductCodeRow {
    pub product_code_id: String,
    pub customer_id: String,
    pub product_code_name: String,
    pub period_sales_amount: Option<f64>,
}

impl SalesPromotionProductCodeRowForProductCode {
    pub fn new(detail: Option<SalesPromotionProductCodeProductCodeRow>) -> Self {
        Self { detail }
    }
}
