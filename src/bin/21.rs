use std::str::FromStr;

use advent_of_code::repository::steps::{Garden, Coord};

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    let garden = Garden::from_str(input).unwrap();
    Some(garden.take_steps(garden.get_starting_position(), 64))
}

pub fn part_two(input: &str) -> Option<u64> {
    let garden = Garden::from_str(input).unwrap();
    let steps = 26501365.0;
    let starting_position = garden.get_starting_position();

    let odd_plot_values = garden.take_steps(garden.get_starting_position(), garden.cols * 2 + 1) as u64;
    let even_plot_values = garden.take_steps(garden.get_starting_position(), garden.cols * 2) as u64;

    let grid_width = (steps / garden.cols as f32).floor() as u64 - 1;
    let full_odd_plots = u64::pow((grid_width as f32 / 2.0).floor() as u64 * 2 + 1, 2);
    let full_even_plots = u64::pow(((grid_width as f32 + 1.0) / 2.0).floor() as u64 * 2, 2);

    let top_corner = garden.take_steps(Coord{ x: starting_position.x, y: garden.rows - 1 }, garden.cols - 1) as u64;
    let right_corner = garden.take_steps(Coord{ x: 0, y: starting_position.y }, garden.cols - 1) as u64;
    let bottom_corner = garden.take_steps(Coord{ x: starting_position.x, y: 0 }, garden.cols - 1) as u64;
    let left_corner = garden.take_steps(Coord{ x: garden.cols - 1, y: starting_position.y }, garden.cols - 1) as u64;

    let small_top_right = garden.take_steps(Coord { x: garden.cols - 1, y: 0 }, (garden.cols as f32 / 2.0).floor() as usize - 1);
    let small_bottom_right = garden.take_steps(Coord { x: 0, y: 0 }, (garden.cols as f32 / 2.0).floor() as usize - 1);
    let small_bottom_left = garden.take_steps(Coord { x: 0, y: garden.rows - 1 }, (garden.cols as f32 / 2.0).floor() as usize - 1);
    let small_top_left = garden.take_steps(Coord { x: garden.cols - 1, y: garden.rows - 1 }, (garden.cols as f32 / 2.0).floor() as usize - 1);

    let large_top_right = garden.take_steps(Coord { x: garden.cols - 1, y: 0 }, (garden.cols as f32 * 3.0 / 2.0).floor() as usize - 1);
    let large_bottom_right = garden.take_steps(Coord { x: 0, y: 0 }, (garden.cols as f32 * 3.0 / 2.0).floor() as usize - 1);
    let large_bottom_left = garden.take_steps(Coord { x: 0, y: garden.rows - 1 }, (garden.cols as f32 * 3.0 / 2.0).floor() as usize - 1);
    let large_top_left = garden.take_steps(Coord { x: garden.cols - 1, y: garden.rows - 1 }, (garden.cols as f32 * 3.0 / 2.0).floor() as usize - 1);

    Some(
        odd_plot_values * full_odd_plots +
        even_plot_values * full_even_plots +
        top_corner + right_corner + bottom_corner + left_corner +
        (grid_width + 1) * (small_top_right + small_bottom_right + small_bottom_left + small_top_left) as u64 +
        grid_width * (large_top_right + large_bottom_right + large_bottom_left + large_top_left) as u64
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
