use regex::Regex;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use utils;

fn main() {
    let file = utils::open_argv1();
    let buffer = BufReader::new(file);

    // match a line of the form "[1518-10-09 00:43] wakes up"
    // groups are the timestamp string ("1518-10-09 00:43") and the message ("wakes up");
    // the timestamp string is further parsed by Timestamp::from_str.
    let line_re = Regex::new(r"^\[([\d: -]+)\] (.*)$").unwrap();

    // track activities as a map timestamp: message
    // using a BTreeMap will keep this in sorted order, so when we iterate next
    // it's in order by timestamp. I could have also stored this in a vector and
    // sorted it by TS, but I wanted to play with other data types :)
    let mut activity_log = BTreeMap::new();

    for line in buffer.lines() {
        let line = line.unwrap();
        let caps = line_re.captures(&line).expect("invalid line");

        // further parse the timestamp
        let ts_str = caps.get(1).unwrap().as_str();
        let timestamp = Timestamp::from_str(&ts_str);

        // and store the timestamp/message into the treemap
        let message = caps.get(2).unwrap().as_str();
        activity_log.insert(timestamp, message.to_owned().to_string());
    }

    // now activity_log is sorted by timestamp.
    // scan through it, recording each guard's sleep time

    // sleep tracker: map guard_id to a vector, where vec[i] represents sleepyness at minute i
    // eg if guard #123 was asleep at 00:23, sleep_tracker[123] += 1
    let mut sleep_tracker: HashMap<u32, Vec<u32>> = HashMap::new();

    // for part two - minute tracker
    // keys are the minute of the hour (0..60)
    // values are another HashMap, mapping guard_id -> number of minutes asleep
    let mut sleep_minute_tracker: HashMap<u8, HashMap<u32, u32>> = HashMap::new();

    // track the current guiard (or none), weather they're asleep or awake,
    // and when the guard fell asleep
    let mut cur_guard: Option<u32> = None;
    let mut fell_asleep_at: Option<u8> = None;

    for (timestamp, message) in activity_log {
        // "Guard #123 begins shift" --> record current guard, continue
        if message[..5] == String::from("Guard") {
            let guard_id_str = message
                .split_whitespace()
                .nth(1)
                .expect("invalid guard message");
            cur_guard = Some(guard_id_str[1..].parse().expect("invalid guard id"));

        // "falls alseep" --> record current sleep minute, continue
        } else if message == String::from("falls asleep") {
            fell_asleep_at = Some(timestamp.minute);

        // "woke up" --> guard woke up, record their asleep time
        // this requires recording as asleep each minute from fell_asleep_at to the current minute.
        } else if message == String::from("wakes up") {
            let waking_guard = cur_guard.expect("woke up but nio current guard");
            let start_minute = fell_asleep_at.expect("woke up but no fell_asleep_at");

            // get the tracking vector for this guard, making  sure this guard has an entry
            let cur_guards_sleep = sleep_tracker.entry(waking_guard).or_insert(vec![0; 60]);

            // track each minute they are asleep
            for minute in start_minute..timestamp.minute {
                cur_guards_sleep[minute as usize] += 1;

                let guards_minute = sleep_minute_tracker.entry(minute).or_insert(HashMap::new());
                *guards_minute.entry(waking_guard).or_insert(0) += 1;
            }
        }
    }

    // find the guard who's been most asleep

    // map total sleep minutes -> guard_id
    let mut sleepiness_map = HashMap::new();
    for (guard_id, sleep_vec) in &sleep_tracker {
        let total_sleep_minutes: u32 = sleep_vec.iter().sum();
        sleepiness_map.insert(total_sleep_minutes, guard_id);
    }

    // and who's the sleepiest?
    // XXX I don't entierly understand why I need the deref here
    let max_sleep: u32 = *sleepiness_map.keys().max().unwrap();
    let sleepiest_guard = sleepiness_map.get(&max_sleep).unwrap();
    println!(
        "sleepiest guard: #{} - {} minutes asleep",
        sleepiest_guard, max_sleep
    );

    // which minute was this guard asleep for the most amount of time?
    let sleepiest_guards_tracker = sleep_tracker.get(sleepiest_guard).unwrap();
    let max_sleeping_minutes: u32 = *sleepiest_guards_tracker.iter().max().unwrap();
    let sleepiest_minute = sleepiest_guards_tracker
        .iter()
        .position(|m| m == &max_sleeping_minutes)
        .unwrap();
    println!(
        "sleepiest minute: {} ({} minutes asleep)",
        sleepiest_minute, max_sleeping_minutes
    );

    // XXX part 2 here using data from sleep_minute_tracker
    // skipping for now because it's kinda tedius
    // println!("{:?}", sleep_minute_tracker);
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Timestamp {
    year: u32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl Timestamp {
    fn from_str(s: &str) -> Timestamp {
        let re = Regex::new(r"^(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})$").unwrap();
        let caps = re.captures(s).expect("invalid claim str");

        return Timestamp {
            year: caps.get(1).unwrap().as_str().parse().unwrap(),
            month: caps.get(2).unwrap().as_str().parse().unwrap(),
            day: caps.get(3).unwrap().as_str().parse().unwrap(),
            hour: caps.get(4).unwrap().as_str().parse().unwrap(),
            minute: caps.get(5).unwrap().as_str().parse().unwrap(),
        };
    }
}
