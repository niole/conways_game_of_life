#[cfg(test)]
mod tests {
    use wasm_game_of_life::Game;

    #[test]
    fn init_game() {
        assert_eq!(Ok(Game::new()), Game::new_board(vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]));
    }
}
