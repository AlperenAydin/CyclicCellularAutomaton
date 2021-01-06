use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
use std::ops::{Index, IndexMut};

// A struct to contain the grid and the various metadata
// The grid is 2d, but it is stored as a 1d vec.
// Here are the rules: Each has a state n from 0..num_states
// If the number of its neighbours whose state is n+1
// is greater than threshold, the cell is consumed
// And its state becomes n+1
// Remember that we are operating in module space. 
// Meaning that its cycles back. num_states + 1 -> 0
pub struct CyclicAutomaton {
    pub grid: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub num_states: u8,
    pub threshold: usize,
}

impl CyclicAutomaton {
    // Creates new automaton, with the value initialized to 0.
    pub fn new(width: usize, height: usize, num_states: u8, threshold: usize) -> CyclicAutomaton {
        CyclicAutomaton {
            grid: vec![0; width * height],
            width,
            height,
            num_states,
            threshold,
        }
    }
    // Randomizes the grid from a seed
    pub fn randomize(&mut self, seed: u64) {
        let rng = StdRng::seed_from_u64(seed);
        let dist = Uniform::from(0..self.num_states);
        let size = self.width * self.height;
        self.grid = rng.sample_iter(&dist).take(size).collect();
    }
    // Return all 8 of the cell located at (x,y)
    fn neighbours(x: usize, y: usize) -> [(i32, i32); 8] {
        let mut cells = [(0 as i32, 0 as i32); 8];
        let (x, y) = (x as i32, y as i32);
        // Starting on the left, going clockwise
        cells[0] = (x - 1, y);
        cells[1] = (x - 1, y + 1);
        cells[2] = (x, y + 1);
        cells[3] = (x + 1, y + 1);
        cells[4] = (x + 1, y);
        cells[5] = (x + 1, y - 1);
        cells[6] = (x, y - 1);
        cells[7] = (x - 1, y - 1);
        cells
    }
    // Updates the cell at (x,y)
    // According to the rue explained above
    pub fn update_cell(&self, x: usize, y: usize) -> u8 {
        let next_state = (self[(x, y)] + 1) % self.num_states;
        let count = CyclicAutomaton::neighbours(x, y)
            .iter()
            .filter(|(n_x, n_y)| self[(*n_x, *n_y)] == next_state).
            count();

        if count > self.threshold{
            return next_state;
        } else {
            return self[(x,y)];
        }
    }

    pub fn next_generation(&mut self)
    {
        let mut new_grid = vec![0; self.width*self.height];
        for x in 0 .. self.width
        {
            for y in 0..self.height{
                let i = x + y * self.width;
                new_grid[i]= self.update_cell(x, y);
            }
        }
        self.grid = new_grid;
    }
}

impl Index<(i32, i32)> for CyclicAutomaton {
    type Output = u8;
    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        let x = (x as usize).rem_euclid(self.width);
        let y = (y as usize).rem_euclid(self.height);
        &self.grid[x + y * self.width]
    }
}

impl Index<(usize, usize)> for CyclicAutomaton {
    type Output = u8;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let x = (x as usize).rem_euclid(self.width);
        let y = (y as usize).rem_euclid(self.height);
        &self.grid[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for CyclicAutomaton {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let x = (x as usize).rem_euclid(self.width);
        let y = (y as usize).rem_euclid(self.height);
        &mut self.grid[x + y * self.width]
    }
}
