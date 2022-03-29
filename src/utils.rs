enum GameState {
  Win, 
  Lose,
  Playing,
  Draw
}

impl GameState {
  fn to_string(&self) -> String {
    match self {
      GameState::Win => "Win".to_string(),
      GameState::Lose => "Lose".to_string(),
      GameState::Playing => "Playing".to_string(),
      GameState::Draw => "Draw".to_string(),
    }
  }
}

fn valid_column(grid: &Vec<Vec<usize>>, col: usize) -> bool {
  grid[0][col] == 0
}

fn drop_piece(grid: &mut Vec<Vec<usize>>, col: usize, turn: usize) {
  if !valid_column(grid, col) {
    return;
  }
  let rows = grid.len();
  for i in (0..rows).rev() {
    if grid[i][col] == 0 {
      grid[i][col] = turn;
      break;
    }
  }
}

fn check_winner(grid: &Vec<Vec<usize>>, turn: usize) -> bool {
  let rows = grid.len();
  let cols = grid[0].len();

  for i in 0..rows {
    for j in 0..(cols - 3) {
      if grid[i][j] == turn && grid[i][j + 1] == turn && grid[i][j + 2] == turn && grid[i][j + 3] == turn {
        return true;
      }
    }
  }

  for i in 0..(rows - 3) {
    for j in 0..cols {
      if grid[i][j] == turn && grid[i + 1][j] == turn && grid[i + 2][j] == turn && grid[i + 3][j] == turn {
        return true;
      }
    }
  }

  for i in 0..(rows - 3) {
    for j in 0..(cols - 3) {
      if grid[i][j] == turn && grid[i + 1][j + 1] == turn && grid[i + 2][j + 2] == turn && grid[i + 3][j + 3] == turn {
        return true;
      }
    }
  }

  for i in 0..(rows - 3) {
    for j in 3..cols {
      if grid[i][j] == turn && grid[i + 1][j - 1] == turn && grid[i + 2][j - 2] == turn && grid[i + 3][j - 3] == turn {
        return true;
      }
    }
  }

  false 
}

fn count_pieces(arr: &Vec<usize>, turn: usize) -> usize {
  arr.iter().filter(|&x| *x == turn).count()
}

fn score_array(arr: &Vec<usize>, turn: usize) -> isize {
  let mut score = 0;
  let opp_turn = if turn == 1 { 2 } else { 1 };
  let count = count_pieces(arr, turn);
  let opp_count = count_pieces(arr, opp_turn);
  let empty_count = count_pieces(arr, 0);

  if count == 4 {
    score += 100;
  } 
  else if count == 3 && empty_count == 1 {
    score += 5;
  } 
  else if count == 2 && empty_count == 2 {
    score += 1;
  }
  if opp_count == 4 {
    score -= 100;
  }
  else if opp_count == 3 && empty_count == 1 {
    score -= 5;
  } 
  else if opp_count == 2 && empty_count == 2 {
    score -= 1;
  }

  score
}

fn calc_heuristic(grid: &Vec<Vec<usize>>, turn: usize) -> isize {
  let mut heuristic = 0;
  let rows = grid.len();
  let cols = grid[0].len();

  let center_col = cols / 2;
  for i in 0..rows {
    if grid[i][center_col] == turn {
      heuristic += 3;
    }
    let opp_turn = if turn == 1 { 2 } else { 1 };
    if grid[i][center_col] == opp_turn {
      heuristic -= 3;
    }
  }

  for i in 0..rows {
    for j in 0..(cols - 3) {
      let mut horizontal: Vec<usize> = Vec::new();
      for k in 0..4 {
        horizontal.push(grid[i][j+k]);
      }
      heuristic += score_array(&horizontal, turn);
    }
  }

  for j in 0..cols {
    for i in 0..(rows - 3) {
      let mut vertical: Vec<usize> = Vec::new();
      for k in 0..4 {
        vertical.push(grid[i+k][j]);
      }
      heuristic += score_array(&vertical, turn);
    }
  }

  for i in 0..(rows - 3) {
    for j in 0..(cols - 3) {
      let mut positive_diagonal: Vec<usize> = Vec::new();
      for k in 0..4 {
        positive_diagonal.push(grid[i+k][j+k]);
      }
      heuristic += score_array(&positive_diagonal, turn);
    }
  }

  for i in 0..(rows - 3) {
    for j in 3..cols {
      let mut negative_diagonal: Vec<usize> = Vec::new();
      for k in 0..4 {
        negative_diagonal.push(grid[i+k][j-k]);
      }
      heuristic += score_array(&negative_diagonal, turn);
    }
  }

  heuristic
}

