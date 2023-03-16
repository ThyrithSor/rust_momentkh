#![allow(unused_imports, uncommon_codepoints, dead_code)]

mod momentkh;

use std::time::Instant;

use chrono::{NaiveDate, Datelike};
use momentkh::{parse_iso_date, KhmerDate, LunarDate, LunarDay, LunarMonth, MoonStatus, សុរិយាឡើងស័ក, គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ, Time};

fn main() {
    println!("{:?}", ត្រូវនឹងថ្ងៃ!(1 កើត ខែ មិគសិរ));
}
