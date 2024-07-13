const EMPTY: u8 = 0;



#[derive(Clone)]
pub struct Model {
    board: SudokuBoard,
}

impl Model {
    pub fn new(board: SudokuBoard) -> Model {
        Model { 
            board
        }
    }

    pub fn try_sudoku(&mut self) {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                self.board[i][j] = 1;
            }
        }

        println!("{:?}", self.board)
    }

    pub fn get_board(&self) -> &SudokuBoard {
        &self.board
    }

    pub fn verify_row(&self, current_number: u8, row: usize, column: usize) -> bool {
        for i in 0..self.board.len() {
            if self.board[row][i] == current_number && column != i {
                return false;
            }
        }
        return true;
    }

            
        
    
    pub fn verify_column(&self, current_number: u8, row: usize, column: usize) -> bool {
        true
    }
    pub fn verify_square(&self, current_number: u8, row: usize, column: usize) -> bool {
        true
    }
}

type SudokuBoard = [[u8; 9]; 9];

pub struct Controller<'a> {
    model: &'a mut Model,
}

impl<'a> Controller<'a> {
    pub fn new(model: &'a mut Model) -> Controller<'a> {
        Controller { model }
    }

    pub fn mutate_board(&mut self) {
        self.model.try_sudoku();
    }
}

pub struct View {
    model: Model,
}

impl View {
    pub fn new(model: Model) -> View {
        View { model }
    }

    pub fn display_board(&self) {
        println!("{:?}", &self.model.get_board())
    }
}

fn main() {
    let board: SudokuBoard = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    let mut model = Model::new(board);
    let mut controller = Controller::new(&mut model);
    controller.mutate_board();
    let view = View::new(model);
    view.display_board();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_complete_board() {
        let board: SudokuBoard = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];
        let mut model = Model::new(board);
        model.try_sudoku();
        for i in 0..model.board.len() {
            for j in 0..model.board[1].len() {
                assert_ne!(model.board[i][j], 0)
            }
        }
    }

    #[test]
    fn can_verify_row() {
        let board: SudokuBoard = [
            [5, 3, 1, 2, 7, 4, 6, 9, 8], // correct
            [6, 0, 0, 1, 9, 5, 0, 0, 0], // incomplete
            [1, 9, 8, 1, 2, 4, 5, 6, 7], // incorrect
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];
        const REPEATED_NUMBER_3RD_ROW: u8 = 1;
        let model = Model::new(board);
        for i in 0..model.board.len() {
            // first row
            let current_number_row_1 = model.board[0][i];
            assert!(model.verify_row(current_number_row_1, 0, i));

            // second row
            let current_number_row_2 = model.board[1][i];
            if current_number_row_2 == EMPTY {
                assert!(!model.verify_row(current_number_row_2, 1, i))
            } else {
                assert!(model.verify_row(current_number_row_2, 1, i))
            }

            // third row
            let current_number_row_3 = model.board[2][i];
            if current_number_row_3 == REPEATED_NUMBER_3RD_ROW {
                assert!(!model.verify_row(current_number_row_3, 2, i))
            } else {
                assert!(model.verify_row(current_number_row_3, 2, i))
            }
        }
    }
}

#[ignore]
#[test]
fn can_solve_board() {
    let board: SudokuBoard = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    let mut model = Model::new(board);
    model.try_sudoku();
    for i in 0..model.board.len() {
        for j in 0..model.board[1].len() {
            let current_number = model.board[i][j];
            assert!(model.verify_row(current_number, i, j));
            assert!(model.verify_column(current_number, i, j));
            assert!(model.verify_square(current_number, i, j));
        }
    }
}
