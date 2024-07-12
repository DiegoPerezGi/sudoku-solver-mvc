#[derive(Clone)]
pub struct Model {
    
    
}

impl Model {
    pub fn new(board: SudokuBoard) -> Model {
        Model {

        }
    }

    pub fn solve_sudoku(board: SudokuBoard){

    }
}

type SudokuBoard = [[[u8; 3]; 3]; 9];

pub struct Controller {
   model: Model,
}

impl Controller {    
    pub fn new(model: Model) -> Controller {
        Controller {
            model,
        }
    }
}

pub struct View {
    model: Model,
}

impl View {
    pub fn new(model: Model) -> View {
        View {
            model,
        }
    }
}


fn main() {
 let board: SudokuBoard = [
        [
            [5, 3, 0],
            [0, 7, 0],
            [0, 0, 0],
        ],
        [
            [6, 0, 0],
            [1, 9, 5],
            [0, 0, 0],
        ],
        [
            [0, 9, 8],
            [0, 0, 0],
            [0, 6, 0],
        ],
        [
            [8, 0, 0],
            [0, 6, 0],
            [0, 0, 3],
        ],
        [
            [4, 0, 0],
            [8, 0, 3],
            [0, 0, 1],
        ],
        [
            [7, 0, 0],
            [0, 2, 0],
            [0, 0, 6],
        ],
        [
            [0, 6, 0],
            [0, 0, 0],
            [2, 8, 0],
        ],
        [
            [0, 0, 0],
            [4, 1, 9],
            [0, 0, 5],
        ],
        [
            [0, 0, 0],
            [0, 8, 0],
            [0, 7, 9],
        ],
    ];
    let model = Model::new(board);
    let controller = Controller::new(model.clone());
    let view = View::new(model.clone());

    

}
