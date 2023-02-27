#![allow(dead_code)]

use chrono::{Datelike, Days, NaiveDate, Weekday};
use phf_macros::phf_map;
use std::ops::{Range, RangeInclusive};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[macro_export]
macro_rules! ត្រូវនឹងថ្ងៃ {
    ($ថ្ងៃទី:expr, $ខ្នើត:ident ខែ $ខែ:ident) => {
        LunarDate {
            day: LunarDay {
                order: $ថ្ងៃទី,
                moon_status: MoonStatus::$ខ្នើត,
            },
            month: LunarMonth::$ខែ,
        }
    };
}

#[macro_export]
macro_rules! វេលា {
    ($hour:expr; $minute:expr) => {
        Time {
            hour: $hour,
            minute: $minute,
        }
    };
}

const EXCLUSION: phf::Map<&'static str, (LunarDate, Time, LunarDate)> = phf_map! {
    "1879" => (ត្រូវនឹងថ្ងៃ!(6, រោច ខែ ចេត្រ), វេលា!(11;36), ត្រូវនឹងថ្ងៃ!(8, រោច ខែ ចេត្រ)),
    "1897" => (ត្រូវនឹងថ្ងៃ!(10, កើត ខែ ចេត្រ), វេលា!(2;00), ត្រូវនឹងថ្ងៃ!(12, កើត ខែ ចេត្រ)),
    "2011" => (ត្រូវនឹងថ្ងៃ!(11, កើត ខែ ចេត្រ), វេលា!(13;12), ត្រូវនឹងថ្ងៃ!(13, កើត ខែ ចេត្រ)),
    "2012" => (ត្រូវនឹងថ្ងៃ!(7, រោច ខែ ចេត្រ), វេលា!(19;11), ត្រូវនឹងថ្ងៃ!(9, រោច ខែ ចេត្រ)),
    "2013" => (ត្រូវនឹងថ្ងៃ!(4, កើត ខែ ចេត្រ), វេលា!(2;12), ត្រូវនឹងថ្ងៃ!(6, កើត ខែ ចេត្រ)),
    "2014" => (ត្រូវនឹងថ្ងៃ!(15, កើត ខែ ចេត្រ), វេលា!(8;07), ត្រូវនឹងថ្ងៃ!(2, រោច ខែ ចេត្រ)),
    "2015" => (ត្រូវនឹងថ្ងៃ!(11, រោច ខែ ចេត្រ), វេលា!(14;02), ត្រូវនឹងថ្ងៃ!(13, រោច ខែ ចេត្រ)),
};

const លំដាប់ថ្ងៃ១រោចខែពិសាខ: u8 = 29 + 30 + 29 + 30 + 29 + 15;

const តារាងឆាយាអាទិត្យ: [ឆាយាអាទិត្យ; 7] = [
    ឆាយាអាទិត្យ {
        មេគុណ: Some(35),
        ឆាយា: 0,
    },
    ឆាយាអាទិត្យ {
        មេគុណ: Some(32),
        ឆាយា: 35,
    },
    ឆាយាអាទិត្យ {
        មេគុណ: Some(27),
        ឆាយា: 67,
    },
    ឆាយាអាទិត្យ {
        មេគុណ: Some(22),
        ឆាយា: 94,
    },
    ឆាយាអាទិត្យ {
        មេគុណ: Some(13),
        ឆាយា: 116,
    },
    ឆាយាអាទិត្យ {
        មេគុណ: Some(5),
        ឆាយា: 129,
    },
    ឆាយាអាទិត្យ {
        មេគុណ: None,
        ឆាយា: 135,
    },
];

macro_rules! modulo {
    ($x:expr, $y:expr) => {
        (($x % $y) + $y) % $y
    };
}

pub fn គណនាចំនួនថ្ងៃពីដើមខែមិគសិរឆ្នាំចាស់ដល់ចុងកក្ដិកឆ្នាំថ្មី(
    ចុល្លសករាជថ្មី: i128,
) -> i128 {
    let _សុរិយាឡើងស័ក = សុរិយាឡើងស័ក::from_jolasakrach(ចុល្លសករាជថ្មី);
    let មិគសិរ_បុស្ស_មាឃ_ផល្គុន_ចេត្រ_ពិសាខ_ស្រាពណ៍_ភទ្របទ_អស្សុជ_កក្ដិក =
        29 + 30 + 29 + 30 + 29 + 30 + 29 + 30 + 29 + 30;
    let mut ចំនួនថ្ងៃ = មិគសិរ_បុស្ស_មាឃ_ផល្គុន_ចេត្រ_ពិសាខ_ស្រាពណ៍_ភទ្របទ_អស្សុជ_កក្ដិក;
    if _សុរិយាឡើងស័ក.ជាឆ្នាំអធិកមាស() {
        ចំនួនថ្ងៃ = ចំនួនថ្ងៃ + 60;
    } else {
        ចំនួនថ្ងៃ = ចំនួនថ្ងៃ + 30;
    }
    if _សុរិយាឡើងស័ក.ជាឆ្នាំចន្ទ្រាធិមាស()
    {
        ចំនួនថ្ងៃ = ចំនួនថ្ងៃ + 30;
    } else {
        ចំនួនថ្ងៃ = ចំនួនថ្ងៃ + 29;
    }
    ចំនួនថ្ងៃ
}

fn គណនាឆ្នាំបុរាណសករាជថ្មីក្នុងគ្រិស្តសករាជ<
    T: Into<i128>,
>(
    gregorian_year: T,
) -> i128 {
    return gregorian_year.into() - 633i128;
}

pub fn គណនាឆ្នាំមហាសករាជពីចុល្លសករាជ<
    T: Into<i128>,
>(
    ចុល្លសករាជ: T,
) -> i128 {
    return ចុល្លសករាជ.into() + 565i128;
}

