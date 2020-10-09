use std::io;
use std::vec;
use std::iter;
fn readLine() -> String {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Something went wrong while reading");
    return buf;
}
struct DayData {
    ax: usize,
    bx: usize,
    ay: usize,
    by: usize,
}

impl DayData {
    fn is_point_inside(&self, x: usize, y: usize) -> bool {
        return x >= self.ax && y >= self.ay && x <= self.bx && y <= self.by;
    }
    fn intersects(&self, other: &DayData) -> bool {
        return self.is_point_inside(other.ax, other.ay) ||
        self.is_point_inside(other.bx, other.by) ||
        other.is_point_inside(self.ax, self.ay) ||
        other.is_point_inside(self.bx, self.by); 
    }
    fn area(&self) -> usize {
        return (self.by - self.ay + 1) * (1 + self.bx - self.ax);
    }
}

type Mosaic = Vec<Vec<u32>>;

fn readDay() -> DayData {
    let digits: Vec<usize> = readLine().split(" ").map(|str| str.trim().parse().expect("Parsing went wrong")).collect();
    return DayData {
        ax: digits[0],
        ay: digits[1],
        bx: digits[2],
        by: digits[3]
    }
}
fn main() {
    let dimensions: Vec<u32> = readLine()
        .split(" ")
        .into_iter()
        .map(|str| str.trim().parse().expect("Parsing went wrong"))
        .collect();

    let days: usize = readLine().trim().parse().expect("Parsing days error");

    let actions: Vec<DayData> = iter::repeat_with(|| readDay()).take(days).collect();
    // let mut mosaic: Mosaic  = iter::repeat(iter::repeat(0).take(width).collect()).take(height).collect();

    let mut total_area = 0;
    let mut workedDays: Vec<usize> = vec![];

    for dayNr in (0..days) {
        let mut intersects = false;
        for workedDay in &workedDays {
            if actions[*workedDay].intersects(&actions[dayNr]) {
                intersects = true;
                break;
            }
        }
        if !intersects {
            workedDays.push(dayNr);
            total_area += actions[dayNr].area();
        }
        println!("{}", total_area);
    }
}
