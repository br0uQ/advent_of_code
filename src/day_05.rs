use std::time::Instant;

struct Mapper {
    name: MapNames,
    map: Vec<[u64; 3]>,
}

impl Mapper {
    pub fn get_value(&mut self, key: u64) -> u64 {
        let last = self.map.last().unwrap();
        let first = self.map.first().unwrap();
        if key > last[0] + last[2] - 1 {
            return key;
        } else if key < first[0] {
            return key;
        }
        for m in &self.map {
            if key < m[0] {
                return key;
            }
            if key < (m[0] + m[2] - 1) {
                return m[1] + key - m[0];
            }
        }
        return key;
    }

    pub fn parse_map(&mut self, text: &str) {
        let mut i = text.split(':');
        let map_name_str = get_mapname_str(&self.name);

        println!("=== get map for: {}", map_name_str);

        if i.next().unwrap() == map_name_str {
            for line in i.next().unwrap().lines() {
                let mut vector: [u64; 3] = [0; 3];
                if line == "" {
                    continue;
                }

                let mut split = line.split(' ');
                vector[1] = split.next().unwrap().parse::<u64>().unwrap();
                vector[0] = split.next().unwrap().parse::<u64>().unwrap();
                vector[2] = split.next().unwrap().parse::<u64>().unwrap();

                self.map.push(vector);
            }
            self.map.sort();
            for m in &self.map {
                println!("{:?}", m);
            }
        } else {
            println!("ERROR: this is no {}", map_name_str);
        }
    }

    pub fn new(name: MapNames) -> Mapper {
        Mapper { name: name, map: Vec::new() }
    }
}

struct FullMap {
    maps: Vec<Mapper>,
}

impl FullMap {
    pub fn get_loc_for_seed(&mut self, seed: u64) -> u64 {
        let mut key = seed;
        //let count = self.maps.len();
        for map in &mut self.maps {
            key = map.get_value(key);
        }
        //for i in 0..count {
        //    key = self.maps[i].get_value(key);
        //}

        return key;
    }
}

enum MapNames {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

fn get_mapname_str(map_name: &MapNames) -> String {
    match map_name {
        MapNames::SeedToSoil => String::from("seed-to-soil map"),
        MapNames::SoilToFertilizer => String::from("soil-to-fertilizer map"),
        MapNames::FertilizerToWater => String::from("fertilizer-to-water map"),
        MapNames::WaterToLight => String::from("water-to-light map"),
        MapNames::LightToTemperature => String::from("light-to-temperature map"),
        MapNames::TemperatureToHumidity => String::from("temperature-to-humidity map"),
        MapNames::HumidityToLocation => String::from("humidity-to-location map"),
    }
}

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

fn get_loc_for_seed(seed: u64, maps: &mut Vec<Mapper>) -> u64 {
    let mut key = seed;
    let mut value;
    for m in maps {
        value = m.get_value(key);
        key = value;
    }

    return key;
}

fn parse_seeds_and_maps(input: String) -> (Vec<Mapper>, Vec<u64>) {
    let seeds;
    let mut map;
    let mut maps: Vec<Mapper> = Vec::new();
    let mut split = input.split("\n\n");

    if split.count() != 8 {
        println!("ERROR seams a map or the seeds are missing", );
    }

    split = input.split("\n\n");

    seeds = get_seeds(split.next().unwrap());
    println!("seeds: {:?}", seeds);
    println!("");

    println!("=== create maps");
    map = Mapper::new(MapNames::SeedToSoil);
    map.parse_map(split.next().unwrap());
    maps.push(map);
    map = Mapper::new(MapNames::SoilToFertilizer);
    map.parse_map(split.next().unwrap());
    maps.push(map);
    map = Mapper::new(MapNames::FertilizerToWater);
    map.parse_map(split.next().unwrap());
    maps.push(map);
    map = Mapper::new(MapNames::WaterToLight);
    map.parse_map(split.next().unwrap());
    maps.push(map);
    map = Mapper::new(MapNames::LightToTemperature);
    map.parse_map(split.next().unwrap());
    maps.push(map);
    map = Mapper::new(MapNames::TemperatureToHumidity);
    map.parse_map(split.next().unwrap());
    maps.push(map);
    map = Mapper::new(MapNames::HumidityToLocation);
    map.parse_map(split.next().unwrap());
    maps.push(map);

    return (maps, seeds);
}

fn part1(input: String) {
    println!("==================================================");
    println!("====== Part 1 ====================================");
    println!("==================================================");
    println!("=== Input: ===");
    println!("{}", input);
    println!("");

    let seeds;
    let mut maps;

    (maps, seeds) = parse_seeds_and_maps(input);

    println!("=== parse locations");
    let mut locations: Vec<u64> = Vec::new();
    for seed in seeds {
        locations.push(get_loc_for_seed(seed, &mut maps));
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

    let seeds;
    let maps;

    (maps, seeds) = parse_seeds_and_maps(input);

    let mut full_map = FullMap {
        maps: maps,
    };

    println!("=== parse locations");
    let mut location: u64;
    let mut location_final = full_map.get_loc_for_seed(seeds[0]);
    println!("parsing seed_array[{}]", seeds.len());
    for i in 0..seeds.len() {
        if (i%2) == 0 {
            println!("parsing seeds[{}] beginning with {}", seeds[i+1], seeds[i]);
            let now = Instant::now();
            for e in 0..seeds[i+1] {
                location = full_map.get_loc_for_seed(seeds[i] + e);

                if location < location_final {
                    location_final = location;
                }
            }
            let elapsed = now.elapsed();
            println!("took: {:.2?}", elapsed);
            println!("lowest location is {}", location_final);
        }
    }
    println!("");

    //locations.sort();
    println!("lowest location is: {}", location_final);
}
