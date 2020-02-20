use rustling_ontology::*;
use rustling_ontology::output::*;

use std::f64;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SlotValue {
    Number(NumberValue),
    Ordinal(OrdinalValue),
    Percentage(PercentageValue),
    InstantTime(InstantTimeValue),
    TimeInterval(TimeIntervalValue),
    AmountOfMoney(AmountOfMoneyValue),
    Temperature(TemperatureValue),
    Duration(DurationValue),
}

impl From<Output> for SlotValue {
    fn from(o: Output) -> SlotValue {
        match o {
            Output::Integer(int) => SlotValue::Number(NumberValue {
                value: (int.0 as f64).into(),
            }),
            Output::Float(float) => SlotValue::Number(NumberValue {
                value: float.0.into(),
            }),
            Output::Ordinal(ordinal) => SlotValue::Ordinal(OrdinalValue {
                value: ordinal.0 as i64,
            }),
            Output::Percentage(percentage) => SlotValue::Percentage(PercentageValue {
                value: percentage.0.into(),
            }),
            Output::Datetime(datetime) => SlotValue::InstantTime(InstantTimeValue {
                value: datetime.moment,
                grain: datetime.grain.into(),
                precision: datetime.precision.into(),
            }),
            Output::DatetimeInterval(datetime_interval) => match datetime_interval.interval_kind {
                DatetimeIntervalKind::After(datetime) => {
                    SlotValue::TimeInterval(TimeIntervalValue {
                        from: Some(datetime.moment),
                        to: None,
                    })
                }
                DatetimeIntervalKind::Before(datetime) => {
                    SlotValue::TimeInterval(TimeIntervalValue {
                        from: None,
                        to: Some(datetime.moment),
                    })
                }
                DatetimeIntervalKind::Between { start, end, .. } => {
                    SlotValue::TimeInterval(TimeIntervalValue {
                        from: Some(start),
                        to: Some(end),
                    })
                }
            },
            Output::AmountOfMoney(amount) => SlotValue::AmountOfMoney(AmountOfMoneyValue {
                value: amount.value,
                precision: amount.precision.into(),
                unit: amount.unit.map(|it| it.to_string()),
            }),
            Output::Temperature(temperature) => SlotValue::Temperature(TemperatureValue {
                value: temperature.value,
                unit: temperature.unit.map(|it| it.to_string()),
            }),
            Output::Duration(duration) => SlotValue::Duration(DurationValue {
                years: *duration.period.0.get(Grain::Year as usize).unwrap_or(&0),
                quarters: *duration.period.0.get(Grain::Quarter as usize).unwrap_or(&0),
                months: *duration.period.0.get(Grain::Month as usize).unwrap_or(&0),
                weeks: *duration.period.0.get(Grain::Week as usize).unwrap_or(&0),
                days: *duration.period.0.get(Grain::Day as usize).unwrap_or(&0),
                hours: *duration.period.0.get(Grain::Hour as usize).unwrap_or(&0),
                minutes: *duration.period.0.get(Grain::Minute as usize).unwrap_or(&0),
                seconds: *duration.period.0.get(Grain::Second as usize).unwrap_or(&0),
                precision: duration.precision.into(),
            }),
        }
    }
}

fn nearly_equal_f64(a: f64, b: f64) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    if a == b {
        // Handle infinities.
        true
    } else if a == 0.0 || b == 0.0 || diff < f64::MIN_POSITIVE {
        // One of a or b is zero (or both are extremely close to it,) use absolute error.
        diff < (f64::EPSILON * f64::MIN_POSITIVE)
    } else {
        // Use relative error.
        (diff / f64::min(abs_a + abs_b, f64::MAX)) < 0.00001
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct NumberValue {
    pub value: f64,
}

impl PartialEq for NumberValue {
    fn eq(&self, other: &NumberValue) -> bool {
        nearly_equal_f64(self.value, other.value)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Copy, Debug)]
pub struct OrdinalValue {
    pub value: i64,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct PercentageValue {
    pub value: f64,
}

impl PartialEq for PercentageValue {
    fn eq(&self, other: &PercentageValue) -> bool {
        nearly_equal_f64(self.value, other.value)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct InstantTimeValue {
    #[serde(with = "moment_json")]
    pub value: Moment<Local>,
    pub grain: TimeGrain,
    pub precision: Precision,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TimeIntervalValue {
    #[serde(with = "optional_moment_json")]
    pub from: Option<Moment<Local>>,
    #[serde(with = "optional_moment_json")]
    pub to: Option<Moment<Local>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AmountOfMoneyValue {
    pub value: f64,
    pub precision: Precision,
    pub unit: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TemperatureValue {
    pub value: f64,
    pub unit: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct DurationValue {
    pub years: i64,
    pub quarters: i64,
    pub months: i64,
    pub weeks: i64,
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
    pub precision: Precision,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug, Hash)]
pub enum TimeGrain {
    Year = 0,
    Quarter = 1,
    Month = 2,
    Week = 3,
    Day = 4,
    Hour = 5,
    Minute = 6,
    Second = 7,
}

impl From<Grain> for TimeGrain {
    fn from(o: Grain) -> TimeGrain {
        match o {
            Grain::Year => TimeGrain::Year,
            Grain::Quarter => TimeGrain::Quarter,
            Grain::Month => TimeGrain::Month,
            Grain::Week => TimeGrain::Week,
            Grain::Day => TimeGrain::Day,
            Grain::Hour => TimeGrain::Hour,
            Grain::Minute => TimeGrain::Minute,
            Grain::Second => TimeGrain::Second,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum Precision {
    Approximate,
    Exact,
}

impl From<dimension::Precision> for Precision {
    fn from(o: dimension::Precision) -> Precision {
        match o {
            dimension::Precision::Approximate => Precision::Approximate,
            dimension::Precision::Exact => Precision::Exact,
        }
    }
}

mod moment_json {
    use rustling_ontology::{Local, Moment};
    use serde::{Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(
        moment: &Moment<Local>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        moment
            .0
            .format("%Y-%m-%d %T")
            .to_string()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        _deserializer: D,
    ) -> Result<Moment<Local>, D::Error> {
        let datetime = Local::now();
        Ok(Moment(datetime))
    }
}

mod optional_moment_json {
    use super::*;
    use rustling_ontology::{Local, Moment};
    use serde::{Deserializer, Serializer};

    pub fn serialize<S: Serializer>(
        moment: &Option<Moment<Local>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match moment {
            &Some(ref moment) => moment_json::serialize(moment, serializer),
            &None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        _deserializer: D,
    ) -> Result<Option<Moment<Local>>, D::Error> {
        let datetime = Local::now();
        Ok(Some(Moment(datetime)))
    }
}