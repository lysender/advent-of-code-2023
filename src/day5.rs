
use std::{collections::HashMap, ops::Range};
use indicatif::ProgressIterator;

#[derive(Clone)]
pub enum MapType {
    Soil,
    Fertilizer,
    Water,
    Light,
    Temp,
    Humidity,
    Location,
}

#[derive(Clone)]
pub struct MapLine {
    pub dest: Range<u64>,
    pub source: Range<u64>,
}

#[derive(Clone)]
pub struct Almanac {
    pub seeds: Vec<u64>,
    pub mapping: HashMap<String, Vec<MapLine>>,
}

pub fn part1(input: &str) -> u64 {
    let almanac = parse_input(input);
    let mut closest_location: Option<u64> = None;

    for seed in almanac.seeds.iter() {
        let soil = find_mapped_value(&almanac.mapping, &MapType::Soil, *seed);
        let fertilizer = find_mapped_value(&almanac.mapping, &MapType::Fertilizer, soil);
        let water = find_mapped_value(&almanac.mapping, &MapType::Water, fertilizer);
        let light = find_mapped_value(&almanac.mapping, &MapType::Light, water);
        let temp = find_mapped_value(&almanac.mapping, &MapType::Temp, light);
        let humidity = find_mapped_value(&almanac.mapping, &MapType::Humidity, temp);
        let location = find_mapped_value(&almanac.mapping, &MapType::Location, humidity);

        if closest_location.is_none() {
            closest_location = Some(location);
        } else if location < closest_location.unwrap() {
            closest_location = Some(location);
        }
    }

    closest_location.unwrap()
}

pub fn part2(input: &str) -> u64 {
    // part2_orig(input)
    part2_reversed(input)
}

pub fn part2_orig(input: &str) -> u64 {
    let almanac = parse_input(input);
    let mut closest_location: Option<u64> = None;

    for chunks in almanac.seeds.chunks(2).progress() {
        let seed_start = chunks[0] as usize;
        let seed_range: Range<usize> = Range { start: seed_start, end: seed_start + chunks[1] as usize };
        for seed_value in seed_range {
            let soil = find_mapped_value(&almanac.mapping, &MapType::Soil, seed_value as u64);
            let fertilizer = find_mapped_value(&almanac.mapping, &MapType::Fertilizer, soil);
            let water = find_mapped_value(&almanac.mapping, &MapType::Water, fertilizer);
            let light = find_mapped_value(&almanac.mapping, &MapType::Light, water);
            let temp = find_mapped_value(&almanac.mapping, &MapType::Temp, light);
            let humidity = find_mapped_value(&almanac.mapping, &MapType::Humidity, temp);
            let location = find_mapped_value(&almanac.mapping, &MapType::Location, humidity);

            if closest_location.is_none() {
                closest_location = Some(location);
            } else if location < closest_location.unwrap() {
                closest_location = Some(location);
            }
        }
    }

    closest_location.unwrap()
}

fn part2_reversed(input: &str) -> u64 {
    let almanac = parse_input(input);
    let mut seed_ranges: Vec<(usize, usize)> = Vec::new();
    for chunk in almanac.seeds.chunks(2) {
        seed_ranges.push((chunk[0] as usize, chunk[1] as usize));
    }

    let mut closest_location: Option<u64> = None;

    // Find the farthest location to build a zero to high range
    let loc_type = map_type_to_string(&MapType::Location);
    if let Some(locations) = almanac.mapping.get(&loc_type) {
        if locations.len() > 0 {
            let mut sorted_locations = locations.clone();
            sorted_locations.sort_by(|a, b| {
                b.dest.end.cmp(&a.dest.end)
            });
            let high = sorted_locations[0].dest.end as usize;
            let range = 0..high;

            for location in range.progress() {
                let loc: u64 = location.try_into().unwrap();
                let humidity = find_mapped_value_reversed(&almanac.mapping, &MapType::Location, loc);
                let temp = find_mapped_value_reversed(&almanac.mapping, &MapType::Humidity, humidity);
                let light = find_mapped_value_reversed(&almanac.mapping, &MapType::Temp, temp);
                let water = find_mapped_value_reversed(&almanac.mapping, &MapType::Light, light);
                let fertilizer = find_mapped_value_reversed(&almanac.mapping, &MapType::Water, water);
                let soil = find_mapped_value_reversed(&almanac.mapping, &MapType::Fertilizer, fertilizer);
                let seed = find_mapped_value_reversed(&almanac.mapping, &MapType::Soil, soil);
                let thinner_seed: usize = seed.try_into().unwrap();

                if seed_exists(&seed_ranges, thinner_seed) {
                    if closest_location.is_none() {
                        closest_location = Some(loc);
                    } else if loc < closest_location.unwrap() {
                        closest_location = Some(loc);
                    }
                }
            }
        }
    }

    if let Some(loc) = closest_location {
        return loc;
    }
    return 0;
}

