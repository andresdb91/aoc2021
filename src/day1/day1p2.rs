pub fn get_sliding_window_increase(data: std::str::Lines) -> u32 {
    let depths: Vec<u32> = data.map(|x| x.parse().unwrap()).collect();
    depths.windows(3)
    .map(|x| (x.iter().sum::<u32>(), 0))
    .reduce(|(x, d), (y, _)| {
        if y > x {
            (y, d + 1)
        } else {
            (y, d)
        }
    }).unwrap().1
}
