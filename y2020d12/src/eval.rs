use crate::*;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum CardinalDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RelativeDirection {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum InstructionKind {
    GoCardinal(CardinalDirection),
    GoForward,
    Turn(RelativeDirection),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instruction {
    kind: InstructionKind,
    magnitude: i32,
}

#[derive(Debug)]
pub struct ParsedInput {
    instructions: Vec<Instruction>,
}
pub fn parse(input: &str) -> IResult<&str, ParsedInput> {
    let north = map(tag("N"), |_| InstructionKind::GoCardinal(CardinalDirection::North));
    let east = map(tag("E"), |_| InstructionKind::GoCardinal(CardinalDirection::East));
    let south = map(tag("S"), |_| InstructionKind::GoCardinal(CardinalDirection::South));
    let west = map(tag("W"), |_| InstructionKind::GoCardinal(CardinalDirection::West));
    let left = map(tag("L"), |_| InstructionKind::Turn(RelativeDirection::Left));
    let right = map(tag("R"), |_| InstructionKind::Turn(RelativeDirection::Right));
    let forward = map(tag("F"), |_| InstructionKind::GoForward);
    let instruction_kind = alt((north, east, south, west, left, right, forward));
    let number = map_res(digit1, FromStr::from_str);
    let instruction = map(tuple((instruction_kind, number)), |(kind, magnitude)| Instruction { kind, magnitude });
    let mut parsed = map(separated_list1(line_ending, instruction), |instructions| ParsedInput { instructions });

    Ok(parsed(input)?)
}

pub struct ShipSimulator {
    facing: CardinalDirection,
    x: i32,
    y: i32,
}
impl ShipSimulator {
    fn origin() -> ShipSimulator {
        ShipSimulator { facing: CardinalDirection::East, x: 0, y: 0 }
    }
    fn do_turn(&mut self, direction: RelativeDirection, magnitude: i32) {
        if magnitude == 0 {
            return;
        };
        if magnitude % 90 != 0 {
            panic!()
        };
        match direction {
            RelativeDirection::Left =>
                self.facing = match self.facing {
                    CardinalDirection::North => CardinalDirection::West,
                    CardinalDirection::East => CardinalDirection::North,
                    CardinalDirection::South => CardinalDirection::East,
                    CardinalDirection::West => CardinalDirection::South,
                },
            RelativeDirection::Right =>
                self.facing = match self.facing {
                    CardinalDirection::North => CardinalDirection::East,
                    CardinalDirection::East => CardinalDirection::South,
                    CardinalDirection::South => CardinalDirection::West,
                    CardinalDirection::West => CardinalDirection::North,
                },
        }
        self.do_turn(direction, magnitude - 90);
    }
    fn do_cardinal_move(&mut self, direction: CardinalDirection, magnitude: i32) {
        match direction {
            CardinalDirection::North => self.y -= magnitude,
            CardinalDirection::East => self.x += magnitude,
            CardinalDirection::South => self.y += magnitude,
            CardinalDirection::West => self.x -= magnitude,
        }
    }
    fn do_forward_move(&mut self, magnitude: i32) {
        self.do_cardinal_move(self.facing, magnitude);
    }
    fn simulate(instructions: &[Instruction]) -> ShipSimulator {
        let mut sim = ShipSimulator::origin();
        for instruction in instructions {
            match instruction.kind {
                InstructionKind::Turn(direction) => sim.do_turn(direction, instruction.magnitude),
                InstructionKind::GoCardinal(direction) => sim.do_cardinal_move(direction, instruction.magnitude),
                InstructionKind::GoForward => sim.do_forward_move(instruction.magnitude),
            }
        }
        sim
    }
}

#[derive(Debug)]
pub struct ShipWaypointSimulator {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}
impl ShipWaypointSimulator {
    fn origin() -> ShipWaypointSimulator {
        ShipWaypointSimulator { x: 0, y: 0, dx: 10, dy: -1 }
    }
    fn turn_waypoint(&mut self, direction: RelativeDirection, magnitude: i32) {
        if magnitude == 0 {
            return;
        };
        if magnitude % 90 != 0 {
            panic!()
        };
        let (ndx, ndy) = match direction {
            RelativeDirection::Left => (self.dy, -self.dx),
            RelativeDirection::Right => (-self.dy, self.dx),
        };
        self.dx = ndx;
        self.dy = ndy;
        self.turn_waypoint(direction, magnitude - 90);
    }
    fn move_waypoint(&mut self, direction: CardinalDirection, magnitude: i32) {
        match direction {
            CardinalDirection::North => self.dy -= magnitude,
            CardinalDirection::East => self.dx += magnitude,
            CardinalDirection::South => self.dy += magnitude,
            CardinalDirection::West => self.dx -= magnitude,
        }
    }
    fn do_move(&mut self, magnitude: i32) {
        self.x += self.dx * magnitude;
        self.y += self.dy * magnitude;
    }
    fn simulate(instructions: &[Instruction]) -> ShipWaypointSimulator {
        let mut sim = ShipWaypointSimulator::origin();
        for instruction in instructions {
            match instruction.kind {
                InstructionKind::Turn(direction) => sim.turn_waypoint(direction, instruction.magnitude),
                InstructionKind::GoCardinal(direction) => sim.move_waypoint(direction, instruction.magnitude),
                InstructionKind::GoForward => sim.do_move(instruction.magnitude),
            }
        }
        sim
    }
}

pub type Task1 = i32;
pub type Task2 = i32;
pub fn compute(input: ParsedInput) -> Result<Output> {
    let sim = ShipSimulator::simulate(&input.instructions);
    let waypoint_sim = ShipWaypointSimulator::simulate(&input.instructions);
    Ok(Output { task1: sim.x.abs() + sim.y.abs(), task2: waypoint_sim.x.abs() + waypoint_sim.y.abs() })
}
