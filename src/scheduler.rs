use crate::context::Context;
use carapax::{
    methods::SendMessage,
    types::{Integer, ParseMode},
    Api,
};
use chrono::{DateTime, Datelike, Duration as ChronoDuration, FixedOffset, NaiveTime, Utc, Weekday};
use rand::{seq::SliceRandom, thread_rng};
use std::{error::Error, fmt, str::FromStr};
use tokio::time::{interval_at, Instant, Interval};
use tokio_postgres::Error as PostgresError;

const TZ_OFFSET: i32 = 3600 * 3;

pub struct Scheduler {
    context: Context,
}

impl Scheduler {
    pub fn new(context: Context) -> Self {
        Self { context }
    }

    pub async fn spawn(self) -> Result<(), SchedulerError> {
        let task_factory = TaskFactory::new(self.context.api, self.context.config.chat_id);
        for row in self
            .context
            .pg_client
            .query("SELECT day, time, messages FROM schedule", &[])
            .await
            .map_err(SchedulerError::GetSchedule)?
        {
            let task = task_factory.create(ScheduleItem {
                day: row.get::<_, String>(0).parse()?,
                time: row.get(1),
                messages: row.get(2),
            })?;
            tokio::spawn(task.run());
        }
        Ok(())
    }
}

#[derive(Debug)]
struct ScheduleItem {
    day: Day,
    time: NaiveTime,
    messages: Vec<String>,
}

#[derive(Debug)]
enum Day {
    Any,
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

impl FromStr for Day {
    type Err = UnknownDay;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw == "*" {
            Ok(Day::Any)
        } else {
            Ok(Day::Weekday(match raw {
                "mon" => Weekday::Mon,
                "tue" => Weekday::Tue,
                "wed" => Weekday::Wed,
                "thu" => Weekday::Thu,
                "fri" => Weekday::Fri,
                "sat" => Weekday::Sat,
                "sun" => Weekday::Sun,
                _ => return Err(UnknownDay(String::from(raw))),
            }))
        }
    }
}

#[derive(Debug)]
pub struct UnknownDay(String);

impl Error for UnknownDay {}

impl fmt::Display for UnknownDay {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "unknown day: {}", self.0)
    }
}

struct TaskFactory {
    api: Api,
    chat_id: Integer,
    instant: Instant,
    now: DateTime<FixedOffset>,
}

impl TaskFactory {
    fn new(api: Api, chat_id: Integer) -> Self {
        Self {
            api,
            chat_id,
            instant: Instant::now(),
            now: Utc::now().with_timezone(&FixedOffset::east(TZ_OFFSET)),
        }
    }

    fn create(&self, item: ScheduleItem) -> Result<Task, SchedulerError> {
        let current_weekday = self.now.weekday();
        let loop_interval = item.day.get_loop_interval();
        let start_interval = item.day.get_start_interval(current_weekday);
        let delay = match (self.now + start_interval).date().and_time(item.time) {
            Some(delay) => delay,
            None => return Err(SchedulerError::TaskDelay),
        };
        let delay = match delay.signed_duration_since(self.now).to_std() {
            Ok(v) => v,
            Err(_) => {
                let delay = delay + loop_interval;
                match delay.signed_duration_since(self.now).to_std() {
                    Ok(delay) => delay,
                    Err(_) => return Err(SchedulerError::TaskDelay),
                }
            }
        };
        Ok(Task {
            interval: interval_at(
                self.instant + delay,
                loop_interval.to_std().map_err(|_| SchedulerError::TaskInterval)?,
            ),
            messages: item.messages,
            api: self.api.clone(),
            chat_id: self.chat_id,
        })
    }
}

struct Task {
    interval: Interval,
    messages: Vec<String>,
    api: Api,
    chat_id: Integer,
}

impl Task {
    fn get_random_message(&self) -> Option<String> {
        self.messages.choose(&mut thread_rng()).cloned()
    }

    async fn run(mut self) {
        loop {
            self.interval.tick().await;
            if let Some(message) = self.get_random_message() {
                let method = SendMessage::new(self.chat_id, message).parse_mode(ParseMode::Html);
                if let Err(err) = self.api.execute(method).await {
                    log::error!("failed to send scheduled message: {}", err)
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum SchedulerError {
    GetSchedule(PostgresError),
    TaskDelay,
    TaskInterval,
    UnknownDay(UnknownDay),
}

impl From<UnknownDay> for SchedulerError {
    fn from(err: UnknownDay) -> Self {
        SchedulerError::UnknownDay(err)
    }
}

impl Error for SchedulerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SchedulerError::GetSchedule(err) => Some(err),
            SchedulerError::UnknownDay(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for SchedulerError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SchedulerError::GetSchedule(err) => write!(out, "unable to get schedule: {}", err),
            SchedulerError::TaskDelay => write!(out, "can not calculate delay for a task"),
            SchedulerError::TaskInterval => write!(out, "can not calculate interval for a task"),
            SchedulerError::UnknownDay(err) => write!(out, "{}", err),
        }
    }
}
