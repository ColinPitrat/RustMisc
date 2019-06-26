#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Black,
    White,
}

impl Cell {
    pub fn next(&self) -> Self {
        match self {
            Cell::Empty => Cell::Empty,
            Cell::White => Cell::Black,
            Cell::Black => Cell::White,
        }
    }
}

pub struct Board {
    to_play: Cell,
    cells: Vec<Vec<Cell>>
}

impl Board {
    pub fn new() -> Board {
        let cells = vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
        ];
        // White starts
        let to_play = Cell::White;
        Board {
            to_play, cells
        }
    }

    pub fn set_pos(&mut self, i: usize, j: usize, c: Cell) -> Result<(), &str> {
        if c != self.to_play {
            Err("Not your turn")
        } else if i > 2 || j > 2 {
            Err("Out of bound")
        } else if let Cell::Empty = self.cells[i][j] {
            self.cells[i][j] = c;
            self.to_play = self.to_play.next();
            Ok(())
        } else {
            Err("Cell already taken")
        }
    }

    pub fn at(&self, i: usize, j: usize) -> Result<Cell, &str> {
        if i > 2 || j > 2 {
            Err("Out of bound")
        } else {
            Ok(self.cells[i][j])
        }
    }

    pub fn winner(&self) -> Option<Cell> {
        // TODO: factorize line & column code
        // Check lines
        for i in 0..3 {
            let c = self.cells[i][0];
            if c == Cell::Empty {
                continue
            }
            let mut win = true;
            for j in 1..3 {
                if self.cells[i][j] != c {
                    win = false;
                    break;
                }
            }
            if win {
                return Some(c);
            }
        }
        // Check columns
        for j in 0..3 {
            let c = self.cells[0][j];
            if c == Cell::Empty {
                continue
            }
            let mut win = true;
            for i in 1..3 {
                if self.cells[i][j] != c {
                    win = false;
                    break;
                }
            }
            if win {
                return Some(c);
            }
        }
        // Check diagonals
        let c = self.cells[0][0];
        if c != Cell::Empty && c == self.cells[1][1] && c == self.cells[2][2] {
            return Some(c);
        }
        let c = self.cells[2][0];
        if c != Cell::Empty && c == self.cells[1][1] && c == self.cells[0][2] {
            return Some(c);
        }
        for i in 0..3 {
            for j in 0..3 {
                if self.cells[i][j] == Cell::Empty {
                    return None;
                }
            }
        }
        // It's a tie ! This is a bit of a hack to return empty ...
        Some(Cell::Empty)
    }

    pub fn to_string(&self) -> String {
        self.cells.iter().map(|line| {
            line.iter().map(|cell| {
                match cell {
                    Cell::Empty => " ",
                    Cell::White => "X",
                    Cell::Black => "O",
                }
            }).collect::<Vec<_>>().join("|")
        }).collect::<Vec<_>>().join("\n-----\n") + "\n"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_pos() {
        let mut board = Board::new();
        assert_eq!(Ok(()), board.set_pos(0, 0, Cell::White));
        assert_eq!(Err("Cell already taken"), board.set_pos(0, 0, Cell::Black));
    }

    #[test]
    fn set_pos_respects_play_order() {
        let mut board = Board::new();
        assert_eq!(Err("Not your turn"), board.set_pos(0, 0, Cell::Black));
        assert_eq!(Ok(()), board.set_pos(0, 0, Cell::White));
        assert_eq!(Err("Not your turn"), board.set_pos(0, 1, Cell::White));
        assert_eq!(Ok(()), board.set_pos(0, 1, Cell::Black));
    }

    #[test]
    fn set_pos_out_of_bound() {
        let mut board = Board::new();
        assert_eq!(Err("Out of bound"), board.set_pos(3, 0, Cell::White));
        assert_eq!(Err("Out of bound"), board.set_pos(0, 3, Cell::White));
    }

    #[test]
    fn at() {
        let mut board = Board::new();
        let mut next_player = Cell::White;
        assert_eq!(Err("Out of bound"), board.at(3, 0));
        assert_eq!(Err("Out of bound"), board.at(0, 3));
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(Ok(Cell::Empty), board.at(i, j));
                board.set_pos(i, j, next_player).unwrap();
                assert_eq!(Ok(next_player), board.at(i, j));
                next_player = next_player.next();
            }
        }
    }

    #[test]
    fn winner_line() {
        for i in 0..3 {
            let mut board = Board::new();
            assert_eq!(None, board.winner());
            board.set_pos(i, 0, Cell::White).unwrap();
            assert_eq!(None, board.winner());
            board.set_pos((i+1)%3, 0, Cell::Black).unwrap();
            assert_eq!(None, board.winner());
            board.set_pos(i, 1, Cell::White).unwrap();
            assert_eq!(None, board.winner());
            board.set_pos((i+1)%3, 1, Cell::Black).unwrap();
            assert_eq!(None, board.winner());
            board.set_pos(i, 2, Cell::White).unwrap();
            assert_eq!(Some(Cell::White), board.winner());
        }
    }

    #[test]
    fn winner_column() {
        for j in 0..3 {
            let mut board = Board::new();
            assert_eq!(None, board.winner());
            board.set_pos(0, j, Cell::White).unwrap();
            assert_eq!(None, board.winner());
            board.set_pos(0, (j+1)%3, Cell::Black).unwrap();
            assert_eq!(None, board.winner());
            board.set_pos(1, j, Cell::White).unwrap();
            assert_eq!(None, board.winner());
            board.set_pos(1, (j+1)%3, Cell::Black).unwrap();
            assert_eq!(None, board.winner());
            board.set_pos(2, j, Cell::White).unwrap();
            assert_eq!(Some(Cell::White), board.winner());
        }
    }

    #[test]
    fn winner_diagonal1() {
        let mut board = Board::new();
        assert_eq!(None, board.winner());
        board.set_pos(0, 0, Cell::White).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(0, 1, Cell::Black).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(1, 1, Cell::White).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(1, 0, Cell::Black).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(2, 2, Cell::White).unwrap();
        assert_eq!(Some(Cell::White), board.winner());
    }

    #[test]
    fn winner_diagonal2() {
        let mut board = Board::new();
        assert_eq!(None, board.winner());
        board.set_pos(2, 0, Cell::White).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(0, 1, Cell::Black).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(1, 1, Cell::White).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(1, 0, Cell::Black).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(0, 2, Cell::White).unwrap();
        assert_eq!(Some(Cell::White), board.winner());
    }

    #[test]
    fn winner_tie() {
        // This corresponds to the following:
        // W B W
        // W B W
        // B W B
        let mut board = Board::new();
        assert_eq!(None, board.winner());
        board.set_pos(0, 0, Cell::White).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(0, 1, Cell::Black).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(0, 2, Cell::White).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(1, 1, Cell::Black).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(1, 0, Cell::White).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(2, 0, Cell::Black).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(2, 1, Cell::White).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(2, 2, Cell::Black).unwrap();
        assert_eq!(None, board.winner());
        board.set_pos(1, 2, Cell::White).unwrap();
        assert_eq!(Some(Cell::Empty), board.winner());
    }

    #[test]
    fn to_string() {
        let mut board = Board::new();
        assert_eq!(" | | \n-----\n | | \n-----\n | | \n", board.to_string());
        board.set_pos(0, 0, Cell::White).unwrap();
        assert_eq!("X| | \n-----\n | | \n-----\n | | \n", board.to_string());
        board.set_pos(1, 1, Cell::Black).unwrap();
        assert_eq!("X| | \n-----\n |O| \n-----\n | | \n", board.to_string());
        board.set_pos(1, 2, Cell::White).unwrap();
        assert_eq!("X| | \n-----\n |O|X\n-----\n | | \n", board.to_string());
    }
}
