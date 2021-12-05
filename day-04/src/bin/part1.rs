use std::{collections::HashMap, fs};

#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();
    let input = fs::read_to_string("./p1.input.txt").unwrap();
    let output = bingo(&input);
    println!("score is {}", output);
}

fn bingo(input: &str) -> u32 {
    let mut game = BingoGame::parse_game_input(input);
    game.play()
}

// fn draw_numbers(boards: Vec<Board>, value_board_map: HashMap<u32, Vec<(usize, (usize, usize))>>) -> (Board, u32) {
//     for (index, number_str) in drawn_numbers.enumerate() {
//         let number:u32 = number_str.parse().unwrap();
//         if let Some(boards_with_value) = value_board_map.get(&number) {
//             for (board_to_mark_index, (row_position, col_position)) in boards_with_value {
//                 let matching_board = boards.get_mut(*board_to_mark_index).unwrap();
//                 matching_board.marked.insert((*row_position, *col_position), 1);
//                 if index >= matching_board.num_rows {
//                     let is_win =
//                 }
//             }
//         }
//     }
// }

#[derive(Clone, Default, Debug)]

struct Board {
    values: HashMap<BoardCoordinates, u32>,
    marked: HashMap<BoardCoordinates, u32>,
    num_rows: usize,
    num_cols: usize,
}
impl Board {
    fn did_col_win(&self, col: &usize) -> bool {
        for row in 0..self.num_rows {
            if self.marked.get(&BoardCoordinates(row, *col)) == Some(&0) {
                return false;
            }
        }
        return true;
    }
    fn did_row_win(&self, row: &usize) -> bool {
        for col in 0..self.num_cols {
            if self.marked.get(&BoardCoordinates(*row, col)) == Some(&0) {
                return false;
            }
        }
        return true;
    }
    fn score(&self, number: u32) -> u32 {
        let mut sum = 0;
        for (BoardCoordinates(row, col), is_marked) in &self.marked {
            if is_marked == &0 {
                if let Some(val) = self.values.get(&BoardCoordinates(*row, *col)) {
                    sum += val;
                }
            }
        }
        sum * number
    }
}
#[derive(Clone, Default, Debug)]
struct BingoGame<'a> {
    drawn_numbers: Vec<&'a str>,
    boards: Vec<Board>,
    value_board_map: HashMap<u32, Vec<(usize, BoardCoordinates)>>,
    winning_board: Option<Board>,
    last_drawn_number: Option<u32>,
}
impl BingoGame<'_> {
    fn parse_game_input(input: &str) -> BingoGame {
        let mut lines = input.lines();
        // first row is drawn numbers
        let drawn_numbers = lines.next().unwrap().split(",").collect();
        // next line is a blank line
        lines.next();
        let mut board: Board = Board::default();
        let mut boards = Vec::<Board>::new();
        let mut value_board_map: HashMap<u32, Vec<(usize, BoardCoordinates)>> = HashMap::new();
        let mut board_index = 0usize;
        let mut row_num = 0usize;
        for line in lines {
            // if it's a blank line, start a new board
            if line == "" {
                board.num_rows = row_num;
                boards.push(board.clone());
                board = Board::default();
                board_index += 1;
                row_num = 0usize;
                continue;
            }
            let numbers = line
                .split(" ")
                .filter(|num| *num != "")
                .collect::<Vec<&str>>();
            board.num_cols = numbers.len();
            for (col_index, number_str) in numbers.into_iter().enumerate() {
                if let Ok(number) = number_str.parse::<u32>() {
                    board
                        .values
                        .insert(BoardCoordinates(row_num, col_index), number);
                    board.marked.insert(BoardCoordinates(row_num, col_index), 0);
                    if let Some(boards_containing_number) = value_board_map.get_mut(&number) {
                        boards_containing_number
                            .push((board_index, BoardCoordinates(row_num, col_index)));
                    } else {
                        value_board_map.insert(
                            number,
                            vec![(board_index, BoardCoordinates(row_num, col_index))],
                        );
                    }
                }
            }
            row_num += 1;
        }
        board.num_rows = row_num;
        boards.push(board.clone());
        BingoGame {
            drawn_numbers,
            boards,
            value_board_map,
            winning_board: None,
            last_drawn_number: None,
        }
    }

    fn draw_numbers(&mut self) {
        for (index, number_str) in self.drawn_numbers.clone().into_iter().enumerate() {
            let number: u32 = number_str.parse().unwrap();
            if let Some(boards_with_value) = self.value_board_map.get(&number) {
                for (board_to_mark_index, BoardCoordinates(row_position, col_position)) in
                    boards_with_value
                {
                    let matching_board = self.boards.get_mut(*board_to_mark_index).unwrap();
                    matching_board
                        .marked
                        .insert(BoardCoordinates(*row_position, *col_position), 1);
                    if index >= matching_board.num_rows && matching_board.did_row_win(row_position)
                    {
                        self.winning_board = Some(matching_board.clone());
                        self.last_drawn_number = Some(number);
                        return;
                    } else if index >= matching_board.num_cols
                        && matching_board.did_col_win(col_position)
                    {
                        self.winning_board = Some(matching_board.clone());
                        self.last_drawn_number = Some(number);
                        return;
                    }
                }
            }
        }
        panic!("No boards won, uh oh")
    }

    fn play(&mut self) -> u32 {
        self.draw_numbers();
        if let Some(board) = &self.winning_board {
            board.score(self.last_drawn_number.unwrap())
        } else {
            panic!("No winning board")
        }
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct BoardCoordinates(usize, usize);

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
 2  0 12  3  7";
    #[test]
    fn part_1() {
        assert_eq!(bingo(INPUT), 4512);
    }
}
