#[cfg(test)]
mod tests {
    use wasm_game_of_life::Game;

    #[test]
    fn init_game() {
        assert_eq!(Game::new(), Game::new());
    }
}
