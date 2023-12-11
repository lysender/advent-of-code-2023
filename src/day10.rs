
use glam::IVec2;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    ElbowNE,
    ElbowNW,
    ElbowSW,
    ElbowSE,
    Ground,
    Start,
}

#[derive(Debug, Clone, PartialEq)]
enum MarkedTile {
    Enclosure,
    NonEnclosure,
    Regular,
}

#[derive(Debug, Clone, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct MoveMap {
    source: Vec<Tile>,
    dest: Vec<Tile>,
    movement: IVec2,
}

pub fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<Tile>> = parse_input(input);
    let directions = create_move_directions();
    let start = find_start(&grid);
    let initial_moves = get_initial_moves(&grid, &directions, &start);
    if let Some(steps) = find_loop(&grid, &directions, &start, &initial_moves.0) {
        return (steps.len() / 2) as u32;
    }
    0
}

pub fn part2(input: &str) -> u32 {
    let grid: Vec<Vec<Tile>> = parse_input(input);
    let directions = create_move_directions();
    let start = find_start(&grid);
    let initial_moves = get_initial_moves(&grid, &directions, &start);
    if let Some(steps) = find_loop(&grid, &directions, &start, &initial_moves.0) {
        return find_enclosure(&grid, &steps) as u32;
    }
    0
}

fn find_enclosure(grid: &Vec<Vec<Tile>>, steps: &Vec<IVec2>) -> usize {
    // We can mark boundaries by only looking for pipe that is part of our loop
    // Mark all pipes across the grid
    // Scan row by row and find a group of tiles that are not part of the loop
    // (ground, useless pipe)
    // Once a group of contiues pipes are found,
    // count the walls (pipe whose side is facing north) to the left and to the right
    // If each are odd, tiles are inside the loop
    
    // Create a marked grid
    let empty_row: Vec<MarkedTile> = vec![MarkedTile::Regular; grid[0].len()];
    let mut marked_grid: Vec<Vec<MarkedTile>> = vec![empty_row; grid.len()];

    for step in steps.iter() {
        let tile = &grid[step.x as usize][step.y as usize];
        let marker = match tile {
            Tile::Start => MarkedTile::Enclosure,
            Tile::Vertical => MarkedTile::Enclosure,
            Tile::Horizontal => MarkedTile::NonEnclosure,
            Tile::ElbowNE => MarkedTile::NonEnclosure,
            Tile::ElbowNW => MarkedTile::NonEnclosure,
            Tile::ElbowSE => MarkedTile::Enclosure,
            Tile::ElbowSW => MarkedTile::Enclosure,
            _ => MarkedTile::Regular,
        };
        marked_grid[step.x as usize][step.y as usize] = marker; 
    }

    let mut enclosed_count: usize = 0;

    for (x, row) in marked_grid.iter().enumerate() {
        let mut contents: Vec<Range<usize>> = Vec::new();
        let mut buffer_start: Option<usize> = None;
        let mut buffer_end: Option<usize> = None;

        for (y, marker) in row.iter().enumerate() {
            if marker == &MarkedTile::Enclosure {
                // Contents always start with an enclosure pipe
                // Flush buffer if there are any
                if buffer_start.is_some() && buffer_end.is_some() {
                    if buffer_start.unwrap() <= buffer_end.unwrap() {
                        contents.push(Range { start: buffer_start.unwrap(), end: buffer_end.unwrap() });
                    }
                }

                // Buffer start on the next tile
                buffer_start = Some(y + 1);
                buffer_end = Some(y + 1);
            } else if marker == &MarkedTile::Regular {
                if buffer_start.is_some() {
                    buffer_end = Some(y);
                }
            }
        }

        for content_range in contents.iter() {
            let (left, _right) = count_walls(&marked_grid, x, &content_range);
            if left > 0 {
                if left % 2 != 0 {
                    for i in content_range.start..=content_range.end {
                        let marker = &marked_grid[x][i];
                        if marker == &MarkedTile::Regular {
                            enclosed_count += 1;
                        }
                    }
                }
            }
        }
    }

    enclosed_count
}

fn count_walls(grid: &Vec<Vec<MarkedTile>>, row_index: usize, content_range: &Range<usize>) -> (u32, u32) {
    let mut left: u32 = 0;
    let mut right: u32 = 0;

    if content_range.start > 0 {
        for i in 0..content_range.start {
            let marker = &grid[row_index][i];
            if marker == &MarkedTile::Enclosure {
                left += 1;
            }
        }
    }

    if content_range.end < grid[0].len() {
        for i in content_range.end..grid[0].len() {
            let marker = &grid[row_index][i];
            if marker == &MarkedTile::Enclosure {
                right += 1;
            }
        }
    }

    (left, right)
}

