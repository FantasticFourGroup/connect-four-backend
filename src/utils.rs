use rand::Rng;

pub fn minimax(grid: Vec<Vec<u8>>, turn: u8) -> u8 {
  let cols = grid[0].len();
  let choice = rand::thread_rng().gen_range(0..cols);
  choice as u8
}