fn seed_exists(pairs: &Vec<(usize, usize)>, seed_value: usize) -> bool {
    for pair in pairs.iter() {
        let start = pair.0;
        let end = start + pair.1;
        let range = start..end;
        if range.contains(&seed_value) {
            return true;
        }
    }
    false
}

fn map_type_to_string(map_type: &MapType) -> String {
    match map_type {
        MapType::Soil => String::from("soil"),
        MapType::Fertilizer => String::from("fertilizer"),
        MapType::Water => String::from("water"),
        MapType::Light => String::from("light"),
        MapType::Temp => String::from("temp"),
        MapType::Humidity => String::from("humidity"),
        MapType::Location => String::from("location"),
    }
}

fn find_mapped_value(mapping: &HashMap<String, Vec<MapLine>>, dest_type: &MapType, source_value: u64) -> u64 {
    let dest_type_str = map_type_to_string(dest_type);
    if let Some(maps) = mapping.get(&dest_type_str) {
        for map in maps.iter() {
            if map.source.contains(&source_value) {
                // Find the distance from the start of the source range
                let distance = source_value - map.source.start;
                return map.dest.start + distance;
            }
        }
    }
    source_value
}

fn find_mapped_value_reversed(mapping: &HashMap<String, Vec<MapLine>>, dest_type: &MapType, dest_value: u64) -> u64 {
    let source_type_str = map_type_to_string(dest_type);
    if let Some(maps) = mapping.get(&source_type_str) {
        for map in maps.iter() {
            if map.dest.contains(&dest_value) {
                // Find the distance from the start of the dest range
                let distance = dest_value - map.dest.start;
                return map.source.start + distance;
            }
        }
    }
    dest_value
}