fn find_loop(grid: &Vec<Vec<Tile>>, directions: &Vec<MoveMap>, start: &IVec2, current: &IVec2) -> Option<Vec<IVec2>> {
    let mut steps: Vec<IVec2> = Vec::new();
    steps.push(current.clone());

    let mut current_coord = current.clone();
    let mut next: Option<IVec2> = next_move(grid, directions, &current_coord, start);

    while next.is_some() {
        let next_coord = next.unwrap();
        steps.push(next_coord.clone());
        next = next_move(grid, directions, &next_coord, &current_coord);
        current_coord = next_coord.clone();
    }

    if steps.len() > 0 {
        if let Some(last) = steps.last() {
            let tile = &grid[last.x as usize][last.y as usize];
            if *tile == Tile::Start {
                return Some(steps);
            }
        }
    }
    None
}

fn get_initial_moves(grid: &Vec<Vec<Tile>>, directions: &Vec<MoveMap>, start: &IVec2) -> (IVec2, IVec2) {
    let rows: i32 = grid.len() as i32;
    let cols: i32 = grid[0].len() as i32;
    let mut moves: Vec<IVec2> = Vec::new();

    for direction in directions.iter() {
        let next = *start + direction.movement;

        // Check for out of bounds
        if next.x >= 0 && next.y >= 0 && next.x < rows && next.y < cols {
            // Make sure that the next tile is compatible
            let next_tile = &grid[next.x as usize][next.y as usize];
            if direction.dest.contains(next_tile) {
                moves.push(next);
            }
        }
    }

    if moves.len() != 2 {
        panic!("There must be exactly 2 initial moves.");
    }
    (moves[0], moves[1])
}

fn next_move(grid: &Vec<Vec<Tile>>, directions: &Vec<MoveMap>, start: &IVec2, prev: &IVec2) -> Option<IVec2> {
    let rows: i32 = grid.len() as i32;
    let cols: i32 = grid[0].len() as i32;
    let tile = &grid[start.x as usize][start.y as usize];
    let mut next_tile_step: Option<IVec2> = None;

    for direction in directions.iter() {
        let next = *start + direction.movement;

        // Check for out of bounds
        if next.x >= 0 && next.y >= 0 && next.x < rows && next.y < cols {
            // Make sure that the next tile is compatible
            let next_tile = &grid[next.x as usize][next.y as usize];
            if direction.source.contains(&tile) && (direction.dest.contains(next_tile) || *next_tile == Tile::Start) {
                // Exclude previous coords
                if prev != &next {
                    next_tile_step = Some(next);
                    break;
                }
            }
        }
    }

    next_tile_step
}

fn create_move_directions() -> Vec<MoveMap> {
    Vec::from([
        // Down
        MoveMap {
            source: vec![Tile::Vertical, Tile::ElbowSE, Tile::ElbowSW],
            dest: vec![Tile::Vertical, Tile::ElbowNE, Tile::ElbowNW],
            movement: IVec2::from((1, 0)),
        },
        // Right
        MoveMap {
            source: vec![Tile::Horizontal, Tile::ElbowNE, Tile::ElbowSE],
            dest: vec![Tile::Horizontal, Tile::ElbowSW, Tile::ElbowNW],
            movement: IVec2::from((0, 1)),
        },
        // Up
        MoveMap {
            source: vec![Tile::Vertical, Tile::ElbowNE, Tile::ElbowNW],
            dest: vec![Tile::Vertical, Tile::ElbowSE, Tile::ElbowSW],
            movement: IVec2::from((-1, 0)),
        },
        // Left
        MoveMap {
            source: vec![Tile::Horizontal, Tile::ElbowSW, Tile::ElbowNW],
            dest: vec![Tile::Horizontal, Tile::ElbowNE, Tile::ElbowSE],
            movement: IVec2::from((0, -1)),
        },
    ])
}

fn find_start(grid: &Vec<Vec<Tile>>) -> IVec2 {
    // Find the starting position of the grid
    for x in 0..grid.len() {
        let row = &grid[x];
        for y in 0..row.len() {
            let tile = &grid[x][y];
            if *tile == Tile::Start {
                return IVec2::from((x as i32, y as i32));
            }
        }
    }
    panic!("Unable to find the staring position.");
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input.lines().map(|line| {
        parse_line(line)
    }).collect::<Vec<Vec<Tile>>>()
}

fn parse_line(line: &str) -> Vec<Tile> {
    line.chars().map(|ch| {
        match ch {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::ElbowNE,
            'J' => Tile::ElbowNW,
            '7' => Tile::ElbowSW,
            'F' => Tile::ElbowSE,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => {
                panic!("Unexpected tile.");
            }
        }
    }).collect::<Vec<Tile>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_data1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        let result = part1(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part1_data2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        let result = part1(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2_data1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let result = part2(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part2_data2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        let result = part2(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2_data3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let result = part2(input);
        assert_eq!(result, 10);
    }
}
