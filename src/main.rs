#![allow(unused_imports, uncommon_codepoints, dead_code)]

mod momentkh;

use std::time::Instant;

use chrono::{NaiveDate, Datelike};
use momentkh::{parse_iso_date, KhmerDate, LunarDate, LunarDay, LunarMonth, MoonStatus, សុរិយាត្រឡើងស័ក, គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ, Time};

fn main() {
    println!("{:?}", ត្រូវនឹងថ្ងៃ!(1, កើត ខែ មិគសិរ));
    // let now = Instant::now();
    // for year in 1900..5100 {
    //     សុរិយាត្រឡើងស័ក::from_jolasakrach(គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ(year)).គណនាវេលាចូលឆ្នាំ();
    // }
    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
}
