static INPUT: &str = include_str!("../../inputs/day12.txt");
const CARDINAL_DIRECTIONS: [char; 4] = ['N', 'E', 'S', 'W'];

type Action = (char, i32);
type Coord = (i32, i32);

pub fn run() {
    let actions = transform_input(INPUT);

    println!("Part 1: {}", part1(&actions));
    println!("Part 2: {}", part2(&actions));
}

fn transform_input(input: &'static str) -> Vec<Action> {
    input
        .lines()
        .map(|x| {
            let (letter, number) = x.split_at(1);
            let action = letter.chars().next().unwrap();
            let value = number.parse().unwrap();

            (action, value)
        })
        .collect()
}

fn part1(actions: &[Action]) -> i32 {
    let (final_coords, _) = actions.iter().fold(
        ((0, 0), 'E'),
        |(ship_coords, direction), &(action, value)| match action {
            'L' => (ship_coords, rotate_cardinal(direction, 'L', value)),
            'R' => (ship_coords, rotate_cardinal(direction, 'R', value)),
            'F' => (move_coord(ship_coords, direction, value), direction),
            d => (move_coord(ship_coords, d, value), direction),
        },
    );

    manhattan_distance((0, 0), final_coords)
}

fn move_coord((x, y): Coord, direction: char, magnitude: i32) -> Coord {
    match direction {
        'N' => (x, y + magnitude),
        'S' => (x, y - magnitude),
        'E' => (x + magnitude, y),
        'W' => (x - magnitude, y),
        _ => panic!("Invalid cardinal direction!"),
    }
}

fn rotate_cardinal(facing: char, direction: char, degrees: i32) -> char {
    let old_idx = CARDINAL_DIRECTIONS
        .iter()
        .position(|&x| x == facing)
        .unwrap();

    let new_idx = match direction {
        'L' => (old_idx + 4 - (degrees as usize / 90)) % 4,
        'R' => (old_idx + (degrees as usize / 90)) % 4,
        _ => panic!("Invalid direction: {}!", direction),
    };

    CARDINAL_DIRECTIONS[new_idx]
}

fn manhattan_distance((x1, y1): Coord, (x2, y2): Coord) -> i32 {
    (y2 - y1).abs() + (x2 - x1).abs()
}

fn part2(actions: &[Action]) -> i32 {
    let (final_coords, _, _) = actions.iter().fold(
        ((0, 0), (10, 1), 'E'),
        |(ship, waypoint, direction), &(action, value)| match action {
            'L' => (ship, rotate_waypoint(waypoint, 'L', value), direction),
            'R' => (ship, rotate_waypoint(waypoint, 'R', value), direction),
            'F' => {
                let new_ship =
                    (ship.0 + waypoint.0 * value, ship.1 + waypoint.1 * value);
                (new_ship, waypoint, direction)
            }
            d => (ship, move_coord(waypoint, d, value), direction),
        },
    );

    manhattan_distance((0, 0), final_coords)
}

fn rotate_waypoint(waypoint: Coord, direction: char, magnitude: i32) -> Coord {
    let rotate_90 = match direction {
        'L' => |(x, y): Coord, _| (-y, x),
        'R' => |(x, y): Coord, _| (y, -x),
        _ => panic!("Invalid direction: {}!", direction),
    };

    (0..magnitude / 90).fold(waypoint, rotate_90)
}
