#![allow(unused_imports, uncommon_codepoints, dead_code)]

use chrono::NaiveDate;
use rust_momentkh::{KhmerDate, LunarDate, LunarDay, LunarMonth, MoonStatus};
use rust_momentkh::parse_iso_date;

fn main() {
    let some_day = parse_iso_date("1996-9-24").expect("Cannot parse date");
    println!("{:?}", KhmerDate::from_naive_date(some_day));
    
    println!(
        "{:?}",
        KhmerDate::from_khmer_date(
            2566,
            LunarDate {
                day: LunarDay {
                    order: 6,
                    moon_status: MoonStatus::កើត
                },
                month: LunarMonth::ផល្គុន,
            }
        )
    );
}
