use std::fmt::Display;
use std::usize;

fn main() {
    let input = include_str!("input.txt");

    let (part1, part2) = simulate(input);

    println!("{}", part1);
    println!("{}", part2);
}

fn simulate(input: &str) -> (usize, usize) {
    let mut part1 = None;
    let mut part2 = None;

    let mut flashes = 0;

    let mut grid = parse_grid(input);

    for step in 1.. {
        let mut flashing = Vec::new();

        let flashes_before_step = flashes;

        for (index, energy) in grid.iter_mut().enumerate() {
            *energy += 1;

            if *energy > 9 {
                flashing.push(index);
            }
        }

        while let Some(flashing_i) = flashing.pop() {
            grid[flashing_i] = 0;
            flashes += 1;

            for adj_i in grid_adj_indices(flashing_i, 10, 10) {
                if grid[adj_i] == 0 || grid[adj_i] > 9 {
                    continue;
                }

                grid[adj_i] += 1;

                if grid[adj_i] > 9 {
                    flashing.push(adj_i);
                }
            }
        }

        if step == 100 {
            part1 = Some(flashes);
        }

        if (flashes - flashes_before_step) == 100 {
            part2 = Some(step);
        }

        if let (Some(part1), Some(part2)) = (part1, part2) {
            return (part1, part2);
        }
    }

    panic!("How did we get there??");
}

fn _grid_print<T: Display>(grid: &[T], grid_width: usize) {
    for (index, item) in grid.iter().enumerate() {
        print!("[{:02}]{:02} ", index, item);

        if (index + 1) % grid_width == 0 {
            println!();
        }
    }

    println!("----");
}

fn grid_adj_indices(index: usize, grid_width: usize, grid_height: usize) -> Vec<usize> {
    let is_left = (index % grid_width) == 0;
    let is_right = ((index + 1) % grid_width) == 0;

    let mut adj_indices = Vec::with_capacity(8);

    if !is_left {
        // top left:
        adj_indices.push(
            index
                .checked_sub(grid_width)
                .and_then(|res| res.checked_sub(1)),
        );

        // left
        adj_indices.push(index.checked_sub(1));

        // bottom left
        adj_indices.push(
            index
                .checked_add(grid_width)
                .and_then(|res| res.checked_sub(1)),
        );
    }

    if !is_right {
        // top right
        adj_indices.push(
            index
                .checked_sub(grid_width)
                .and_then(|res| res.checked_add(1)),
        );

        // right
        adj_indices.push(index.checked_add(1));

        // bottom right
        adj_indices.push(
            index
                .checked_add(grid_width)
                .and_then(|res| res.checked_add(1)),
        );
    }

    // top
    adj_indices.push(index.checked_sub(grid_width));

    // bottom
    adj_indices.push(index.checked_add(grid_width));

    adj_indices
        .into_iter()
        .flatten()
        .filter(|index| *index < grid_width * grid_height)
        .collect()
}

fn parse_grid(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn example() {
        assert_eq!(simulate(EXAMPLE), (1656, 195));
    }
}
