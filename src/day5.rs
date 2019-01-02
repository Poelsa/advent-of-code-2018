use super::tools;
use regex::Regex;

pub fn do_thing()
{
    do_thing2();
}

const PAIRS: &str = 
"(Aa|Bb|Cc|Dd|Ee|Ff|Gg|Hh|Ii|Jj|Kk|Ll|Mm|Nn|Oo|Pp|Qq|Rr|Ss|Tt|Uu|Vv|Ww|Xx|Yy|Zz|aA|bB|cC|dD|eE|fF|gG|hH|iI|jJ|kK|lL|mM|nN|oO|pP|qQ|rR|sS|tT|uU|vV|wW|xX|yY|zZ)";

fn do_thing1()
{
    let input_vec = tools::get_string_input("input/day5");
    let mut input = input_vec.first().unwrap().clone();
    let re = Regex::new(PAIRS).unwrap();
    let mut flag = true;
    println!("{}", input.len());
    while flag {
        let s = input.clone();
        let tmp = re.replace_all(s.as_str(), "");
        if input == tmp {
            flag = false;
        }
        input = tmp.into_owned();
        //println!("{}", tmp);
        //println!("{}", input);
        println!("{}", input.len());
    }
    println!("{}", input.len());
}

fn do_thing2()
{
    let input_vec = tools::get_string_input("input/day5");
    let input = input_vec.first().unwrap().clone();
    let mut min = input.len() as u32;
    let upper: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    for mut c in upper {
        let mut sub = input.replace(c, "");
        sub = sub.replace(c.to_string().to_lowercase().chars().next().unwrap(), "");
        let tmp = reduce(sub);
        if tmp < min {
            min = tmp;
        }
    }
    println!("{}", min);
}

fn reduce(mut input: String) -> u32 {
    let re = Regex::new(PAIRS).unwrap();
    let mut flag = true;
    while flag {
        let s = input.clone();
        let tmp = re.replace_all(s.as_str(), "");
        if input == tmp {
            flag = false;
        }
        input = tmp.into_owned();
    }
    input.len() as u32
}