fn គណនាឆ្នាំមហាសករាជថ្មីក្នុងគ្រិស្តសករាជ<
    T: Into<i128>,
>(
    gregorian_year: T,
) -> i128 {
    return gregorian_year.into() - 73i128;
}

pub fn គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ(
    gregorian_year: i128,
) -> i128 {
    return gregorian_year - 638;
}

fn គណនាឆ្នាំចុល្លសករាជថ្មីពីពុទ្ធសករាជថ្មី(
    ពុទ្ធសករាជ: i128,
) -> i128 {
    return ពុទ្ធសករាជ - 1182;
}

fn គណនាឆ្នាំពុទ្ធសករាជថ្មីពីចុល្លសករាជថ្មី(
    ចុល្លសករាជ: i128,
) -> i128 {
    return ចុល្លសករាជ + 1182;
}

pub fn parse_iso_date(d: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(d, "%Y-%m-%d")
}

pub fn weekday_from_number(num: u8) -> Result<Weekday, String> {
    match num {
        0 => Ok(Weekday::Sat),
        1 => Ok(Weekday::Sun),
        2 => Ok(Weekday::Mon),
        3 => Ok(Weekday::Tue),
        4 => Ok(Weekday::Wed),
        5 => Ok(Weekday::Thu),
        6 => Ok(Weekday::Fri),
        _ => Err("Failed to parse weekday".to_owned()),
    }
}

pub fn lunar_day_from_number(num: u8) -> Result<LunarDay, String> {
    if num < 1 || num > 30 {
        return Err("Invalid day : {num}".to_owned());
    }
    Ok(LunarDay {
        order: if num > 15 { num - 15 } else { num },
        moon_status: if num > 15 {
            MoonStatus::រោច
        } else {
            MoonStatus::កើត
        },
    })
}

#[derive(Debug, PartialEq, EnumIter, Clone, Copy)]
pub enum សត្វ {
    ជូត,
    ឆ្លូវ,
    ខាល,
    ថោះ,
    រោង,
    ម្សាញ់,
    មមីរ,
    មមែ,
    វក,
    រកា,
    ច,
    កុរ,
}

impl សត្វ {
    pub fn from_num(num: u8) -> Self {
        let mut index = 1;
        for _សត្វ in សត្វ::iter() {
            if num == index {
                return _សត្វ;
            }
            index = index + 1;
        }
        panic!("Number must be from 1 to 12");
    }

    pub fn to_num(&self) -> u8 {
        match self {
            Self::ជូត => 1,
            Self::ឆ្លូវ => 2,
            Self::ខាល => 3,
            Self::ថោះ => 4,
            Self::រោង => 5,
            Self::ម្សាញ់ => 6,
            Self::មមីរ => 7,
            Self::មមែ => 8,
            Self::វក => 9,
            Self::រកា => 10,
            Self::ច => 11,
            Self::កុរ => 12,
        }
    }
}

#[derive(Debug, PartialEq, EnumIter, Clone, Copy)]
pub enum ស័ក {
    ឯកស័ក,
    ទោស័ក,
    ត្រីស័ក,
    ចត្វាស័ក,
    បញ្ចស័ក,
    ឆស័ក,
    សព្វស័ក,
    អដ្ឋស័ក,
    នព្វស័ក,
    សំរឺទ្ធស័ក,
}

impl ស័ក {
    pub fn from_num(num: u8) -> Self {
        let mut index = 1;
        for _ស័ក in ស័ក::iter() {
            if num == index {
                return _ស័ក;
            }
            index = index + 1;
        }
        panic!("Number must be from 1 to 10");
    }