fn parse_input(input: &str) -> Almanac {
    let mut seeds: Vec<u64> = Vec::new();
    let mut mapping: HashMap<String, Vec<MapLine>> = HashMap::new();
    let mut map_type: Option<MapType> = None;
    let mut map_buffer: Vec<MapLine> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            // Seeds
            let line_chunks: Vec<&str> = line.split(": ").collect();
            if let Some(seeds_line) = line_chunks.get(1) {
                for seed in seeds_line.split(" ") {
                    let seed_str = seed.to_string();
                    let seed_num: u64 = seed_str.parse::<u64>().unwrap();
                    seeds.push(seed_num);
                }
            }
        } else {
            // Identify if this is a header, a line entry or a group break
            if line.ends_with("map:") {
                // Flush buffer
                let mt = map_type.clone();
                let mb = map_buffer.clone();
                if mt.is_some() && mb.len() > 0 {
                    mapping.insert(map_type_to_string(&mt.unwrap()), mb);
                }

                // Empty buffer to start collecting new values
                map_buffer = Vec::new();

                // Identify the new header type
                let header_chunks: Vec<&str> = line.split(" ").collect();
                if header_chunks.len() == 2 {
                    map_type = match header_chunks[0] {
                        "seed-to-soil" => {
                            Some(MapType::Soil)
                        },
                        "soil-to-fertilizer" => {
                            Some(MapType::Fertilizer)
                        },
                        "fertilizer-to-water" => {
                            Some(MapType::Water)
                        },
                        "water-to-light" => {
                            Some(MapType::Light)
                        },
                        "light-to-temperature" => {
                            Some(MapType::Temp)
                        },
                        "temperature-to-humidity" => {
                            Some(MapType::Humidity)
                        },
                        "humidity-to-location" => {
                            Some(MapType::Location)
                        },
                        _ => {
                            None
                        }
                    }
                }
            } else if line.len() == 0 {
                // Flush buffer
                let mt = map_type.clone();
                let mb = map_buffer.clone();
                if mt.is_some() && mb.len() > 0 {
                    mapping.insert(map_type_to_string(&mt.unwrap()), mb);
                }
                map_type = None;
                map_buffer = Vec::new();
            } else {
                // Add item to the map
                let numbers: Vec<u64> = line.split(" ").map(|x| {
                    let str = x.to_string();
                    let num: u64 = str.parse::<u64>().unwrap();
                    num
                }).collect();

                if numbers.len() == 3 {
                    map_buffer.push(MapLine {
                        dest: numbers[0]..(numbers[0] + numbers[2]),
                        source: numbers[1]..(numbers[1] + numbers[2]),
                    });
                }
            }
        } 
    }

    // Flush buffer if there are still items
    let mt = map_type.clone();
    let mb = map_buffer.clone();
    if mt.is_some() && mb.len() > 0 {
        //almanac.set_map_values(mt.unwrap(), mb);
        mapping.insert(map_type_to_string(&mt.unwrap()), mb);
    }

    Almanac { seeds, mapping }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        // Test individual lookups
        let almanac = parse_input(input);
        assert_eq!(find_mapped_value(&almanac.mapping, &MapType::Soil, 79), 81);
        assert_eq!(find_mapped_value(&almanac.mapping, &MapType::Soil, 14), 14);
        assert_eq!(find_mapped_value(&almanac.mapping, &MapType::Soil, 55), 57);
        assert_eq!(find_mapped_value(&almanac.mapping, &MapType::Soil, 13), 13);
        assert_eq!(find_mapped_value(&almanac.mapping, &MapType::Soil, 53), 55);

        // Test sequential lookups
        assert_eq!(find_mapped_value(&almanac.mapping, &MapType::Fertilizer, 81), 81);
        assert_eq!(find_mapped_value(&almanac.mapping, &MapType::Water, 81), 81);
        assert_eq!(find_mapped_value(&almanac.mapping, &MapType::Light, 81), 74);
        assert_eq!(find_mapped_value(&almanac.mapping, &MapType::Temp, 74), 78);
        assert_eq!(find_mapped_value(&almanac.mapping, &MapType::Humidity, 78), 78);
        assert_eq!(find_mapped_value(&almanac.mapping, &MapType::Location, 78), 82);

        // Test final output
        let result = part1(input);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = part2_orig(input);
        assert_eq!(result, 46);

        // Try the reversed method
        let almanac = parse_input(input);
        assert_eq!(find_mapped_value_reversed(&almanac.mapping, &MapType::Location, 82), 78);
        assert_eq!(find_mapped_value_reversed(&almanac.mapping, &MapType::Humidity, 78), 78);
        assert_eq!(find_mapped_value_reversed(&almanac.mapping, &MapType::Temp, 78), 74);
        assert_eq!(find_mapped_value_reversed(&almanac.mapping, &MapType::Light, 74), 81);
        assert_eq!(find_mapped_value_reversed(&almanac.mapping, &MapType::Water, 81), 81);
        assert_eq!(find_mapped_value_reversed(&almanac.mapping, &MapType::Fertilizer, 81), 81);
        assert_eq!(find_mapped_value_reversed(&almanac.mapping, &MapType::Soil, 81), 79);

        let result2 = part2_reversed(input);
        assert_eq!(result2, 46);
    }
}
