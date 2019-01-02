use super::tools;

pub fn do_thing()
{
    do_thing1();
}

struct Point {
    id: u16,
    x: u16,
    y: u16,
}

impl Point {
    fn distance(&self, other: &Point) -> u16 {
        let x: i16 = self.x as i16 - other.x as i16;
        let y: i16 = self.y as i16 - other.y as i16;
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
}