    pub fn to_num(&self) -> u8 {
        match self {
            Self::ឯកស័ក => 1,
            Self::ទោស័ក => 2,
            Self::ត្រីស័ក => 3,
            Self::ចត្វាស័ក => 4,
            Self::បញ្ចស័ក => 5,
            Self::ឆស័ក => 6,
            Self::សព្វស័ក => 7,
            Self::អដ្ឋស័ក => 8,
            Self::នព្វស័ក => 9,
            Self::សំរឺទ្ធស័ក => 10,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
}

impl Time {
    pub fn from_minutes(minute: u16) -> Self {
        Self {
            hour: minute as u8 / 60u8,
            minute: modulo!(minute, 60) as u8,
        }
    }

    pub fn គណនានាទីសរុប(&self) -> u16 {
        self.hour as u16 * 60 + self.minute as u16
    }
}

#[derive(Debug)]
pub struct ឆាយាអាទិត្យ {
    pub មេគុណ: Option<u16>,
    pub ឆាយា: u16,
}

#[derive(Debug, PartialEq, EnumIter, Clone, Copy)]
pub enum LunarMonth {
    មិគសិរ,
    បុស្ស,
    មាឃ,
    ផល្គុន,
    ចេត្រ,
    ពិសាខ,
    ជេស្ឋ,
    អាសាឍ,
    បឋមាសាឍ,
    ទុតិយាសាឍ,
    ស្រាពណ៍,
    ភទ្របទ,
    អស្សុជ,
    កក្ដិក,
}

impl LunarMonth {
    pub fn to_number(&self) -> u8 {
        match self {
            LunarMonth::មិគសិរ => 1,
            LunarMonth::បុស្ស => 2,
            LunarMonth::មាឃ => 3,
            LunarMonth::ផល្គុន => 4,
            LunarMonth::ចេត្រ => 5,
            LunarMonth::ពិសាខ => 6,
            LunarMonth::ជេស្ឋ => 7,
            LunarMonth::អាសាឍ => 8,
            LunarMonth::បឋមាសាឍ => 9,
            LunarMonth::ទុតិយាសាឍ => 10,
            LunarMonth::ស្រាពណ៍ => 11,
            LunarMonth::ភទ្របទ => 12,
            LunarMonth::អស្សុជ => 13,
            LunarMonth::កក្ដិក => 14,
        }
    }
    pub fn compare(&self, another: LunarMonth) -> i8 {
        if another == *self {
            return 0;
        } else if self.to_number() > another.to_number() {
            return 1;
        } else {
            return -1;
        }
    }
    pub fn get_previous_month(&self, ចុល្លសករាជថ្មី: i128) -> Self {
        match self {
            Self::មិគសិរ => Self::កក្ដិក,
            Self::បុស្ស => Self::មិគសិរ,
            Self::មាឃ => Self::បុស្ស,
            Self::ផល្គុន => Self::មាឃ,
            Self::ចេត្រ => Self::ផល្គុន,
            Self::ពិសាខ => Self::ចេត្រ,
            Self::ជេស្ឋ => Self::ពិសាខ,
            Self::អាសាឍ => Self::ជេស្ឋ,
            Self::បឋមាសាឍ => Self::ជេស្ឋ,
            Self::ទុតិយាសាឍ => Self::បឋមាសាឍ,
            Self::ស្រាពណ៍ => {
                if សុរិយាឡើងស័ក::from_jolasakrach(ចុល្លសករាជថ្មី).ជាឆ្នាំអធិកមាស()
                {
                    Self::ទុតិយាសាឍ
                } else {
                    Self::អាសាឍ
                }
            }
            Self::ភទ្របទ => Self::ស្រាពណ៍,
            Self::អស្សុជ => Self::ភទ្របទ,
            Self::កក្ដិក => Self::អស្សុជ,
        }
    }
    pub fn get_next_month(&self, ចុល្លសករាជថ្មី: i128) -> Self {
        match self {
            Self::មិគសិរ => Self::បុស្ស,
            Self::បុស្ស => Self::មាឃ,
            Self::មាឃ => Self::ផល្គុន,
            Self::ផល្គុន => Self::ចេត្រ,
            Self::ចេត្រ => Self::ពិសាខ,
            Self::ពិសាខ => Self::ជេស្ឋ,
            Self::ជេស្ឋ => {
                if សុរិយាឡើងស័ក::from_jolasakrach(ចុល្លសករាជថ្មី).ជាឆ្នាំអធិកមាស()
                {
                    Self::បឋមាសាឍ
                } else {
                    Self::អាសាឍ
                }
            }
            Self::បឋមាសាឍ => Self::ទុតិយាសាឍ,
            Self::ទុតិយាសាឍ => Self::ស្រាពណ៍,
            Self::អាសាឍ => Self::ស្រាពណ៍,
            Self::ស្រាពណ៍ => Self::ភទ្របទ,
            Self::ភទ្របទ => Self::អស្សុជ,
            Self::អស្សុជ => Self::កក្ដិក,
            Self::កក្ដិក => Self::មិគសិរ,
        }
    }
    pub fn get_total_day(&self, ចុល្លសករាជ: i128) -> u8 {
        let this_year = សុរិយាឡើងស័ក::from_jolasakrach(ចុល្លសករាជ);
        match self {
            LunarMonth::ជេស្ឋ => {
                if this_year.ជាឆ្នាំចន្ទ្រាធិមាស() {
                    return 30;
                } else {
                    return 29;
                }
            }
            LunarMonth::បឋមាសាឍ | LunarMonth::ទុតិយាសាឍ => {
                if this_year.ជាឆ្នាំអធិកមាស() {
                    return 30;
                } else {
                    return 0;
                }
            }
            LunarMonth::អាសាឍ => {
                if this_year.ជាឆ្នាំអធិកមាស() {
                    return 0;
                } else {
                    return 30;
                }
            }
            LunarMonth::មិគសិរ
            | LunarMonth::មាឃ
            | LunarMonth::ចេត្រ
            | LunarMonth::ស្រាពណ៍
            | LunarMonth::អស្សុជ => 29,
            LunarMonth::បុស្ស
            | LunarMonth::ផល្គុន
            | LunarMonth::ពិសាខ
            | LunarMonth::ភទ្របទ
            | LunarMonth::កក្ដិក => 30,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MoonStatus {
    កើត,
    រោច,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LunarDay {
    pub order: u8,
    pub moon_status: MoonStatus,
}

impl LunarDay {
    pub fn from_day_in_month(day: u8) -> Self {
        if day > 15 {
            Self {
                order: day - 15,
                moon_status: MoonStatus::រោច,
            }
        } else {
            Self {
                order: day,
                moon_status: MoonStatus::កើត,
            }
        }
    }
    // start from 1 to 30
    pub fn រាប់ថ្ងៃពីដើមខែ(&self) -> u8 {
        self.order
            + (if self.moon_status == MoonStatus::កើត {
                0
            } else {
                15
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LunarDate {
    pub day: LunarDay,
    pub month: LunarMonth,
}

impl LunarDate {
    pub fn រាប់ថ្ងៃពីដើមខែមិគសិរ(
        &self,
        ចុល្លសករាជ: i128,
    ) -> u16 {
        let mut count = 0u16;
        for month in LunarMonth::iter() {
            if self.month == month {
                count = count + self.day.រាប់ថ្ងៃពីដើមខែ() as u16;
                break;
            }
            count = count + month.get_total_day(ចុល្លសករាជ) as u16;
        }
        count
    }
}

#[derive(Debug, Clone)]
pub struct មធ្យមព្រះអាទិត្យ {
    រាសី: u32,
    អង្សា: u32,
    លិប្ដា: u32,
}

impl មធ្យមព្រះអាទិត្យ {
    pub fn from_លិប្ដា(លិប្ដា: u32) -> Self {
        មធ្យមព្រះអាទិត្យ {
            រាសី: លិប្ដា / 1800,
            អង្សា: modulo!(លិប្ដា, 1800) / 60,
            លិប្ដា: modulo!(លិប្ដា, 60),
        }
    }

    pub fn លិប្ដាសរុប(&self) -> u32 {
        (self.រាសី * 30 * 60) + (self.អង្សា * 60) + self.លិប្ដា
    }

    pub fn add(
        &self, another: &មធ្យមព្រះអាទិត្យ
    ) -> មធ្យមព្រះអាទិត្យ {
        let result = self.លិប្ដាសរុប() as i32 + another.លិប្ដាសរុប() as i32;
        មធ្យមព្រះអាទិត្យ::from_លិប្ដា(
            modulo!(result, (12 * 60 * 30)) as u32,
        )
    }

    pub fn subtract(
        &self,
        another: &មធ្យមព្រះអាទិត្យ,
    ) -> មធ្យមព្រះអាទិត្យ {
        let mut result = self.លិប្ដាសរុប() as i32 - another.លិប្ដាសរុប() as i32;
        loop {
            if result > 0 {
                break;
            }
            result += 12 * 1800;
        }
        មធ្យមព្រះអាទិត្យ::from_លិប្ដា(result as u32)
    }

    pub fn គណនាសម្ពោធព្រះអាទិត្យ(
        &self,
    ) -> មធ្យមព្រះអាទិត្យ {
        let _rs1 = self.subtract(&មធ្យមព្រះអាទិត្យ {
            រាសី: 2,
            អង្សា: 20,
            លិប្ដា: 0,
        });
        let កែន = _rs1.រាសី;

        let _rs2 = match កែន {
            0 | 1 | 2 => _rs1.clone(),
            3 | 4 | 5 => មធ្យមព្រះអាទិត្យ {
                រាសី: 6,
                អង្សា: 0,
                លិប្ដា: 0,
            }
            .subtract(&_rs1),
            6 | 7 | 8 => _rs1.subtract(&មធ្យមព្រះអាទិត្យ {
                រាសី: 6,
                អង្សា: 0,
                លិប្ដា: 0,
            }),
            9 | 10 | 11 => មធ្យមព្រះអាទិត្យ {
                រាសី: 11,
                អង្សា: 29,
                លិប្ដា: 60,
            }
            .subtract(&_rs1),
            _ => panic!("កែន មិនត្រឹមត្រូវ"),
        };

        let ខណ្ឌ = match _rs2.អង្សា {
            a if a >= 15 => (2 * _rs2.រាសី) + 1,
            a if a < 15 => 2 * _rs2.រាសី,
            _ => panic!("អង្សា មិនត្រឹមត្រូវ"),
        };

        let pouichalip = match _rs2.អង្សា {
            a if a >= 15 => 60 * (a - 15) + _rs2.លិប្ដា,
            a if a < 15 => 60 * a + _rs2.លិប្ដា,
            _ => panic!("អង្សា មិនត្រឹមត្រូវ"),
        };

        let phol = match ខណ្ឌ {
            0..=5 => {
                let _ឆាយាព្រះអាទិត្យ = &តារាងឆាយាអាទិត្យ[ខណ្ឌ as usize];
                let _total1 = pouichalip * _ឆាយាព្រះអាទិត្យ.មេគុណ.unwrap() as u32;
                let _qp1 = _total1 / 900;
                let _rp1 = modulo!(_total1, 900);
                let _total2 = _qp1 + _ឆាយាព្រះអាទិត្យ.ឆាយា as u32;
                let _qp2 = _total2 / 60;
                let _rp2 = modulo!(_total2, 60);
                មធ្យមព្រះអាទិត្យ {
                    រាសី: 0,
                    អង្សា: _qp2,
                    លិប្ដា: _rp2,
                }
            }
            _ => panic!("ខណ្ឌមិនត្រឹមត្រូវទេ"),
        };

        match កែន {
            0..=5 => self.subtract(&phol),
            6..=11 => self.add(&phol),
            _ => panic!("កែន មិនត្រឹមត្រូវ"),
        }
    }
}

#[derive(Debug)]
pub struct សុរិយាឡើងស័ក {
    ចុល្លសករាជ: i128,
    ហារគុណ: i128,
    kromathopol: u32,
    អវមាន: u32,  
    bodethey: u8, 
}

impl សុរិយាឡើងស័ក {
    pub fn from_jolasakrach(ចុល្លសករាជថ្មី: i128) -> Self {
        let _r: i128 = ចុល្លសករាជថ្មី * 292_207i128 + 373i128;
        let h1 = _r / 800;
        let r1 = modulo!(_r, 800);
        let ហារគុណ = h1 + 1;
        let _a = (11i128 * ហារគុណ) + 650i128;
        let q1 = _a / 692;
        let អវមាន = modulo!(_a, 692);
        let បូតិថី = modulo!((ហារគុណ + q1), 30);

        សុរិយាឡើងស័ក {
            ចុល្លសករាជ: ចុល្លសករាជថ្មី,
            ហារគុណ: ហារគុណ,
            kromathopol: (800 - r1) as u32,
            អវមាន: អវមាន as u32,
            bodethey: បូតិថី as u8,
        }
    }

    // គណនារកឆ្នាំមាន ៣៦៦ថ្ងៃ សុរិយគតិខ្មែរ
    pub fn has366(&self) -> bool {
        self.kromathopol <= 207
    }

    pub fn គណនាចំនួនថ្ងៃវ័នបត(&self) -> Range<u16> {
        let previous_year =
            សុរិយាឡើងស័ក::from_jolasakrach(self.ចុល្លសករាជ - 1);
        let សុទិនដែលត្រូវរាប់: RangeInclusive<u16>;
        if previous_year.has366() {
            សុទិនដែលត្រូវរាប់ = 363..=366;
        } else {
            សុទិនដែលត្រូវរាប់ = 362..=365;
        }
        for សុទិន in សុទិនដែលត្រូវរាប់.clone() {
            let _មធ្យមព្រះអាទិត្យ =
                self.គណនាមធ្យមព្រះអាទិត្យ(សុទិន);
            let _សម្ពោធព្រះអាទិត្យ = _មធ្យមព្រះអាទិត្យ.គណនាសម្ពោធព្រះអាទិត្យ();

            if _សម្ពោធព្រះអាទិត្យ.រាសី == 0 && _សម្ពោធព្រះអាទិត្យ.អង្សា == 0
            {
                return (សុទិន + 1)..(សុទិនដែលត្រូវរាប់.last().unwrap());
            }
        }
        panic!("គណនាចំនួនថ្ងៃវ័នបតមិនចេញ");
    }

    pub fn គណនាឈ្មោះថ្ងៃឡើងស័ក(&self) -> Weekday {
        weekday_from_number(modulo!(self.ហារគុណ, 7) as u8).unwrap()
    }

    pub fn គណនាថ្ងៃឡើងស័ក(&self) -> LunarDate {
        let គស = self.ចុល្លសករាជ + 638;
        let key = format!("{}", គស);
        if EXCLUSION.contains_key(&key) {
            return EXCLUSION.get(&key).unwrap().2;
        }

        let previous_year =
            សុរិយាឡើងស័ក::from_jolasakrach(self.ចុល្លសករាជ - 1);
        if !previous_year.ជាឆ្នាំអធិកមាស() {
            return LunarDate {
                day: if self.bodethey > 5 {
                    lunar_day_from_number(self.bodethey).unwrap()
                } else {
                    lunar_day_from_number(self.bodethey + 1).unwrap()
                },
                month: if self.bodethey > 5 {
                    LunarMonth::ចេត្រ
                } else {
                    LunarMonth::ពិសាខ
                },
            };
        } else {
            if !previous_year.ជាឆ្នាំចន្ទ្រាធិមាស() {
                if self.bodethey > 5 {
                    return LunarDate {
                        day: lunar_day_from_number(self.bodethey).unwrap(),
                        month: LunarMonth::ចេត្រ,
                    };
                } else {
                    return LunarDate {
                        day: lunar_day_from_number(self.bodethey + 1).unwrap(),
                        month: LunarMonth::ពិសាខ,
                    };
                }
            } else {
                let increase_bodethey = self.bodethey + 1;
                if increase_bodethey > 5 {
                    return LunarDate {
                        day: lunar_day_from_number(increase_bodethey).unwrap(),
                        month: LunarMonth::ចេត្រ,
                    };
                } else {
                    return LunarDate {
                        day: lunar_day_from_number(increase_bodethey + 1).unwrap(),
                        month: LunarMonth::ពិសាខ,
                    };
                }
            }
        }
    }

    pub fn គណនាថ្ងៃចូលឆ្នាំ(&self) -> LunarDate {
        let គស = self.ចុល្លសករាជ + 638;
        let key = format!("{}", គស);
        if EXCLUSION.contains_key(&key) {
            return EXCLUSION.get(&key).unwrap().0;
        }

        let ថ្ងៃឡើងស័ក = self.គណនាថ្ងៃឡើងស័ក();
        let ចំនួនថ្ងៃវ័នបត = self.គណនាចំនួនថ្ងៃវ័នបត().len() as u8;
        let លំដាប់ថ្ងៃឡើងស័កក្នុងខែ = ថ្ងៃឡើងស័ក.day.រាប់ថ្ងៃពីដើមខែ();
        if លំដាប់ថ្ងៃឡើងស័កក្នុងខែ > ចំនួនថ្ងៃវ័នបត + 1
        {
            LunarDate {
                day: LunarDay::from_day_in_month(លំដាប់ថ្ងៃឡើងស័កក្នុងខែ - ចំនួនថ្ងៃវ័នបត - 1),
                month: ថ្ងៃឡើងស័ក.month,
            }
        } else {
            let month = ថ្ងៃឡើងស័ក.month.get_previous_month(self.ចុល្លសករាជ);
            LunarDate {
                day: LunarDay::from_day_in_month(
                    month.get_total_day(self.ចុល្លសករាជ)
                        + លំដាប់ថ្ងៃឡើងស័កក្នុងខែ
                        - ចំនួនថ្ងៃវ័នបត
                        - 1,
                ),
                month: month,
            }
        }
    }

    pub fn គណនាវេលាចូលឆ្នាំ(&self) -> KhmerDate {
        let ពស = គណនាឆ្នាំពុទ្ធសករាជថ្មីពីចុល្លសករាជថ្មី(self.ចុល្លសករាជ) - 1;
        let ថ្ងៃចូលឆ្នាំ = self.គណនាថ្ងៃចូលឆ្នាំ();
        let ម៉ោងទទួលទេវតា = self.គណនាម៉ោងទទួលទេវតា();
        let result = KhmerDate::from_khmer_date_time(ពស, ថ្ងៃចូលឆ្នាំ, Some(ម៉ោងទទួលទេវតា));
        return result;
    }

    /// សុទិនជាចំនួនថ្ងៃ គិតថ្ងៃបន្ទាប់នៃថ្ងៃឡើងស័កឆ្នាំចាស់ មកដល់ថ្ងៃដែលយើងចង់គណនាមធ្យមព្រះអាទិត្យ
    /// បន្ទាប់ពីថ្ងៃឡើងស័ក១ថ្ងៃ គឺសុទិន = 1
    /// ដូច្នេះបើឆ្នាំមាន ៣៦៥ ថ្ងៃ ថ្ងៃដែលចូលឆ្នាំ អាចសុទិន ៣៦២ ឬ ៣៦៣
    /// បើឆ្នាំមាន ៣៦៦ ថ្ងៃ ថ្ងៃដែលចូលឆ្នាំ អាចសុទិន ៣៦៣ ឬ ៣៦៤
    pub fn គណនាមធ្យមព្រះអាទិត្យ(
        &self,
        សុទិន: u16,
    ) -> មធ្យមព្រះអាទិត្យ {
        let previous_year =
            សុរិយាឡើងស័ក::from_jolasakrach(self.ចុល្លសករាជ - 1);
        let _total = (សុទិន as u32 * 800) + previous_year.kromathopol;
        let រាសី = _total / 24350;
        let _r3 = modulo!(_total, 24350);
        let អង្សា = _r3 / 811;
        let _r4 = modulo!(_r3, 811);
        let _l1 = _r4 / 14;
        let _r5 = modulo!(_r4, 14);
        let លិប្ដា = _l1 as i32 - 3;

        let mut លិប្ដាសរុប: i32 = 30 * 60 * រាសី as i32 + 60 * អង្សា as i32 + លិប្ដា;

        while លិប្ដាសរុប < 0 {
            លិប្ដាសរុប += 30 * 60 * 12;
        }

        មធ្យមព្រះអាទិត្យ::from_លិប្ដា(លិប្ដាសរុប as u32)
    }

    pub fn គណនាម៉ោងទទួលទេវតា(&self) -> Time {
        let គស = self.ចុល្លសករាជ + 638;
        let key = format!("{}", គស);
        if EXCLUSION.contains_key(&key) {
            return EXCLUSION.get(&key).unwrap().1;
        }

        let ថ្ងៃវ័នបត = self.គណនាចំនួនថ្ងៃវ័នបត();
        let _មធ្យមព្រះអាទិត្យថ្ងៃចូលឆ្នាំ =
            self.គណនាមធ្យមព្រះអាទិត្យ(ថ្ងៃវ័នបត.start - 1);
        let _សម្ពោធព្រះអាទិត្យថ្ងៃចូលឆ្នាំ =
            _មធ្យមព្រះអាទិត្យថ្ងៃចូលឆ្នាំ.គណនាសម្ពោធព្រះអាទិត្យ();
        let លិប្ដា = _សម្ពោធព្រះអាទិត្យថ្ងៃចូលឆ្នាំ.លិប្ដា;
        // ៦០ លិប្ដា = ២៤ ម៉ោង
        let នាទី = លិប្ដា * 24;
        let នាទីក្នង១ថ្ងៃ = 24 * 60;
        Time {
            hour: ((នាទីក្នង១ថ្ងៃ - នាទី) / 60) as u8,
            minute: modulo!((នាទីក្នង១ថ្ងៃ - នាទី), 60) as u8,
        }
    }

    // គណារកឆ្នាំចន្ទ្រាធិមាស ឬ​ឆ្នាំដែលខែជេស្ឋមាន៣០ថ្ងៃ
    pub fn ជាឆ្នាំចន្ទ្រាធិមាស(&self) -> bool {
        if self.has366() && self.អវមាន < 127 {
            return true;
        } else if !self.has366() {
            if self.អវមាន == 137
                && សុរិយាឡើងស័ក::from_jolasakrach(
                    self.ចុល្លសករាជ + 1,
                )
                .អវមាន
                    == 0
            {
                return false;
            } else if self.អវមាន < 138 {
                return true;
            }
        }
        let previous_year =
            សុរិយាឡើងស័ក::from_jolasakrach(self.ចុល្លសករាជ + 1);
        if !previous_year.has366() && previous_year.អវមាន == 137 && self.អវមាន == 0
        {
            return true;
        }
        false
    }

    // គណនារកឆ្នាំអធិកមាស ឬ ឆ្នាំមាន១៣ខែចន្ទគតិ
    pub fn ជាឆ្នាំអធិកមាស(&self) -> bool {
        if self.bodethey == 25
            && សុរិយាឡើងស័ក::from_jolasakrach(self.ចុល្លសករាជ + 1).bodethey == 5
        {
            return false;
        } else if self.bodethey > 24 || self.bodethey < 6 {
            return true;
        } else if self.bodethey == 24
            && សុរិយាឡើងស័ក::from_jolasakrach(self.ចុល្លសករាជ + 1).bodethey == 6
        {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KhmerDate {
    pub gregorian_date: NaiveDate,
    pub lunar_date: LunarDate,
    pub ចុល្លសករាជ: i128,
    pub មហាសករាជ: i128,
    pub ពុទ្ធសករាជ: i128,
    pub ស័ក: ស័ក,
    pub សត្វ: សត្វ,
    pub time: Option<Time>,
}

impl KhmerDate {
    pub fn get_epoch() -> Self {
        KhmerDate {
            gregorian_date: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
            lunar_date: LunarDate {
                day: LunarDay {
                    order: 1,
                    moon_status: MoonStatus::កើត,
                },
                month: LunarMonth::បុស្ស,
            },
            ពុទ្ធសករាជ: 2443,
            សត្វ: សត្វ::កុរ,
            ស័ក: ស័ក::ឯកស័ក,
            ចុល្លសករាជ: 1261,
            មហាសករាជ: 1916,
            time: None,
        }
    }

    pub fn is_valid_date(&self) -> bool {
        let month = self.lunar_date.month.clone();
        let ចុល្លសករាជថ្មី = គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ(self.gregorian_date.year() as i128);
        let max_day_in_month = month.get_total_day(ចុល្លសករាជថ្មី);
        let day_in_month = self.lunar_date.day.រាប់ថ្ងៃពីដើមខែ();
        day_in_month > 0 && day_in_month <= max_day_in_month
    }

    pub fn subtract(&self, days: i128) -> Self {
        self.add(-1 * days, None)
    }

    pub fn add(&self, days: i128, time: Option<Time>) -> Self {
        let mut addition = days;
        let mut current_date = self.lunar_date.clone();
        let mut ចុល្លសករាជថ្មី = គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ( self.gregorian_date.year() as i128);

        loop {
            let day_in_year = current_date
                .រាប់ថ្ងៃពីដើមខែមិគសិរ(ចុល្លសករាជថ្មី);
            let max_day_in_year = គណនាចំនួនថ្ងៃពីដើមខែមិគសិរឆ្នាំចាស់ដល់ចុងកក្ដិកឆ្នាំថ្មី(ចុល្លសករាជថ្មី);

            if addition < 0 {
                if day_in_year as i128 + addition > 0 {
                    let day_in_month = current_date.day.រាប់ថ្ងៃពីដើមខែ();
                    if addition + (day_in_month as i128) > 0 {
                        current_date = LunarDate {
                            month: current_date.month,
                            day: LunarDay::from_day_in_month(day_in_month + addition as u8),
                        };
                        addition = 0;
                    } else {
                        let previous_month = current_date.month.get_previous_month(ចុល្លសករាជថ្មី);
                        let max_day_in_previous_month = previous_month.get_total_day(ចុល្លសករាជថ្មី);
                        current_date = LunarDate {
                            month: previous_month,
                            day: LunarDay::from_day_in_month(1),
                        };
                        addition = addition + (day_in_month as i128) - 1
                            + (max_day_in_previous_month as i128);
                    }
                } else {
                    let max_day_in_previous_year = គណនាចំនួនថ្ងៃពីដើមខែមិគសិរឆ្នាំចាស់ដល់ចុងកក្ដិកឆ្នាំថ្មី(ចុល្លសករាជថ្មី - 1);

                    current_date = LunarDate {
                        month: LunarMonth::មិគសិរ,
                        day: LunarDay::from_day_in_month(1),
                    };
                    addition = addition + (day_in_year as i128) - 1 + max_day_in_previous_year;
                    ចុល្លសករាជថ្មី = ចុល្លសករាជថ្មី - 1;
                }
            } else if addition > 0 {
                let ចំនួនថ្ងៃទៅដល់មិគសិរថ្មី = 1 + max_day_in_year - day_in_year as i128;
                if addition
                    < ចំនួនថ្ងៃទៅដល់មិគសិរថ្មី
                {
                    let day_in_month = current_date.day.រាប់ថ្ងៃពីដើមខែ();
                    let max_day_in_month = current_date.month.get_total_day(ចុល្លសករាជថ្មី);
                    let ចំនួនថ្ងៃទៅដល់ខែថ្មី = (1 + max_day_in_month - day_in_month) as i128;
                    if addition < ចំនួនថ្ងៃទៅដល់ខែថ្មី {
                        current_date = LunarDate {
                            month: current_date.month,
                            day: LunarDay::from_day_in_month(day_in_month + addition as u8),
                        };
                        addition = 0;
                    } else {
                        current_date = LunarDate {
                            month: current_date.month.get_next_month(ចុល្លសករាជថ្មី),
                            day: LunarDay::from_day_in_month(1),
                        };
                        addition =
                            addition - ចំនួនថ្ងៃទៅដល់ខែថ្មី;
                    }
                } else {
                    current_date = LunarDate {
                        month: LunarMonth::មិគសិរ,
                        day: LunarDay {
                            order: 1,
                            moon_status: MoonStatus::កើត,
                        },
                    };
                    addition = addition
                        - ចំនួនថ្ងៃទៅដល់មិគសិរថ្មី;
                    ចុល្លសករាជថ្មី = ចុល្លសករាជថ្មី + 1;
                }
            } else {
                break;
            }
        }

        let _សុរិយាឡើងស័ក =
            សុរិយាឡើងស័ក::from_jolasakrach(ចុល្លសករាជថ្មី);
        let រាប់ពីដើមមិគសិរ = current_date
            .រាប់ថ្ងៃពីដើមខែមិគសិរ(ចុល្លសករាជថ្មី);

        let ថ្ងៃឡើងស័ក = _សុរិយាឡើងស័ក
            .គណនាថ្ងៃឡើងស័ក()
            .រាប់ថ្ងៃពីដើមខែមិគសិរ(ចុល្លសករាជថ្មី);

        let ចុល្លសករាជ = if រាប់ពីដើមមិគសិរ < ថ្ងៃឡើងស័ក
        {
            ចុល្លសករាជថ្មី - 1
        } else {
            ចុល្លសករាជថ្មី
        };

        let មហាសករាជ =
            គណនាឆ្នាំមហាសករាជពីចុល្លសករាជ(
                ចុល្លសករាជ,
            );

        let ពុទ្ធសករាជ = if រាប់ពីដើមមិគសិរ
            < លំដាប់ថ្ងៃ១រោចខែពិសាខ as u16
        {
            ចុល្លសករាជថ្មី + 1181
        } else {
            ចុល្លសករាជថ្មី + 1182
        };

        let mut _ស័ក = self.ស័ក.to_num() as i128 + (ចុល្លសករាជ - self.ចុល្លសករាជ);
        _ស័ក = modulo!(_ស័ក, 10);
        _ស័ក = if _ស័ក == 0 { 10 } else { _ស័ក };

        let ថ្ងៃចូលឆ្នាំ = _សុរិយាឡើងស័ក
            .គណនាថ្ងៃចូលឆ្នាំ()
            .រាប់ថ្ងៃពីដើមខែមិគសិរ(ចុល្លសករាជថ្មី);
        let mut _សត្វ = self.សត្វ.to_num() as i128
            + (if រាប់ពីដើមមិគសិរ < ថ្ងៃចូលឆ្នាំ
            {
                ចុល្លសករាជថ្មី - 1 - self.ចុល្លសករាជ
            } else if រាប់ពីដើមមិគសិរ > ថ្ងៃចូលឆ្នាំ
            {
                ចុល្លសករាជថ្មី - self.ចុល្លសករាជ
            } else {
                if let Some(time) = self.time {
                    let នាទីចូលឆ្នាំ = _សុរិយាឡើងស័ក.គណនាម៉ោងទទួលទេវតា().គណនានាទីសរុប();
                    if time.គណនានាទីសរុប() < នាទីចូលឆ្នាំ
                    {
                        ចុល្លសករាជថ្មី - 1 - self.ចុល្លសករាជ
                    } else {
                        ចុល្លសករាជថ្មី - self.ចុល្លសករាជ
                    }
                } else {
                    ចុល្លសករាជថ្មី - self.ចុល្លសករាជ
                }
            });
        _សត្វ = modulo!(_សត្វ, 12);
        _សត្វ = if _សត្វ == 0 {
            12
        } else {
            _សត្វ
        };

        Self {
            gregorian_date: if days >= 0 {
                self.gregorian_date
                    .checked_add_days(Days::new(days as u64))
                    .unwrap()
            } else {
                self.gregorian_date
                    .checked_sub_days(Days::new(days.abs() as u64))
                    .unwrap()
            },
            lunar_date: current_date,
            ចុល្លសករាជ: ចុល្លសករាជ,
            មហាសករាជ: មហាសករាជ,
            ពុទ្ធសករាជ: ពុទ្ធសករាជ,
            ស័ក: ស័ក::from_num(_ស័ក as u8),
            សត្វ: សត្វ::from_num(_សត្វ as u8),
            time: time,
        }
    }

    pub fn គណនាវេលាចូលឆ្នាំបន្ទាប់(&self) -> Self {
        let mut ចុល្លសករាជ = គណនាឆ្នាំចុល្លសករាជថ្មីក្នុងគ្រិស្តសករាជ(self.gregorian_date.year() as i128);
        loop {
            let _សុរិយាឡើងស័ក =
                សុរិយាឡើងស័ក::from_jolasakrach(ចុល្លសករាជ);
            let វេលាចូលឆ្នាំ = Self::from_khmer_date_time(គណនាឆ្នាំពុទ្ធសករាជថ្មីពីចុល្លសករាជថ្មី(ចុល្លសករាជ) - 1, _សុរិយាឡើងស័ក.គណនាថ្ងៃចូលឆ្នាំ(), Some(_សុរិយាឡើងស័ក.គណនាម៉ោងទទួលទេវតា()));
            let mut compared_minutes = វេលាចូលឆ្នាំ
                .gregorian_date
                .signed_duration_since(self.gregorian_date)
                .num_minutes();

            if let Some(new_year_time) = វេលាចូលឆ្នាំ.time {
                if let Some(time) = self.time {
                    compared_minutes +=
                        new_year_time.គណនានាទីសរុប() as i64 - time.គណនានាទីសរុប() as i64;
                }
            }

            if compared_minutes > 0 {
                break វេលាចូលឆ្នាំ;
            }
            ចុល្លសករាជ += 1;
        }
    }

    pub fn from_naive_date_time(date: NaiveDate, time: Option<Time>) -> Self {
        let duration_from_epoche = date.signed_duration_since(Self::get_epoch().gregorian_date);
        Self::get_epoch().add(duration_from_epoche.num_days() as i128, time)
    }

    pub fn from_khmer_date_time(
        ពុទ្ធសករាជ: i128, ថ្ងៃខែ: LunarDate, time: Option<Time>
    ) -> Self {
        let epoch = KhmerDate::get_epoch();
        let mut marked_date = epoch.clone().lunar_date;
        let mut marked_ពស_ថ្មី = epoch.clone().ពុទ្ធសករាជ + 1;
        let mut different = 0i128;
        let ពីដើមមិគសិរ = ថ្ងៃខែ.រាប់ថ្ងៃពីដើមខែមិគសិរ(0);
        let ពុទ្ធសករាជថ្មី = if ពីដើមមិគសិរ
            < លំដាប់ថ្ងៃ១រោចខែពិសាខ as u16
        {
            ពុទ្ធសករាជ + 1
        } else {
            ពុទ្ធសករាជ
        };

        loop {
            let ចុល្លសករាជថ្មី = គណនាឆ្នាំចុល្លសករាជថ្មីពីពុទ្ធសករាជថ្មី(marked_ពស_ថ្មី);
            if ពុទ្ធសករាជថ្មី > marked_ពស_ថ្មី {
                let diff = 1 + គណនាចំនួនថ្ងៃពីដើមខែមិគសិរឆ្នាំចាស់ដល់ចុងកក្ដិកឆ្នាំថ្មី(ចុល្លសករាជថ្មី) - marked_date.រាប់ថ្ងៃពីដើមខែមិគសិរ(ចុល្លសករាជថ្មី) as i128;
                marked_date = LunarDate {
                    day: LunarDay {
                        order: 1,
                        moon_status: MoonStatus::កើត,
                    },
                    month: LunarMonth::មិគសិរ,
                };
                different += diff;
                marked_ពស_ថ្មី += 1;
            } else if ពុទ្ធសករាជថ្មី < marked_ពស_ថ្មី {
                let diff = marked_date.រាប់ថ្ងៃពីដើមខែមិគសិរ(ចុល្លសករាជថ្មី) as i128 - 1 + គណនាចំនួនថ្ងៃពីដើមខែមិគសិរឆ្នាំចាស់ដល់ចុងកក្ដិកឆ្នាំថ្មី(ចុល្លសករាជថ្មី - 1) ;
                marked_date = LunarDate {
                    day: LunarDay {
                        order: 1,
                        moon_status: MoonStatus::កើត,
                    },
                    month: LunarMonth::មិគសិរ,
                };
                different -= diff;
                marked_ពស_ថ្មី -= 1;
            } else {
                let diff = ថ្ងៃខែ.រាប់ថ្ងៃពីដើមខែមិគសិរ(គណនាឆ្នាំចុល្លសករាជថ្មីពីពុទ្ធសករាជថ្មី(ពុទ្ធសករាជថ្មី)) as i128 - marked_date.រាប់ថ្ងៃពីដើមខែមិគសិរ(គណនាឆ្នាំចុល្លសករាជថ្មីពីពុទ្ធសករាជថ្មី(marked_ពស_ថ្មី)) as i128;
                different += diff;
                break;
            }
        }
        return epoch.add(different, time);
    }
}
