#![allow(unused_imports, uncommon_codepoints, dead_code)]

mod momentkh;

use std::time::Instant;

use chrono::{NaiveDate, Datelike};
use momentkh::{parse_iso_date, KhmerDate, LunarDate, LunarDay, LunarMonth, MoonStatus, សុរិយាឡើងស័ក, គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ, Time};

fn main() {
    println!("{:?}", KhmerDate::from_naive_date_time(NaiveDate::from_ymd_opt(2041, 4, 14).unwrap(), None));
    // println!("{:?}", ត្រូវនឹងថ្ងៃ!(1 កើត ខែ មិគសិរ));
    println!("{}", សុរិយាឡើងស័ក::from_jolasakrach(គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ(2041)).ខែជេស្ឋមាន៣០ថ្ងៃ());
    let year = សុរិយាឡើងស័ក::from_jolasakrach(គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ(2041));
    println!("{:?}", year.គណនាថ្ងៃឡើងស័ក());
    println!("{:?}", year.គណនាវេលាចូលឆ្នាំ());
}
