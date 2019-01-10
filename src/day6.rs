use super::tools;

pub fn do_thing()
{
    do_thing1();
}

struct Point {
    id: i16,
    x: u16,
    y: u16,
}

impl Point {
    fn new(x: u16, y: u16) -> Point {
        Point {
            id: -1,
            x: x,
            y: y,
        }
    }
    fn distance2(&self, other: &Point) -> u16 {
        let x: i16 = self.x as i16 - other.x as i16;
        let y: i16 = self.y as i16 - other.y as i16;
        x.abs() as u16 + y.abs() as u16
    }
    fn distance(&self, x: u16, y: u16) -> u16 {
        let x: i16 = self.x as i16 - x as i16;
        let y: i16 = self.y as i16 - y as i16;
        x.abs() as u16 + y.abs() as u16
    }
}

fn do_thing1()
{
    // total area: 350x350
    // for each point on map
    //  find closest "danger point"
    //  if equal, no point is closest
    //  mark point on map with that id or something
    // go through map and count area of ids
    // mark as invalid if it's "infinite" i.e. touching an edge
    // print the largest area
    
    let input_vec = tools::get_string_input("input/day6");
    let mut square = [[0u16; 350]; 350];
    let mut danger_points = Vec::<Point>::new();

    let mut it = 1 as i16;
    for line in input_vec {
        danger_points.push(Point{id: it,
                                 x: line.split(",").next().unwrap().parse::<u16>().expect("x"),
                                 y: line.split(",").last().unwrap().parse::<u16>().expect("y")});
        it += 1;
    }
    for i in 0..350 {
        for j in 0..350 {
            let mut closest = 0 as i16;
            for danger in &danger_points {
                let dist = danger.distance(i as u16, j as u16);
                if dist < closest {
                    
                }
            }
        }
    }
}