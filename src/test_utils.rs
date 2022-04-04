use super::*;

#[test]
fn test_valid_column() {
  let grid = vec![vec![0, 0, 0, 1], vec![0, 0, 0, 1], vec![0, 0, 0, 1], vec![0, 0, 0, 1]];
  assert_eq!(valid_column(&grid, 0), true);
  assert_eq!(valid_column(&grid, 1), true);
  assert_eq!(valid_column(&grid, 2), true);
  assert_eq!(valid_column(&grid, 3), false);
}

#[test]
fn test_drop_piece() {
  let mut grid = vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]];
  drop_piece(&mut grid, 0, 1);
  assert_eq!(grid, vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![1, 0, 0, 0]]);
  drop_piece(&mut grid, 0, 2);
  assert_eq!(grid, vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![2, 0, 0, 0], vec![1, 0, 0, 0]]);
  drop_piece(&mut grid, 0, 1);
  assert_eq!(grid, vec![vec![0, 0, 0, 0], vec![1, 0, 0, 0], vec![2, 0, 0, 0], vec![1, 0, 0, 0]]);
  drop_piece(&mut grid, 0, 1);
  assert_eq!(grid, vec![vec![1, 0, 0, 0], vec![1, 0, 0, 0], vec![2, 0, 0, 0], vec![1, 0, 0, 0]]);
  drop_piece(&mut grid, 0, 2);
  assert_eq!(grid, vec![vec![1, 0, 0, 0], vec![1, 0, 0, 0], vec![2, 0, 0, 0], vec![1, 0, 0, 0]]);
}

#[test]
fn test_winner() {
  // vertical
  let grid1 = vec![vec![0, 0, 0, 1], vec![0, 0, 0, 1], vec![0, 0, 0, 1], vec![0, 0, 0, 1]];
  assert_eq!(check_winner(&grid1, 1), true);
  assert_eq!(check_winner(&grid1, 2), false);
  // horizontal 
  let grid2 = vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![2, 2, 2, 2], vec![0, 0, 0, 0]];
  assert_eq!(check_winner(&grid2, 1), false);
  assert_eq!(check_winner(&grid2, 2), true);
  // positive diagonal
  let grid3 = vec![vec![0, 0, 0, 1], vec![0, 0, 1, 0], vec![0, 1, 0, 0], vec![1, 0, 0, 0]];
  assert_eq!(check_winner(&grid3, 1), true);
  assert_eq!(check_winner(&grid3, 2), false);
  // negative diagonal
  let grid4 = vec![vec![0, 0, 0, 1], vec![0, 0, 1, 0], vec![0, 1, 0, 0], vec![1, 0, 0, 0]];
  assert_eq!(check_winner(&grid4, 1), true);
  assert_eq!(check_winner(&grid4, 2), false);
}

#[test]
fn test_game_over() {
  // 1 wins
  let grid1 = vec![vec![0, 0, 0, 1], vec![0, 0, 0, 1], vec![0, 0, 0, 1], vec![0, 0, 0, 1]];
  assert_eq!(check_game_over(&grid1), true);

  // 2 wins
  let grid2 = vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![2, 2, 2, 2], vec![0, 0, 0, 0]];
  assert_eq!(check_game_over(&grid2), true);

  // no winner
  let grid3 = vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]];
  assert_eq!(check_game_over(&grid3), false);

  // draw game
  let grid4 = vec![vec![1, 1, 1, 2], vec![2, 2, 2, 1], vec![1, 1, 1, 2], vec![2, 2, 2, 1]];
  assert_eq!(check_game_over(&grid4), true);
}

#[test]
fn test_valid_columns() {
  let grid1 = vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]];
  assert_eq!(get_valid_columns(&grid1), vec![0, 1, 2, 3]);

  let grid2 = vec![vec![1, 0, 2, 0], vec![1, 0, 2, 0], vec![1, 0, 2, 0], vec![1, 0, 1, 0]];
  assert_eq!(get_valid_columns(&grid2), vec![1, 3]);
}

