use hello::cross_line::*;

fn main() {
    let n = 10;
    let m = n;
    let cell = ball_in_box(n, m);
    let set = count_cross(n, cell.as_ref());
    println!("topology count = {}", cell.len());
    println!("cross point = {:?}, count = {}", set, set.len());
}
