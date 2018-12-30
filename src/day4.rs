use super::tools;
use std::collections::HashMap;
use chrono::{Timelike, NaiveDateTime, NaiveDate, Duration};
use regex::Regex;

pub fn do_thing()
{
    do_thing1();
}

struct DateEntry {
    id: usize,
    slept: Vec<usize>,
    woke: Vec<usize>,
}

impl DateEntry {
    fn new() -> DateEntry {
        DateEntry {id: 0,
                   slept: Vec::<usize>::new(),
                   woke: Vec::<usize>::new(),
                  }
    }
}

struct Guard { // Use map <id, array> instead?
    id: usize,
    sleeping: [usize; 60],
}

fn do_thing1()
{
    let input_vec = tools::get_string_input("input/day4");
    let mut map = HashMap::<NaiveDate, DateEntry>::new();
    let re_date = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:(\d{2})").unwrap();
    let re_id = Regex::new(r"#(\d*) ").unwrap();
    for line in input_vec {
        let date = get_rounded_date(re_date.find(line.as_str()).unwrap().as_str());
        let entry = map.entry(date).or_insert(DateEntry::new());
        match &line[19..19] {
            "w" => entry.woke.push(re_date.captures(line.as_str()).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap()),
            "f" => entry.slept.push(re_date.captures(line.as_str()).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap()),
            "G" => {
                entry.id = re_id.captures(line.as_str()).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
            },
            _ => ()
        }
    }
    // Loop through date map
    // sort woke and slept Vecs
    // add sleep time based on woke and slept minutes, per guard id
}

// Input 
fn get_rounded_date(date_string: &str) -> NaiveDate {
    //println!("{}", date_string);
    let date_time = NaiveDateTime::parse_from_str(date_string, "%Y-%m-%d %H:%M").unwrap();
    let mut ret_date = date_time.date();
    if date_time.time().hour() == 23 {
        ret_date += Duration::days(1);
    }
    return ret_date;
}

fn add_sleeping_time(arr: &mut [usize; 60], begin: usize, end: usize)
{
    for i in begin..end {
        arr[i] += 1;
    }
}

fn do_thing2()
{
    let input_vec = tools::get_string_input("input/day4");
}