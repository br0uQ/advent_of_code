use std::collections::HashMap;

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

fn get_seeds(seeds_string: &str) -> Vec<u64> {
    let mut ret: Vec<u64> = Vec::new();

    let seeds_string = String::from(seeds_string).split_off(7);

    for n in seeds_string.split(' ') {
        ret.push(n.parse::<u64>().unwrap());
    }

    return ret;
}

fn create_hashmap(text: &str) -> HashMap<u64, u64> {
    let mut ret: HashMap<u64, u64> = HashMap::new();

    for line in text.lines() {
        let mut vector: Vec<u64> = Vec::new();
        if line == "" {
            continue;
        }

        for n in line.split(' ') {
            match n.parse::<u64>() {
                Ok(parsed) => {
                    vector.push(parsed);
                },
                Err(_) => {},
            }
        }

        for e in 0..vector[2] {
            ret.insert(vector[1] + e, vector[0] + e);
        }
    }

    return ret;
}

fn get_map_for(map_name: &str, seed_to_soil_string: &str) -> HashMap<u64, u64> {
    let mut i = seed_to_soil_string.split(':');

    println!("=== get map for: {}", map_name);

    if i.next().unwrap() == map_name {
        return create_hashmap(i.next().unwrap());
    }

    println!("ERROR: this is no {}", map_name);
    return HashMap::new();
}

struct Maps {
    seed_to_soil: HashMap<u64, u64>,
    soil_to_fertilizer: HashMap<u64, u64>,
    fertilizer_to_water: HashMap<u64, u64>,
    water_to_light: HashMap<u64, u64>,
    light_to_temperature: HashMap<u64, u64>,
    temperature_to_humidity: HashMap<u64, u64>,
    humidity_to_location: HashMap<u64, u64>,
}

fn get_loc_for_seed(seed: u64, maps: &Maps) -> u64 {
    let soil = match maps.seed_to_soil.get(&seed) {
        Some(s) => *s,
        None => seed,
    };
    let fertilizer = match maps.soil_to_fertilizer.get(&soil) {
        Some(s) => *s,
        None => soil,
    };
    let water = match maps.fertilizer_to_water.get(&fertilizer) {
        Some(s) => *s,
        None => fertilizer,
    };
    let light = match maps.water_to_light.get(&water) {
        Some(s) => *s,
        None => water,
    };
    let temp = match maps.light_to_temperature.get(&light) {
        Some(s) => *s,
        None => light,
    };
    let humid = match maps.temperature_to_humidity.get(&temp) {
        Some(s) => *s,
        None => temp,
    };
    let loc = match maps.humidity_to_location.get(&humid) {
        Some(s) => *s,
        None => humid,
    };

    println!("Seed {}, soil {}, fertilizer {}, water {}, light {}, temperature {}, humidity {}, location {}",
             seed, soil, fertilizer, water, light, temp, humid, loc);

    return loc;
}

fn part1(input: String) {
    println!("==================================================");
    println!("====== Part 1 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    println!("{}", input);

    let mut split = input.split("\n\n");

    if split.count() != 8 {
        println!("ERROR seams a map or the seeds are missing", );
        return;
    }

    split = input.split("\n\n");

    let seeds = get_seeds(split.next().unwrap());
    println!("seeds: {:?}", seeds);
    println!("");

    println!("=== create maps");
    let maps: Maps = Maps {
        seed_to_soil: get_map_for("seed-to-soil map", split.next().unwrap()),
        soil_to_fertilizer: get_map_for("soil-to-fertilizer map", split.next().unwrap()),
        fertilizer_to_water: get_map_for("fertilizer-to-water map", split.next().unwrap()),
        water_to_light: get_map_for("water-to-light map", split.next().unwrap()),
        light_to_temperature: get_map_for("light-to-temperature map", split.next().unwrap()),
        temperature_to_humidity: get_map_for("temperature-to-humidity map", split.next().unwrap()),
        humidity_to_location: get_map_for("humidity-to-location map", split.next().unwrap())
    };
    println!("");

    println!("=== parse locations");
    let mut locations: Vec<u64> = Vec::new();
    for seed in seeds {
        locations.push(get_loc_for_seed(seed, &maps));
    }
    println!("");

    locations.sort();
    println!("locations sorted: {:?}", locations);
}

fn part2(input: String) {
    println!("==================================================");
    println!("====== Part 2 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    println!("{}", input);

    let mut _sum = 0;

    println!("===> sum is {}", _sum);
}
