use aoc::*;
use ndarray::{s, Array1, Array2};

const BINGO_SIZE: usize = 5;

#[derive(Clone)]
struct ParsedInput {
    sequence: Vec<u32>,
    boards: Vec<BingoBoard>,
}

#[derive(Clone)]
struct BingoBoard {
    board: Array2<u32>,
    marked: Array2<bool>,
}

fn parse(input: &str) -> ParseResult<ParsedInput> {
    use nom::{
        character::complete::{char, line_ending, space0, space1, u32},
        combinator::{map, map_res},
        multi::separated_list1,
        Parser,
    };

    let sequence = separated_list1(char(','), u32);
    let gap = |i| line_ending.and(line_ending).parse(i);
    let row = space0.and(separated_list1(space1, u32));
    let rows = separated_list1(line_ending, row);
    let board = map_res(rows, |i| {
        Array1::from_iter(i.into_iter().flat_map(|(_, i)| i))
            .into_shape((BINGO_SIZE, BINGO_SIZE))
            .map(|board| BingoBoard {
                board,
                marked: Array2::from_elem((BINGO_SIZE, BINGO_SIZE), false),
            })
    });
    let boards = separated_list1(gap, board);
    let mut parser = map(sequence.and(gap).and(boards), |((sequence, _), boards)| {
        ParsedInput { sequence, boards }
    });
    parser.parse(input)
}

impl BingoBoard {
    fn mark(&mut self, n: u32) -> Option<u32> {
        for (pos @ (i, j), value) in self.board.indexed_iter() {
            if *value == n {
                if let Some(v) = self.marked.get_mut(pos) {
                    *v = true
                };
            }
            let i_set = self.marked.slice(s![i, ..]);
            let j_set = self.marked.slice(s![.., j]);
            if i_set.iter().all(|v| *v) || j_set.iter().all(|v| *v) {
                return Some(self.compute_score());
            }
        }
        None
    }
    fn compute_score(&self) -> u32 {
        self.marked
            .indexed_iter()
            .filter(|(_, value)| !**value)
            .filter_map(|(pos, _)| self.board.get(pos))
            .sum()
    }
}

fn task1(input: &ParsedInput) -> Result<u32> {
    let mut boards = input.boards.clone();
    for &value in input.sequence.iter() {
        for board in boards.iter_mut() {
            if let Some(win) = board.mark(value) {
                return Ok(win * value);
            }
        }
    }
    Err("No winner found".into())
}

fn task2(input: &ParsedInput) -> Result<u32> {
    let mut boards = input.boards.clone();
    let mut board_has_won = boards.iter().map(|_| false).collect::<Vec<_>>();
    for &value in input.sequence.iter() {
        for (i, board) in boards.iter_mut().enumerate() {
            let boards_remaining = board_has_won.iter().filter(|n| !**n).count();
            if board_has_won.get(i) == Some(&false) {
                if let Some(win) = board.mark(value) {
                    if let Some(v) = board_has_won.get_mut(i) {
                        *v = true
                    };
                    if boards_remaining == 1 {
                        return Ok(win * value);
                    }
                }
            }
        }
    }
    Err("No loser found".into())
}

#[test]
fn test() {
    let input = r#"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
    "#
    .trim();

    assert_task!(parse, task1, input, 4512);
    assert_task!(parse, task2, input, 1924);
}

aoc_main!(parse, task1, task2);
