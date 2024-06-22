mod timehelp;
mod liturgy;

use crate::liturgy::Kalendar;
use chrono::NaiveDate;

fn main() {
    let ly = Kalendar::from_year(2023).unwrap();
    println!("{:#?}", ly.get_celebrations(NaiveDate::from_ymd(2024, 2, 14)));
}