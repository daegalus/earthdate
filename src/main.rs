use chrono::prelude::*;

const MABV: [&str;13] = ["","J","F","M","A","Y","U","L","G","S","O","N","D"];

fn main() {
    let time = Local::now();
    let sweden = FixedOffset::east_opt(3600).unwrap().from_utc_datetime(&time.naive_utc());
    
    println!("Current Time: {}", time.format("%Y-%m-%d %H:%M:%S %Z"));
    // seconds from midnight
    let seconds = time.num_seconds_from_midnight();

    // print the date
    println!("\n# EarthDate/StarDate");
    println!("{}.{}", time.format("%y%j"),seconds*1000/86400);
    println!("{}{}", time.year()-1900, time.format("%m.%d"));
    println!("{}{}.{}", time.year()-1900, time.format("%j"),seconds*1000/86400);

    let mut year = time.year();
    if year < 1000 {
        year += 1900;
    }
    let mut sign = -1;
    if 100.0*year as f64*time.month() as f64 -190002.5 > 0.0 {
        sign = 1;
    }
    let part1 = 367*year as i64;
    let part2 = (7*(year as i64+((time.month() as i64+9)/12)))/4;
    let part3 = (time.day()+((275*time.month())/9)) as i64;
    let part4 = 1721013i64;
    let jd = (part1 - part2 + part3 + part4)*sign;

    println!("\n# Julian Dates");
    println!("{}", jd);
    println!("{}.{}", jd-2400000,seconds*1000/86400);

    println!("\n# @beat");
    println!("{:.2}", (sweden.num_seconds_from_midnight() as f32*1000.0/86400.0));

    println!("\n# EarthyDate");
    println!("{}{}{}", time.format("%y"), MABV[time.month() as usize], time.format("%d.%H%M"));
    println!("{}{}{}", time.format("%y"), MABV[time.month() as usize], seconds*1000/86400);
    println!("{}{}{}", time.format("%Y"), MABV[time.month() as usize], time.format("%d.%H%M"));
    println!("{}{}{}", time.format("%Y"), MABV[time.month() as usize], time.format("%d.%H%M%S"));
}
