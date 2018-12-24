use super::tools;

pub fn do_thing()
{
    do_thing2();
}

#[derive(Debug)]
struct Rect {
    n: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

fn do_thing1()
{
    let input_vec = tools::get_string_input("input/day3");
    let mut square = [[0usize; 1000]; 1000];
    let mut tot_affected : usize = 0;

    for line in input_vec {
        let mut line_it = line.split_whitespace();
        let id = line_it.next();
        let mut pos = line_it.nth(1);
        let mut size = line_it.next();
        let rect = Rect { //Horrible horrible horrible horrible
            n: id.unwrap().trim_left_matches("#").parse::<usize>().expect("id"),
            x: pos.unwrap().split(",").next().unwrap().parse::<usize>().expect("x"),
            y: pos.unwrap().split(",").last().unwrap().trim_right_matches(":").parse::<usize>().expect("y"),
            w: size.unwrap().split("x").next().unwrap().parse::<usize>().expect("h"),
            h: size.unwrap().split("x").last().unwrap().trim().parse::<usize>().expect("w"),
        };
        tot_affected += update_square(&mut square,
                                      rect.x, rect.y, rect.w, rect.h);
    }
    println!("{}", tot_affected);
}

fn update_square(square: &mut [[usize; 1000]; 1000],
                 x: usize, y: usize,
                 w: usize, h: usize) -> usize {
    let mut tot = 0;
    for i in x..x+w {
        for j in y..y+h {
            square[i][j] += 1 as usize;
            if square[i][j] > 1 as usize {
                tot += 1 as usize;
            }
        }
    }
    return tot;
}

fn do_thing2()
{
    let input_vec = tools::get_string_input("input/day3");
    let mut square = [[0usize; 1000]; 1000];
    let mut set = Vec::<Rect>::new();

    for line in input_vec {
        let mut line_it = line.split_whitespace();
        let id = line_it.next();
        let mut pos = line_it.nth(1);
        let mut size = line_it.next();
        let rect = Rect { //Horrible horrible horrible horrible
            n: id.unwrap().trim_left_matches("#").parse::<usize>().expect("id"),
            x: pos.unwrap().split(",").next().unwrap().parse::<usize>().expect("x"),
            y: pos.unwrap().split(",").last().unwrap().trim_right_matches(":").parse::<usize>().expect("y"),
            w: size.unwrap().split("x").next().unwrap().parse::<usize>().expect("h"),
            h: size.unwrap().split("x").last().unwrap().trim().parse::<usize>().expect("w"),
        };
        let ret = update_square(&mut square, rect.x, rect.y, rect.w, rect.h);
        if ret == 0 {
            set.push(rect);
        }
        else {
            for i in (0..set.len()).rev() {
                if intersects(set.get(i).unwrap(), &rect) {
                    set.remove(i);
                }
            }
        }
    }
    println!("{}", set.first().unwrap().n);
}

fn intersects(exi: &Rect, new: &Rect) -> bool {
    return exi.x <= new.x+new.w-1 && exi.x+exi.w-1 >= new.x &&
           exi.y <= new.y+new.h-1 && exi.y+exi.h-1 >= new.y;
}