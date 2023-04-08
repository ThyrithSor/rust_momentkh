#![allow(uncommon_codepoints, dead_code)]

mod momentkh;

#[cfg(test)]
mod tests {
    use chrono::{Weekday, Datelike, NaiveTime};

    use crate::momentkh::{
        parse_iso_date, KhmerDate, គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ, គណនាឆ្នាំពុទ្ធសករាជថ្មីពីចុល្លសករាជថ្មី, សុរិយាឡើងស័ក, LunarDate, MoonStatus, LunarDay, ស័ក, សត្វ, parse_khmer_number
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
            let _សុរិយាឡើងស័ក = សុរិយាឡើងស័ក::from_jolasakrach(គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ(year));
            assert_eq!(_សុរិយាឡើងស័ក.គណនាឈ្មោះថ្ងៃឡើងស័ក(), *expect.get(i).unwrap());
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

    #[test]
    fn khmer_number() {
        assert_eq!(parse_khmer_number("១៥"), 15);
        assert_eq!(parse_khmer_number("០១"), 1);
        assert_eq!(parse_khmer_number("-៥៦១"), -561);
    }

    #[test]
    fn new_year_day_of_week() {
        for year in 2023..=20000 {
            let ចុល្លសករាជថ្មី = គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ(year);
            let ពុទ្ធសករាជថ្មី = គណនាឆ្នាំពុទ្ធសករាជថ្មីពីចុល្លសករាជថ្មី(ចុល្លសករាជថ្មី);
            let _សុរិយាឡើងស័ក = សុរិយាឡើងស័ក::from_jolasakrach(ចុល្លសករាជថ្មី);
            let ឈ្មោះថ្ងៃឡើងស័ក = _សុរិយាឡើងស័ក.គណនាឈ្មោះថ្ងៃឡើងស័ក();
            let ថ្ងៃឡើងស័ក = _សុរិយាឡើងស័ក.គណនាថ្ងៃឡើងស័ក();
            let ថ្ងៃខ្មែរឡើងស័ក = KhmerDate::from_khmer_date_time(ពុទ្ធសករាជថ្មី - 1, ថ្ងៃឡើងស័ក, None);

            assert_eq!(ឈ្មោះថ្ងៃឡើងស័ក, ថ្ងៃខ្មែរឡើងស័ក.gregorian_date.and_time(NaiveTime::from_num_seconds_from_midnight_opt(0, 0).unwrap()).weekday(), "Year: {}, {:?}", year, ថ្ងៃខ្មែរឡើងស័ក);
        }
    }
}
