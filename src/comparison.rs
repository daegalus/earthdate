use chrono::NaiveDate;
use std::cmp::Ordering;

use crate::MABV;

/// Compare Earthdates in the default C20, Alpha, Metric format.
pub fn compare(left: &str, right: &str) -> Result<Ordering, String> {
    let parse = |value| {
        parse_default(value).ok_or_else(|| {
            format!("invalid Earthdate {value:?}; expected C20/Alpha/Metric (e.g. 26S11.500)")
        })
    };
    Ok(parse(left)?.cmp(&parse(right)?))
}

fn parse_default(value: &str) -> Option<(NaiveDate, u16)> {
    let (date, beats) = value.split_once('.')?;
    if !date.is_ascii() || date.len() < 4 || beats.is_empty() {
        return None;
    }

    let (year, month_day) = date.split_at(date.len() - 3);
    let (month, day) = month_day.split_at(1);
    if ![year, day, beats]
        .iter()
        .all(|part| part.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return None;
    }

    let year = year.parse::<i32>().ok()?.checked_add(2000)?;
    let month = MABV.iter().position(|letter| *letter == month)? as u32;
    let date = NaiveDate::from_ymd_opt(year, month, day.parse().ok()?)?;
    let beats = beats.parse::<u16>().ok()?;
    (beats < 1000).then_some((date, beats))
}
