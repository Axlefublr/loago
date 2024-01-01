use std::collections::HashMap;
use std::fmt;

use chrono::Duration;
use chrono::NaiveDateTime;
use chrono::Utc;

pub const APP_NAME: &str = "loago";

pub struct Tasks(HashMap<String, NaiveDateTime>);

impl From<HashMap<String, NaiveDateTime>> for Tasks {
    fn from(value: HashMap<String, NaiveDateTime>) -> Self {
        Self(value)
    }
}

impl TryFrom<HashMap<String, String>> for Tasks {
    type Error = chrono::format::ParseError;

    fn try_from(value: HashMap<String, String>) -> Result<Self, Self::Error> {
        let mut map = HashMap::new();
        for (key, timestamp) in value {
            let timestamp = timestamp.parse()?;
            map.insert(key, timestamp);
        }
        Ok(Tasks(map))
    }
}

impl From<Tasks> for HashMap<String, String> {
    fn from(value: Tasks) -> Self {
        value
            .0
            .into_iter()
            .map(|(key, timestamp)| (key, timestamp.to_string()))
            .collect()
    }
}

impl Tasks {
    pub fn update(&mut self, task: impl Into<String>) {
        self.0.insert(task.into(), now());
    }

    pub fn update_multiple(&mut self, tasks: impl IntoIterator<Item = impl Into<String>>) {
        let now = now();
        for task in tasks.into_iter() {
            self.0.insert(task.into(), now);
        }
    }

    pub fn remove(&mut self, task: &str) {
        self.0.remove(task);
    }

    pub fn remove_multiple(&mut self, tasks: &[impl AsRef<str>]) {
        for task in tasks {
            self.0.remove(task.as_ref());
        }
    }

    /// Assumes you're checking how long ago the tasks were done compared to *now*.
    /// The "how long ago" of every task is just the amount of days + the letter d.
    /// So if you did a task ten days ago, it would show up as `10d`.
    pub fn output_days(self) -> OutputTasks {
        self.output(|duration| format!("{}d", duration.num_days()))
    }

    /// Assumes you're checking how long ago the tasks were done compared to *now*.
    /// Convert the chrono::duration::Duration into a String representation of your choosing.
    pub fn output<F>(self, to_string: F) -> OutputTasks
    where
        F: Fn(Duration) -> String,
    {
        self.output_when(now(), to_string)
    }

    /// Allows to pass the chrono::naive::NaiveDateTime that will be considered "now".
    /// "now" is the date and time, compared to which the "how long ago" of tasks is compared to.
    /// Useful for testing and other applications I'm probably missing, which is why this is public.
    /// Convert the chrono::duration::Duration into a String representation of your choosing.
    pub fn output_when<F>(self, now: NaiveDateTime, to_string: F) -> OutputTasks
    where
        F: Fn(Duration) -> String,
    {
        type KeyToDuration = (String, Duration);
        let mut output: Vec<KeyToDuration> = self
            .0
            .into_iter()
            .map(|(key, timestamp)| (key, now - timestamp))
            .collect();
        output.sort_by_key(|(_, diff_days)| *diff_days);
        let output: Vec<KeyToDisplay> = output
            .into_iter()
            .map(|(key, duration)| (key, to_string(duration)))
            .collect();
        OutputTasks(output)
    }
}

fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

type KeyToDisplay = (String, String);

pub struct OutputTasks(Vec<KeyToDisplay>);

impl fmt::Display for OutputTasks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut length = 0;
        self.0.iter().for_each(|(task_name, _)| {
            let task_name_len = task_name.len();
            if task_name_len > length {
                length = task_name_len;
            }
        });
        let mut buffer = String::new();
        for (key, days_diff) in self.0.iter() {
            let whitespace = " ".repeat(length - key.len());
            buffer.push_str(key);
            buffer.push_str(&whitespace);
            buffer.push(' ');
            buffer.push('—');
            buffer.push(' ');
            buffer.push_str(&days_diff.to_string());
            buffer.push('\n')
        }
        write!(f, "{}", buffer)
    }
}

#[cfg(test)]
mod tasks {
    use std::collections::HashMap;

    use chrono::NaiveDate;
    use chrono::NaiveDateTime;

    use crate::now;
    use crate::Tasks;

    impl Tasks {
        fn same_days() -> Self {
            let mut map = HashMap::new();
            let december = december();
            map.insert(String::from("dust"), december);
            map.insert(String::from("vacuum"), december);
            map.insert(String::from("exercise"), december);
            Self(map)
        }

        fn different_days() -> Self {
            let mut map = HashMap::new();
            map.insert(String::from("dust"), november(1));
            map.insert(String::from("vacuum"), november(2));
            map.insert(String::from("exercise"), november(3));
            Self(map)
        }
    }

    fn december() -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2023, 12, 20)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }

    fn november(day: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2023, day, 20)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }

    #[test]
    fn update() {
        let mut tasks = Tasks::same_days();
        tasks.update("dust");
        let dust_ago = now() - tasks.0["dust"];
        assert_eq!(dust_ago.num_minutes(), 0);
    }

    #[test]
    fn update_multiple() {
        let mut tasks = Tasks::same_days();
        tasks.update_multiple(["vacuum", "dust"]);
        let vacuum_ago = now() - tasks.0["vacuum"];
        let dust_ago = now() - tasks.0["dust"];
        let exercise_ago = now() - tasks.0["exercise"];
        assert_eq!(vacuum_ago.num_minutes(), 0);
        assert_eq!(dust_ago.num_minutes(), 0);
        assert!(exercise_ago.num_days() > 0);
    }

    #[test]
    #[should_panic]
    fn remove() {
        let mut tasks = Tasks::same_days();
        tasks.remove("vacuum");
        let _ = tasks.0["vacuum"];
    }

    #[test]
    #[should_panic]
    fn remove_multiple() {
        let mut tasks = Tasks::same_days();
        tasks.remove_multiple(&["vacuum", "dust"]);
        let _ = tasks.0["vacuum"];
        let _ = tasks.0["dust"];
    }

    #[test]
    fn output_days() {
        let tasks = Tasks::same_days().output_days();
        let expected = format!("{}d", (now() - december()).num_days());
        for (_, actual) in tasks.0 {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn output_display() {
        let tasks =
            Tasks::different_days().output_when(december(), |duration| format!("{}d", duration.num_days()));
        let expected = String::from("exercise — 275d\nvacuum   — 303d\ndust     — 334d\n");
        let actual = tasks.to_string();
        assert_eq!(expected, actual);
    }
}
