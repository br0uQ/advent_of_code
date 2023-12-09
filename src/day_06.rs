pub fn run_part(input: String, part: i8) {
    println!("==================================================");
    println!("====== Day 05         ============================");
    println!("==================================================");

    match part {
        1 => part1(input),
        2 => part2(input),
        _=> println!("What's happening...??!?!?"),
    }
}

fn part1(input: String) {
    println!("==================================================");
    println!("====== Part 1 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    println!("{}", input);
    println!("");

    let mut split = input.lines();

    let times = get_times(split.next().unwrap());
    let distances = get_distances(split.next().unwrap());

    println!("times: {:?}", times);
    println!("distances: {:?}", distances);

    let mut product: u64 = 1;
    for i in 0..times.len() {
        product = product * get_wins(times[i], distances[i]);
    }

    println!("product is {}", product);
}

fn part2(input: String) {
    println!("==================================================");
    println!("====== Part 2 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    println!("{}", input);

    let mut split = input.lines();

    let time = get_time(split.next().unwrap());
    let distance = get_distance(split.next().unwrap());

    println!("time: {:?}", time);
    println!("distance: {:?}", distance);

    println!("possible ways is {}", get_wins(time, distance));
}

fn get_time(line: &str) -> u64 {
    let mut str_time = String::from("");
    for n in line.split(':').last().unwrap().split_whitespace() {
        str_time = str_time + n;
    }

    return str_time.parse::<u64>().unwrap();
}

fn get_distance(line: &str) -> u64 {
    let mut str_distance = String::from("");
    for n in line.split(':').last().unwrap().split_whitespace() {
        str_distance = str_distance + n;
    }

    return str_distance.parse::<u64>().unwrap();
}

fn get_times(line: &str) -> Vec<u64> {
    let mut times: Vec<u64> = Vec::new(); 

    for n in line.split(':').last().unwrap().split_whitespace() {
        times.push(n.parse::<u64>().unwrap());
    }

    return times;
}

fn get_distances(line: &str) -> Vec<u64> {
    let mut distances: Vec<u64> = Vec::new(); 

    for n in line.split(':').last().unwrap().split_whitespace() {
        distances.push(n.parse::<u64>().unwrap());
    }

    return distances;
}

fn get_wins(time: u64, distance: u64) -> u64 {
    // x^2 - time*x + distance = 0
    // (time + sqrt(time^2 - 4*distance)) / 2
    let f_time = time as f64;
    let f_distance = distance as f64;
    let min = (f_time - f64::sqrt(f_time * f_time - 4. * f_distance)) / 2.;
    let mut min_rounded = min.ceil() as u64;
    
    if min == min.ceil() {
        min_rounded += 1;
    }

    let ret = time - (min_rounded * 2 - 1);
    println!("{} possible solutions", ret);
    return ret;
}
