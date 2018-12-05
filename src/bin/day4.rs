extern crate regex;
use std::collections::HashMap;
use std::mem::drop;
static LOG_ENTRIES: &str = include_str!("../../inputs/day4.txt");

#[derive(Debug,PartialEq, Eq, PartialOrd, Ord)]
enum Event {
    BeginsShift(usize),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Entry<'a> {
    date: &'a str,
    hour: u8,
    minute: u8,
    event: Event,
}

fn load_entries<'a>(entries_src: &'a str) -> Vec<Entry<'a>> {
    let re = regex::Regex::new(r#"\[(\d{4}-\d{2}-\d{2}) (\d{2}):(\d{2})\] (.+)"#).unwrap();
    let extract_id_re = regex::Regex::new(r"Guard #(\d+) begins shift").unwrap();
    let mut entries = vec![];
    for line in entries_src.lines() {
        let caps = re.captures(line).unwrap();
        let date = caps.get(1).unwrap().as_str();
        let hour = caps.get(2).unwrap().as_str().parse().unwrap();
        let minute = caps.get(3).unwrap().as_str().parse().unwrap();
        let event = match caps.get(4).unwrap().as_str() {
            "falls asleep" => Event::FallsAsleep,
            "wakes up" => Event::WakesUp,
            x => {
                let gaurd_id = extract_id_re.captures(x).unwrap().get(1).unwrap().as_str().parse().unwrap();
                Event::BeginsShift(gaurd_id)
            }
        };
        let entry = Entry {
            date,
            hour,
            minute,
            event,
        };
        entries.push(entry);
    }
    entries
}

fn get_sleep_times<'a>(entries: &Vec<Entry<'a>>) -> HashMap<usize, HashMap<u8, usize>> {
    let mut sleep_times = HashMap::new();
    let mut gaurd_id = 0;
    let mut sleep_start = 0;
    for entry in entries {
        match entry.event {
            Event::BeginsShift(id) => {
                gaurd_id = id;
            },
            Event::FallsAsleep => {
                sleep_start = entry.minute;
            },
            Event::WakesUp => {
                let sleep_end = entry.minute;
                let mut map = sleep_times.entry(gaurd_id).or_insert_with(HashMap::new);
                for minute in sleep_start..sleep_end {
                    *map.entry(minute).or_insert(0) += 1;
                }
            }

        }
    }
    sleep_times
}

fn find_gaurd_who_slept_the_most(sleep_times: &HashMap<usize, HashMap<u8, usize>>) -> usize {
    let mut gaurd_who_slept_the_most = 0;
    let mut most_time_asleep = 0;
    for (gaurd_id, gaurd_sleep_times) in sleep_times.iter() {
        let time_asleep = gaurd_sleep_times.values().sum();
        if time_asleep > most_time_asleep {
            most_time_asleep = time_asleep;
            gaurd_who_slept_the_most = *gaurd_id;
        }
    }
    gaurd_who_slept_the_most
}

fn find_minute_most_slept_in(sleeping_minutes: &HashMap<u8, usize>) -> u8 {
    *sleeping_minutes.iter().max_by_key(|&(_, time)| time).unwrap().0
}

fn part1<'a>(entries: &Vec<Entry<'a>>) {
    let sleep_times = get_sleep_times(entries);
    let gaurd_who_slept_the_most = find_gaurd_who_slept_the_most(&sleep_times);
    println!("Gaurd #{} slept the most", gaurd_who_slept_the_most);
    let minute_most_slept_in = find_minute_most_slept_in(sleep_times.get(&gaurd_who_slept_the_most).unwrap());
    println!("Minute most slept: {}", minute_most_slept_in);
    println!("Result: {}", gaurd_who_slept_the_most * (minute_most_slept_in as usize));
}

fn part2<'a>(entries: &Vec<Entry<'a>>) {
    let sleep_times = get_sleep_times(entries);
    let mut minutes = HashMap::new();
    for i in 0..60 {
        for (gaurd_id, gaurd_sleep_times) in sleep_times.iter() {
            let mut x = minutes.entry(i).or_insert((0, 0)); // (gaurd_id, number_of_times_asleep)
            if let Some(time) = gaurd_sleep_times.get(&i) {
                if time > &x.1 {
                    *x = (*gaurd_id, *gaurd_sleep_times.get(&i).unwrap());
                }
            }
            drop(x);
        }
    }
    let (minute, (gaurd_id, _)) = minutes.iter().max_by_key(|(_, (_, x))| x).unwrap();
    println!("{}", *gaurd_id * (*minute as usize));
}


fn main() {
    let mut entries = load_entries(LOG_ENTRIES);
    entries.sort();
    part1(&entries);
    part2(&entries);
}