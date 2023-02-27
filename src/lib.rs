#![allow(uncommon_codepoints, dead_code)]

mod momentkh;

#[cfg(test)]
mod tests {
    use chrono::Weekday;

    use crate::momentkh::{
        parse_iso_date, KhmerDate, គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ, សុរិយាឡើងស័ក, LunarDate, MoonStatus, LunarDay, ស័ក, សត្វ
    };

    #[test]
    fn new_year_weekday() {
        let mut i = 0;
        let expect = vec![
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
        ];
        for year in 1878..=1891 {
            let _សុរិយាត្រឡើងស័ក = សុរិយាឡើងស័ក::from_jolasakrach(គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ(year));
            assert_eq!(_សុរិយាត្រឡើងស័ក.គណនាឈ្មោះថ្ងៃឡើងស័ក(), *expect.get(i).unwrap());
            i += 1;
        }
    }

    #[test]
    fn birthday() {
        assert_eq!(
            KhmerDate::from_naive_date_time(parse_iso_date("1996-9-24").unwrap(), None),
            KhmerDate {
                gregorian_date: parse_iso_date("1996-9-24").unwrap(),
                lunar_date: LunarDate {
                    day: LunarDay {
                        order: 12,
                        moon_status: MoonStatus::កើត,
                    },
                    month: crate::momentkh::LunarMonth::ភទ្របទ,
                },
                ចុល្លសករាជ: 1358,
                មហាសករាជ: 1923,
                ពុទ្ធសករាជ: 2540,
                ស័ក: ស័ក::អដ្ឋស័ក,
                សត្វ: សត្វ::ជូត,
                time: None
            }
        );
    }
}