#[test] 
fn test_score_array() {
  let arr1 = vec![1, 1, 0, 0];
  assert_eq!(score_array(&arr1, 1), 10);

  let arr2 = vec![1, 1, 1, 0];
  assert_eq!(score_array(&arr2, 1), 30);

  let arr3 = vec![1, 1, 1, 1];
  assert_eq!(score_array(&arr3, 1), 100);

  let arr4 = vec![0, 2, 2, 0];
  assert_eq!(score_array(&arr4, 2), 10);

  let arr5 = vec![0, 2, 2, 2];
  assert_eq!(score_array(&arr5, 2), 30);

  let arr6 = vec![2, 2, 2, 2];
  assert_eq!(score_array(&arr6, 1), -100);
}

#[test] 
fn test_calc_position() {
  assert_eq!(calc_board_position(6, 7, 3, 3), 6);
  assert_eq!(calc_board_position(6, 7, 3, 2), 5);
  assert_eq!(calc_board_position(6, 7, 0, 0), 0);
  assert_eq!(calc_board_position(6, 7, 5, 6), 1);
}

#[test]
fn test_solve_board() {
  let board1 = vec![
                vec![0, 0, 0, 0, 0, 0, 0], 
                vec![0, 0, 0, 0, 0, 0, 0], 
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0], 
                vec![0, 0, 0, 0, 0, 0, 0], 
                vec![0, 0, 0, 1, 0, 0, 0]
              ];
  let (res, _) = solve_board(board1, 4, 2);
  assert_eq!(res, 3);

  let board2 = vec![
                vec![0, 0, 0, 0, 0, 0, 0], 
                vec![0, 0, 0, 0, 0, 0, 0], 
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0], 
                vec![0, 0, 0, 2, 0, 0, 0], 
                vec![0, 0, 0, 1, 0, 0, 0]
              ];
  let (res, _) = solve_board(board2, 4, 2);
  assert_eq!(res, 2);

  let board3 = vec![
                vec![0, 0, 0, 0, 0, 0, 0], 
                vec![0, 0, 0, 0, 0, 0, 0], 
                vec![0, 0, 0, 2, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0], 
                vec![0, 0, 0, 2, 0, 0, 0], 
                vec![0, 0, 0, 1, 1, 0, 0]
              ];
  let (res, _) = solve_board(board3, 4, 2);
  assert_eq!(res, 5);

  let board4 = vec![
                vec![0, 0, 0, 0, 0, 0, 0], 
                vec![0, 0, 0, 0, 0, 0, 0], 
                vec![0, 0, 0, 2, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0], 
                vec![0, 0, 0, 2, 1, 0, 0], 
                vec![0, 0, 2, 1, 1, 0, 0]
              ];
  let (res, _) = solve_board(board4, 4, 2);
  assert_eq!(res, 4);

  let board5 = vec![
                vec![0, 0, 0, 0, 0, 0, 0], 
                vec![0, 0, 0, 0, 0, 0, 0], 
                vec![0, 0, 0, 2, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0], 
                vec![0, 0, 0, 2, 1, 0, 0], 
                vec![0, 0, 2, 1, 1, 1, 0]
              ];
  let (res, _) = solve_board(board5, 4, 2);
  assert_eq!(res, 6);

  let grid6 = vec![
                vec![2, 1, 1, 2, 1, 1, 0], 
                vec![1, 2, 2, 1, 2, 2, 1], 
                vec![2, 1, 1, 2, 1, 1, 2],
                vec![1, 2, 2, 1, 2, 2, 1], 
                vec![2, 1, 1, 2, 1, 1, 2], 
                vec![1, 2, 2, 1, 2, 2, 1]
              ];
  let (res, game_state) = solve_board(grid6, 4, 2);
  assert_eq!(res, 6);
  assert_eq!(game_state, "Draw");

  let grid7 = vec![
                vec![0, 0, 0, 0, 0, 0, 0], 
                vec![0, 0, 0, 0, 0, 0, 0], 
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![1, 0, 0, 2, 0, 0, 0], 
                vec![1, 1, 0, 2, 2, 0, 0], 
                vec![1, 1, 1, 2, 2, 0, 0]
              ];
  let (res, _) = solve_board(grid7, 4, 2);
  assert_eq!(res, 3);
}