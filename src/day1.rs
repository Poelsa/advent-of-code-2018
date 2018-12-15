use super::tools;
use std::collections::HashSet;

pub fn do_thing()
{
    do_thing2();
}

pub fn do_thing1()
{
    let mut total : i32 = 0;
    //Read input by 
    let input_vec = tools::get_string_input("input/day1");

    for line in &input_vec {
        if &line[..1] == "+" {
            total += line[1..].parse::<i32>().expect("Hej");
        }
        else if &line[..1] == "-" {
            total -= line[1..].parse::<i32>().expect("Hej");
        }
    }
    println!("{}", total);
}

pub fn do_thing2()
{
    let mut freqs = Vec::<i32>::new();
    let mut set = HashSet::<i32>::new();
    let mut total : i32 = 0;
    set.insert(total);
    //Read input by 
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
}