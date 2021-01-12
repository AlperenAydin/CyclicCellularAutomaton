use itertools::iproduct;
use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
use std::ops::{Index, IndexMut};
use rayon::prelude::*;

// A struct to contain the grid and the various metadata
// The grid is 2d, but it is stored as a 1d vec.
// Here are the rules: Each has a state n from 0..num_states
// If the number of its neighbours whose state is n+1
// is greater than threshold, the cell is consumed
// And its state becomes n+1
// Remember that we are operating in module space.
// Meaning that its cycles back. num_states + 1 -> 0
// i = x + y*width
// (0,0), (1,0)
// (0,1), (1,1)
// => [(0,0), (1,0), (0,1), (1,1)]
pub struct CyclicAutomaton {
    pub grid: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub num_states: u8,
    pub threshold: usize,
    pub neighbours: Vec<[usize; 8]>,
}

impl CyclicAutomaton {
    // Creates new automaton, with the value initialized to 0.
    pub fn new(width: usize, height: usize, num_states: u8, threshold: usize) -> CyclicAutomaton {
        let mut new_automaton = CyclicAutomaton {
            grid: vec![0; width * height],
            width,
            height,
            num_states,
            threshold,
            neighbours: vec![[0 as usize; 8]; width * height],
        };
        for index in 0..(width * height) {
            new_automaton.neighbours[index] = CyclicAutomaton::get_neighbours(index, width, height);
        }
        new_automaton
    }
    // Randomizes the grid from a seed
    pub fn randomize(&mut self, seed: u64) {
        let rng = StdRng::seed_from_u64(seed);
        let dist = Uniform::from(0..self.num_states);
        let size = self.width * self.height;
        self.grid = rng.sample_iter(&dist).take(size).collect();
    }
    // Return all 8 neighbours of the cell located at index
    fn get_neighbours(index: usize, width: usize, height: usize) -> [usize; 8] {
        // The neighbours of (x,y) is (x+a, x+b)
        // st. for (a,b) in [-1,0,1]**2 except (0,0)*
        let x = (index % width) as i32;
        let y = (index / width) as i32;
        let mask2coords = |(a, b): (i32, i32)| {
            (
                (x + a).rem_euclid(width as i32),
                (y + b).rem_euclid(height as i32),
            )
        };
        let neighbours = iproduct!(-1..=1, -1..=1) // Mask for each direction
            .filter(|&p| p != (0, 0)) // Removes the center
            .map(mask2coords) // Turns the mask into 2d coordinates
            .map(|(a, b)| (a as usize, b as usize))
            .map(|(a, b)| a + (b * width)); // Turns coords into index
        let mut ret: [usize; 8] = [0; 8]; // The return value
        // Filling in the return value
        for (i, val) in neighbours.enumerate() {
            ret[i] = val;
        }
        ret
    }
    // Updates the cell at (x,y)
    // According to the rue explained above
    pub fn update_cell(&self, index: usize) -> u8 {
        let curr_state = self.grid[index];
        let next_state = (curr_state + 1) % self.num_states;
        
        let neighbour_count = self.neighbours[index]
            .iter()
            .filter(|&i| self.grid[*i] == next_state)
            .count();
        if neighbour_count >= self.threshold {
            return next_state;
        }
        curr_state
    }

    pub fn next_generation(&mut self) {
        let index_range = 0..(self.width * self.height);
        self.grid = index_range.into_par_iter().map(|index| self.update_cell(index)).collect();
    }

}

// This was previously useful. But I will live it in just in case.
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

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: Write more and better tests

    #[test]
    fn test_get_neighbours() {
        let ground_truth: [usize; 8] = [99, 399, 699, 100, 700, 101, 401, 701];
        let neighbours = CyclicAutomaton::get_neighbours(400, 300, 300);
        assert_eq!(neighbours, ground_truth);
        //assert_eq!(0,1);

        let ground_truth: [usize; 8] = [149899, 399, 899, 149900, 900, 149901, 401, 901];
        let neighbours = CyclicAutomaton::get_neighbours(400, 500, 300);
        assert_eq!(neighbours, ground_truth);
    }
}