fn get_valid_columns(grid: &Vec<Vec<usize>>) -> Vec<usize> {
  let mut valid_cols = Vec::new();
  for i in 0..grid[0].len() {
    if valid_column(grid, i) {
      valid_cols.push(i);
    }
  }
  valid_cols
}

fn check_game_over(grid: &Vec<Vec<usize>>) -> bool {
  if check_winner(grid, 1) || check_winner(grid, 2) {
    return true;
  }
  if get_valid_columns(grid).len() == 0 {
    return true;
  }
  false
}

fn is_draw(grid: &Vec<Vec<usize>>) -> bool {
  if get_valid_columns(grid).len() == 0 {
    return true;
  }
  false
}

fn minimax(grid: Vec<Vec<usize>>, depth: usize, player_piece: usize, ai_piece: usize, is_mini: bool) -> (Option<usize>, isize, GameState) {
  let valid_cols = get_valid_columns(&grid);
  if valid_cols.len() == 0 {
    return (None, 0, GameState::Draw);
  }
  let is_game_over = check_game_over(&grid);
  if depth == 0 || is_game_over {
    if is_game_over {
      if check_winner(&grid, player_piece) {
        return (None, -1000000, GameState::Win);
      }
      else if check_winner(&grid, ai_piece) {
        return (None, 1000000, GameState::Lose);
      }
      else {
        return (None, 0, GameState::Draw);
      }
    }
    else {
      return (None, calc_heuristic(&grid, ai_piece), GameState::Playing);
    }
  }
  if is_mini {
    let mut best_score: isize = 100000;
    let mut best_col = valid_cols[0];
    for col in valid_cols {
      let mut new_grid = grid.clone();
      drop_piece(&mut new_grid, col, player_piece);
      let (_, new_score, _) = minimax(new_grid, depth - 1, player_piece, ai_piece, false);
      if new_score < best_score {
        best_score = new_score;
        best_col = col;
      }
    }
    return (Some(best_col), best_score, GameState::Playing);
  }
  else {
    let mut best_score: isize = -100000;
    let mut best_col = valid_cols[0];
    for col in valid_cols {
      let mut new_grid = grid.clone();
      drop_piece(&mut new_grid, col, ai_piece);
      let (_, new_score, _) = minimax(new_grid, depth - 1, player_piece, ai_piece, true);
      if new_score > best_score {
        best_score = new_score;
        best_col = col;
      }
    }
    return (Some(best_col), best_score, GameState::Playing);
  }
}

pub fn solve_board(grid: Vec<Vec<usize>>, depth: usize, ai_piece: usize) -> (usize, String) {
  let player_piece = if ai_piece == 1 { 2 } else { 1 };

  if check_winner(&grid, player_piece) {
    return (0, "Win".to_string());
  }

  if is_draw(&grid) {
    return (0, "Draw".to_string());
  }	

  let mut grid_copy = grid.clone();

  let (col, _, game_state) = minimax(grid, depth, player_piece, ai_piece, false);

  match col {
    Some(c) => {
      drop_piece(&mut grid_copy, c, ai_piece);
      if check_winner(&grid_copy, ai_piece) {
        return (c, "Lose".to_string());
      }
      else {
        return (c, game_state.to_string());
      }
    },
    None => {
      return (0, game_state.to_string());
    }
  }
}

#[cfg(test)]
#[path="test_utils.rs"]
mod test_utils;