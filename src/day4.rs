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

struct Answer {
    id: usize,
    g_max: usize,
    l_max: usize,
    minute: usize,
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
        match &line.chars().nth(19).unwrap() {
            'w' => {
                entry.woke.push(re_date.captures(line.as_str()).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap());
            }
            'f' => {
                entry.slept.push(re_date.captures(line.as_str()).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap());
            }
            'G' => {
                entry.id = re_id.captures(line.as_str()).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
            },
            x => println!("{}", x),
        }
    }
    println!("{}", map.len());
    let mut guard_map = HashMap::<usize, [usize; 60]>::new();
    for (_d, mut e) in map {
        e.slept.sort_unstable();
        e.woke.sort_unstable();
        let mut guard = guard_map.entry(e.id).or_insert([0usize; 60]);
        for i in 0..e.slept.len() {
            for t in e.slept[i]..e.woke[i] {
                guard[t] += 1;
            }
        }
    }
    let mut a = Answer {id: 0, g_max: 0, l_max: 0, minute: 0};
    println!("{}", guard_map.len());
    for (id, arr) in guard_map {
        //if arr.iter().sum::<usize>() > a.g_max { // Part 1
        if *arr.iter().max().unwrap() > a.l_max { // Part 2
            a.id = id;
            a.g_max = arr.iter().sum();
            a.l_max = *arr.iter().max().unwrap();
            a.minute = arr.iter().position(|&x| x == a.l_max).unwrap();
        }
    }
    println!("ID: {}\nTotal: {}\nMinute: {}", a.id, a.g_max, a.minute);
    println!("Answer: {}", a.minute*a.id);
}

fn get_rounded_date(date_string: &str) -> NaiveDate {
    //println!("{}", date_string);
    let date_time = NaiveDateTime::parse_from_str(date_string, "%Y-%m-%d %H:%M").unwrap();
    let mut ret_date = date_time.date();
    if date_time.time().hour() == 23 {
        ret_date += Duration::days(1);
    }
    return ret_date;
}