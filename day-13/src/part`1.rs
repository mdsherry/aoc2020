static START: usize = 1002632;
static BUS_TIMES: &str = "23,x,x,x,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,829,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,677,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,19";
fn main() {
    let bus_times: Vec<_> = BUS_TIMES.split(',').filter(|s| *s != "x").map(|s| s.parse::<usize>().unwrap()).collect();
    let first_time = bus_times.iter().copied().min_by_key(|t| dbg!(t - START % *t)).unwrap();
    println!("{} * {} = {}", first_time, START % first_time, (first_time - START % first_time) * first_time);
}
