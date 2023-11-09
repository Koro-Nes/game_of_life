use rand::Rng;
use crate::cell::Cell as Cell;
use crate::cell::State as State;

#[derive(Debug)]
pub struct Game {
    board: Vec<Vec<Cell>>,
    len: usize,
}

impl Game {

    //initialize board with dead cells
    pub fn new(size: usize) -> Game {
        let s: usize = size;
        let init_pos = Cell::new();

        let mut board: Vec<Vec<Cell>> = Vec::new();

        let mut row: Vec<Cell> = Vec::new();
        for _i in 0..size-1 {
            row.push(init_pos.clone());
        }
        for _i in 0..size-1 {
            board.push(row.clone());
        }
        return Game {
            board: board,
            len: s,
        };
    }

    // create board with random initial cells based on GENERATE_THRESHOLD
    pub fn init_board(&mut self, threshold: f32) {

        let mut rng = rand::thread_rng();
        
        //total amount of Cells in board
        let n = self.len-1;
        let mut col = self.board.iter_mut();
        for _i in 0..n {
            let curr_col = col.next().unwrap();
            let mut row_iter = curr_col.iter_mut();
            for _j in 0..n {
                let probabilty = rng.gen_range(0.0..1.0);
                let curr_pos = row_iter.next().unwrap();
                if probabilty > threshold {
                    curr_pos.toggle_state();
                }
            }
        }
    }


    pub fn update(&mut self) -> String {
        let mut str = String::new();
        for x in 0..self.len-1 {
            for y in 0..self.len-1 {

                let x_signed = x as i16;
                let y_signed = y as i16;
                let len = self.len;

                let y_bigger_one: bool = y_signed - 1 >= 0;
                let y_smaller_len: bool = y_signed + 1 <= (len as i16) - 2;
                let x_bigger_one: bool = x_signed - 1 >= 0;
                let x_smaller_len: bool = x_signed + 1 <= (len as i16) - 2;
        
                let mut count_alive_neighbors: u8 = 0;

                if x_bigger_one {
        
                    if self.board[x-1][y].eq(&State::Alive) {
                        count_alive_neighbors += 1;
                    }
        
                    if  y_bigger_one {
                        if self.board[x-1][y-1].eq(&State::Alive) {
                            count_alive_neighbors += 1;
                        }
                    }
                    if y_smaller_len {
                        if self.board[x-1][y+1].eq(&State::Alive) {
                            count_alive_neighbors += 1;
                        }
                    }
        
                }
                if x_smaller_len {
                    if self.board[x+1][y].eq(&State::Alive) {
                        count_alive_neighbors += 1;
                    }
        
                    if y_bigger_one {
                        if self.board[x+1][y-1].eq(&State::Alive) {
                            count_alive_neighbors += 1;
                        }
                    }
                    if y_smaller_len {
                        if self.board[x+1][y+1].eq(&State::Alive) {
                            count_alive_neighbors += 1;
                        }
                    }
                }
                if y_bigger_one {
                    if self.board[x][y-1].eq(&State::Alive) {
                        count_alive_neighbors += 1;
                    }
                }
                if y_smaller_len {
                    if self.board[x][y+1].eq(&State::Alive) {
                        count_alive_neighbors += 1;
                    }
                }
                self.board[x][y].update_cell(count_alive_neighbors);
                str.push(self.board[x][y].c);
            }        
            str.push('\n');
        }
        return str;    
    }
}