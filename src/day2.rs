use super::tools;
use std::collections::HashMap;

pub fn do_thing()
{
    do_thing1();
}

fn do_thing1()
{
    let mut total2 : u32 = 0;
    let mut total3 : u32 = 0;
    let input_vec = tools::get_string_input("input/day2");

    for line in &input_vec {
        let mut map = HashMap::<char, u32>::new();
        for z in line.chars() {
            let counter = map.entry(z).or_insert(0);
            *counter += 1;
        }
        let mut has2 = false;
        let mut has3 = false;
        for (_, i) in map.iter() {
            if !has2 && *i == 2 as u32 {
                total2 += 1;
                has2 = true;
            }
            else if !has3 && *i == 3 as u32 {
                total3 += 1;
                has3 = false;
            }
        }
    }
    println!("2: {}, 3: {}", total2, total3);
    println!("{}", total2*total3);
}
/*
fn do_thing2()
{
    let mut freqs = Vec::<i32>::new();
    let mut set = HashSet::<i32>::new();
    let mut total : i32 = 0;
    set.insert(total);
    let input_vec = tools::get_string_input("input/day1");

    for line in &input_vec {
        freqs.push(line.parse::<i32>().expect("Hej"));
    }

    'outer: loop {
        for freq in &freqs {
            total += freq;
            if !set.insert(total) {
                break 'outer;
            }
        }
    }
    println!("{}", total);
}*/