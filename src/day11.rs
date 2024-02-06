use std::collections::{HashMap, HashSet};
use glam::I64Vec2;
use num::abs;

#[derive(Debug, Clone)]
struct Galaxy {
    id: u64,
    coord: I64Vec2,
}

#[derive(Debug, Clone, PartialEq)]
enum SkyItem {
    Galaxy,
    Space,
}

pub fn part1(input: &str) -> u64 {
    let sky = parse_input(input);
    sum_shortest_paths(&sky, 2)
}

pub fn part2(input: &str) -> u64 {
    let sky = parse_input(input);
    sum_shortest_paths(&sky, 1000000)
}

fn sum_shortest_paths(sky: &Vec<Vec<SkyItem>>, pad: i64) -> u64 {
    let galaxies = expand_universe(&sky, pad);
    let pairs = create_galaxy_pairs(&galaxies);

    let mut shortest_paths: u64 = 0;
    for pair in pairs.iter() {
        let distance = find_manhattan_distance(&pair.0.coord, &pair.1.coord);
        shortest_paths += distance as u64;
    }
    shortest_paths
}

fn find_manhattan_distance(source: &I64Vec2, dest: &I64Vec2) -> i64 {
    abs(source.x - dest.x) + abs(source.y - dest.y)
}

fn create_galaxy_pairs(galaxies: &Vec<Galaxy>) -> Vec<(Galaxy, Galaxy)> {
    let mut pairs: HashMap<I64Vec2, (Galaxy, Galaxy)> = HashMap::new();

    for x in 0..galaxies.len() {
        for y in 1..galaxies.len() {
            let x_item = &galaxies[x];
            let y_item = &galaxies[y];
            if x_item.id != y_item.id {
                let key = I64Vec2::from((x_item.id as i64, y_item.id as i64));
                let key_rev = I64Vec2::from((y_item.id as i64, x_item.id as i64));
                if pairs.get(&key).is_none() && pairs.get(&key_rev).is_none() {
                    pairs.insert(key, (x_item.clone(), y_item.clone()));
                }
            }
        }
    }
    pairs.into_values().collect()
}

fn expand_universe(sky: &Vec<Vec<SkyItem>>, pad: i64) -> Vec<Galaxy> {
    let galaxies = collect_galaxies(sky);

    // Find empty rows and empty columns
    let mut empty_rows: HashSet<usize> = HashSet::new();
    let mut empty_cols: HashSet<usize> = HashSet::new();

    // Find empty rows
    for (i, row) in sky.iter().enumerate() {
        let count = row.iter().filter(|x| *x == &SkyItem::Space).count();
        if count == row.len() {
            empty_rows.insert(i);
        }
    }

    // Find empty cols
    let row_len = sky[0].len();
    for y in 0..row_len {
        let mut count: usize = 0;
        for x in 0..sky.len() {
            let item = &sky[x][y];
            if *item == SkyItem::Space {
                count += 1;
            }
        }
        if count == sky.len() {
            empty_cols.insert(y);
        }
    }

    // Simply move the galaxies away based on the padding
    galaxies.iter().map(|galaxy| {
        let mut x = galaxy.coord.x;
        let mut y = galaxy.coord.y;

        for empty_x in empty_rows.iter() {
            if (*empty_x as i64) < galaxy.coord.x {
                x += pad - 1;
            }
        }

        for empty_y in empty_cols.iter() {
            if (*empty_y as i64) < galaxy.coord.y {
                y += pad - 1;
            }
        }
        Galaxy {
            id: galaxy.id,
            coord: I64Vec2::from((x, y))
        }
    }).collect::<Vec<Galaxy>>()
}

fn collect_galaxies(sky: &Vec<Vec<SkyItem>>) -> Vec<Galaxy> {
    let mut galaxy_count: u64 = 0;
    let mut galaxies: Vec<Galaxy> = Vec::new();
    for (x, row) in sky.iter().enumerate() {
        for (y, item) in row.iter().enumerate() {
            if item == &SkyItem::Galaxy {
                galaxy_count += 1;

                let coord = I64Vec2::from((x as i64, y as i64));
                galaxies.push(Galaxy { id: galaxy_count, coord });
            }
        }
    }
    galaxies
}

fn parse_input(input: &str) -> Vec<Vec<SkyItem>> {
    input.lines().map(|line| {
        line.chars().map(|ch| {
            match ch {
                '#' => {
                    SkyItem::Galaxy
                },
                '.' => {
                    SkyItem::Space
                },
                _ => {
                    panic!("Unexpected character.");
                }
            }
        }).collect::<Vec<SkyItem>>()
    }).collect::<Vec<Vec<SkyItem>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let result = part1(input);
        assert_eq!(result, 374);
    }

    #[test]
    fn test_part2_data1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let sky = parse_input(input);
        let result = sum_shortest_paths(&sky, 10);
        assert_eq!(result, 1030);
    }

    #[test]
    fn test_part2_data2() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let sky = parse_input(input);
        let result = sum_shortest_paths(&sky, 100);
        assert_eq!(result, 8410);
    }
}
