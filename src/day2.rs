pub fn get_planned_course(data: &Vec<&str>) -> u32 {
    let depth = data.iter().map(|x| {
        let a: Vec<&str> = x.split(' ').collect();
        (a[0], a[1].parse::<u32>().unwrap())
    });
    1
}

pub fn get_fixed_course(data: &Vec<&str>) -> u32 {
    1
}
