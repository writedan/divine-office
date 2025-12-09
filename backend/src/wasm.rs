use crate::{kalendar, runtime, lexer, parser};
use lexer::Lexer;
use parser::Parser;
use runtime::Runtime;
use chrono::{NaiveDate, Datelike};
use std::collections::HashMap;
use std::rc::Rc;

type R<T> = Result <T, String>;

pub fn read_file<P>(path: P) -> R<String>
where
    P: AsRef<std::path::Path> + std::fmt::Debug,
{
    let file = match crate::asset::Asset::get(&path.as_ref().to_string_lossy()) {
        Some(file) => file.data,
        None => return Err(format!("No such file exists: {:?}", path)),
    };

    match std::str::from_utf8(file.as_ref()) {
        Ok(string) => Ok(string.to_string()),
        Err(why) => Err(why.to_string()),
    }
}

pub fn get_identifiers(date: NaiveDate) -> R<(Vec<kalendar::Celebration>, Vec<kalendar::Celebration>)> {
    let kalendar = kalendar::Kalendar::from_date(date).ok_or_else(|| format!("Provided date {:?} is beyond the bounds of the Gregorian calendar.", date))?;
    let today = kalendar.get_celebrations(date)?;
    let tomorrow = kalendar.get_celebrations(date + chrono::Days::new(1))?;

    Ok((today, tomorrow))
}

pub fn get_monthly_identifiers(y: i32, m: u32) -> R<HashMap<u32, Vec<kalendar::Celebration>>> {
    let first_day_of_month = from_ymd(y, m, 1)?;
    let next_month = first_day_of_month
        .with_month(m + 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(y + 1, 1, 1).unwrap());
    let days_in_month = (next_month - first_day_of_month).num_days();

    let mut month: HashMap<u32, Vec<kalendar::Celebration>> = HashMap::new();

    for day in 1..=days_in_month {
        let date = from_ymd(y, m, day as u32)?;

        let identifier = get_identifiers(date)?.0;
        month.insert(day as u32, identifier);
    }

    Ok(month)
}

pub fn get_hour(celebration: kalendar::Celebration, hour: &str) -> R<Vec<runtime::Value>> {
    use runtime::Value;

    let runtime = Runtime::new();

    for iden in celebration.identifiers {
        runtime.borrow_mut().define("propers".into(), Value::String(iden.to_path().display().to_string()));
        runtime.borrow_mut().define("iden.season".into(), Value::String(iden.season.as_str().to_string().to_lowercase()));
        runtime.borrow_mut().define("iden.weekday".into(), Value::String(iden.weekday.to_string()));
        runtime.borrow_mut().define("iden.day".into(), Value::String(iden.day.to_lowercase()));
        runtime.borrow_mut().define("iden.week".into(), Value::String(iden.week.to_lowercase()));

        let mut lexer = Lexer::from_file(iden.season.to_path().join(hour.to_owned() + ".lit"))?;
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let exprs = parser.parse()?;
        Runtime::run(Rc::clone(&runtime), exprs); // value isnt immediately useful to us
    }

    let ordo = match runtime.borrow().get("order".into()) {
        Some(s) => s,
        None => return Err("Field \"order\" was not set.".into())
    };

    let mut lexer = Lexer::from_file(ordo.to_string())?;
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let exprs = parser.parse()?;
    Ok(Runtime::run(Rc::clone(&runtime), exprs))
}

pub fn get_exprs(input: String) -> R<Vec<parser::Expr>> {
    let mut lexer = Lexer::from_str(input.as_str());
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse()
}

pub fn from_ymd(year: i32, month: u32, day: u32) -> R<NaiveDate> {
    NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| format!("Provided date {}-{}-{} is invalid.", year, month, day))
}

pub fn has_first_vespers(today: kalendar::Celebration, tomorrow: kalendar::Celebration) -> bool {
    use kalendar::Rank::*;
    (tomorrow.rank > today.rank && today.rank != StrongFeria && tomorrow.rank != StrongFeria) || today.rank == Eve
}