use chrono::{
    DateTime, Datelike, Duration as ChronoDuration, FixedOffset, NaiveTime, Utc, Weekday,
};
use failure::Error;
use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;
use std::{
    fs::File,
    io::Read,
    path::Path,
    time::{Duration as StdDuration, Instant},
    vec::IntoIter as VecIntoIter,
};
use tokio_timer::Interval;

const TZ_OFFSET: i32 = 3600 * 3;

#[derive(Debug, Deserialize)]
struct RawScheduleItem {
    day: Day,
    time: NaiveTime,
    messages: Vec<String>,
}

#[derive(Debug, Deserialize)]
enum Day {
    #[serde(rename = "*")]
    Any,
    #[serde(rename = "week")]
    Weekday(Weekday),
}

impl Day {
    fn get_loop_interval(&self) -> ChronoDuration {
        ChronoDuration::days(match *self {
            Day::Any => 1,
            Day::Weekday(_) => 7,
        })
    }

    fn get_start_interval(&self, day: Weekday) -> ChronoDuration {
        ChronoDuration::days(match *self {
            Day::Any => 0,
            Day::Weekday(weekday) => {
                let day_num = i64::from(day.number_from_monday());
                let target_day_num = i64::from(weekday.number_from_monday());
                let mut v = target_day_num - day_num;
                if v < 0 {
                    v += 7;
                }
                v
            }
        })
    }
}

struct ScheduleItemFactory {
    instant: Instant,
    now: DateTime<FixedOffset>,
}

impl ScheduleItemFactory {
    fn new() -> Self {
        Self {
            instant: Instant::now(),
            now: Utc::now().with_timezone(&FixedOffset::east(TZ_OFFSET)),
        }
    }

    fn create(&self, raw: RawScheduleItem) -> ScheduleItem {
        let current_weekday = self.now.weekday();
        let loop_interval = raw.day.get_loop_interval();
        let start_interval = raw.day.get_start_interval(current_weekday);
        let delay = (self.now + start_interval)
            .date()
            .and_time(raw.time)
            .expect("Failed to get datetime for delay");
        let delay = match delay.signed_duration_since(self.now).to_std() {
            Ok(v) => v,
            Err(_) => {
                let delay = delay + loop_interval;
                delay
                    .signed_duration_since(self.now)
                    .to_std()
                    .expect("Failed to get duration for instant")
            }
        };
        ScheduleItem {
            at: self.instant + delay,
            interval: loop_interval.to_std().expect("Failed to get interval"),
            messages: raw.messages,
        }
    }
}

#[derive(Debug)]
pub struct ScheduleItem {
    at: Instant,
    interval: StdDuration,
    messages: Vec<String>,
}

impl ScheduleItem {
    pub fn get_interval(&self) -> Interval {
        Interval::new(self.at, self.interval)
    }

    pub fn get_random_message(&self) -> Option<String> {
        self.messages.choose(&mut thread_rng()).cloned()
    }
}

#[derive(Debug, Default)]
pub struct ScheduleStore {
    items: Vec<ScheduleItem>,
}

impl ScheduleStore {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut f = File::open(path)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        let raw_items: Vec<RawScheduleItem> = serde_yaml::from_slice(&buf)?;
        let mut store = Self::default();
        let item_factory = ScheduleItemFactory::new();
        for raw_item in raw_items {
            store.items.push(item_factory.create(raw_item));
        }
        Ok(store)
    }
}

impl IntoIterator for ScheduleStore {
    type Item = ScheduleItem;
    type IntoIter = VecIntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
