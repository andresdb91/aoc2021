pub fn get_depth_increase(data: std::str::Lines) -> u32 {
    data.map(
        |x| (x.parse::<u32>().unwrap(), 0))
        .reduce(|(x, d), (y, _)| {
            if y > x {
                (y, d + 1)
            } else {
                (y, d)
            }
        })
        .unwrap().1
}
