
pub struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

pub fn part1(input: &str, cube_set: CubeSet) -> u32 {
    get_total_games(input, cube_set)
}

pub fn part2(input: &str) -> u32 {
    get_sum_of_power(input)

}

fn get_total_games(input: &str, limits: CubeSet) -> u32 {
    // Cubes is a tupple of red, green and blue cubes
    // Value can be zero
    let mut total: u32 = 0;
    for line in input.lines() {
        // Each line contains the game ID and the sets
        // Number of sets may be arbitrary
        if line.len() > 0 {
            let game_info: Vec<&str> = line.split(": ").collect();
            if let Some(game_id) = extract_game_id(game_info[0]) {
                if game_info.get(1).is_some() {
                    let sets_str = game_info[1].trim();

                    let cube_sets = extract_game_sets(sets_str);
                    if within_limits(&limits, cube_sets) {
                        total += game_id;
                    }
                }
            }
        }
    }

    total
}

fn within_limits(limits: &CubeSet, sets: Vec<CubeSet>) -> bool {
    let mut result: bool = true;
    for set in sets.iter() {
        if set.red > limits.red {
            result = false;
            break;
        }
        if set.green> limits.green {
            result = false;
            break;
        }
        if set.blue > limits.blue {
            result = false;
            break;
        }
    }

    result
}

fn extract_game_id(line: &str) -> Option<u32> {
    let chunks: Vec<&str> = line.split(" ").collect();
    if chunks.len() == 2 {
        // The second chunk should be the game ID
        if let Some(digits) = chunks.get(1) {
            let digits_string = digits.to_string();
            let game_id: u32 = digits_string.parse::<u32>().unwrap();
            return Some(game_id);
        }
    }
    None
}

fn extract_game_sets(line: &str) -> Vec<CubeSet> {
    let mut cube_sets: Vec<CubeSet> = Vec::new();
    let set_lines: Vec<&str> = line.split("; ").collect();
    for set_line in set_lines.iter() {
        cube_sets.push(extract_game_set(set_line));
    }
    cube_sets 
}

fn extract_game_set(line: &str) -> CubeSet {
    let mut cube_set = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };

    let color_lines: Vec<&str> = line.split(", ").collect();
    for color_line in color_lines.iter() {
        // First element is the value, second element is the color
        let chunks: Vec<&str> = color_line.split(" ").collect();
        let mut digit: u32 = 0;
        if let Some(num_str) = chunks.get(0) {
            let num_str_owned = num_str.to_string();
            digit = num_str_owned.parse::<u32>().unwrap();
        }
        if let Some(color_str) = chunks.get(1) {
            let color: &str = *color_str;
            match color {
                "red" => {
                    cube_set.red = digit;
                },
                "green" => {
                    cube_set.green = digit;
                },
                "blue" => {
                    cube_set.blue = digit;
                },
                _ => {
                    // Do nothing...
                } 
            }
        }
    }

    cube_set
}

fn get_sum_of_power(input: &str) -> u32 {
    // Get minimum cube set for each game and compute its power
    // Sum all the power
    let mut total: u32 = 0;
    for line in input.lines() {
        // Each line contains the game ID and the sets
        // Number of sets may be arbitrary
        if line.len() > 0 {
            let game_info: Vec<&str> = line.split(": ").collect();
            if let Some(sets_str) = game_info.get(1) {
                let cube_sets = extract_game_sets(sets_str);
                let min_power = compute_min_power(cube_sets);
                total += min_power; 
            }
        }
    }

    total
}

fn compute_min_power(sets: Vec<CubeSet>) -> u32 {
    let mut min_set = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };

    for set in sets.iter() {
        if set.red > min_set.red {
            min_set.red = set.red;
        }
        if set.green > min_set.green {
            min_set.green = set.green;
        }
        if set.blue > min_set.blue {
            min_set.blue = set.blue;
        }
    }

    min_set.red * min_set.green * min_set.blue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_puzzle01() {
        let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        let cube_set = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        let total = get_total_games(input, cube_set);
        assert_eq!(total, 8);
    }

    #[test]
    fn test_day2_puzzle02() {
        let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        let total = get_sum_of_power(input);
        assert_eq!(total, 2286);

    }

}
