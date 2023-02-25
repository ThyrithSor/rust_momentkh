#![allow(unused_imports, uncommon_codepoints, dead_code)]

use chrono::NaiveDate;
use rust_momentkh::{KhmerDate, LunarDate, LunarDay, LunarMonth, MoonStatus, សុរិយាត្រឡើងស័ក, គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ, Time};
use rust_momentkh::parse_iso_date;

fn main() {
    // println!("New year {:?}", សុរិយាត្រឡើងស័ក::from_jolasakrach(គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ(2023)).គណនាម៉ោងទទួលទេវតា());
    let some_day = parse_iso_date("2011-4-14").expect("Cannot parse date");
    println!("{:?}", KhmerDate::from_naive_date_time(some_day, Some(Time {hour: 13, minute: 36})).គណនាវេលាចូលឆ្នាំបន្ទាប់());
    // println!("{:?}", សុរិយាត្រឡើងស័ក::from_jolasakrach(គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ(1896)).គណនាឈ្មោះថ្ងៃឡើងស័ក());
    // println!("{:?}", សុរិយាត្រឡើងស័ក::from_jolasakrach(គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ(1896)).គណនាថ្ងៃឡើងស័ក());
    // println!("{:?}", សុរិយាត្រឡើងស័ក::from_jolasakrach(គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ(1896)).គណនាថ្ងៃចូលឆ្នាំ());
    
    // println!(
    //     "{:?}",
    //     KhmerDate::from_khmer_date_time(
    //         2566,
    //         LunarDate {
    //             day: LunarDay {
    //                 order: 6,
    //                 moon_status: MoonStatus::កើត
    //             },
    //             month: LunarMonth::ផល្គុន,
    //         },
    //         None,
    //     )
    // );
}
