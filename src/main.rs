mod timehelp;
mod kalendar;
mod liturgy;

use crate::kalendar::Kalendar;
use crate::liturgy::Liturgy;
use chrono::NaiveDate;

fn main() {
    let date = NaiveDate::from_ymd(2024, 11, 23);
    println!("{:#?}", get_hours(date));
}

fn get_hours(date: NaiveDate) -> Liturgy {
    let ly = Kalendar::from_date(date).unwrap();
    let today = ly.get_celebrations(date);
    let tomorrow = ly.get_celebrations(date + chrono::Days::new(1));

    crate::liturgy::resolve_hours(&today[0], &tomorrow[0])
}