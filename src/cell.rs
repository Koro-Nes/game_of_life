#[derive(PartialEq, Clone, Debug)]
pub enum State {
    Alive,
    Dead
}

#[derive(Clone, Debug)]
pub struct Cell {
    state: State,
    pub c: char,
}

impl Cell {
    pub fn new() -> Cell {
        Self {
            state: State::Dead,
            c: ' ',
        }
    } 

    pub fn toggle_state(&mut self) {
        if self.eq(&State::Dead) {
            self.state = State::Alive;
            self.c = '█';
        } else {
            self.state = State::Dead;
            self.c = ' ';
        }
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
        if self.eq(&State::Alive) {
            self.c = '█';
        } else {
            self.c = ' ';
        }
    }

    pub fn eq(&self, state: &State) -> bool {
        if self.state.eq(state) {
            return true;
        }
        return false;
    }
    pub fn update_cell(&mut self, count_alive: u8) {

        match self.eq(&State::Alive) {
            true => {
                // Cells with fewer than 2 alive neighbors die from underpopulation,
                // Cells with more than 3 alive neighbors die from overpopulation.
                if count_alive < 2 || count_alive > 3 {
                    self.set_state(State::Dead);
                }
            }
            false => {
                if count_alive == 3 {
                    self.set_state(State::Alive);
                }
            }
        }
        
    }
}