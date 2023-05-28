#[cfg(test)]
mod tests {
    use wasm_game_of_life::Game;

    #[test]
    fn init_game() {
        assert_eq!(Ok(Game::new()), Game::new_board(vec![1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]));
    }

    #[test]
    fn get_live_neighbor_count() {
        let game = Game::new();
        let n_count = game.get_alive_neighbor_count(5);
        assert_eq!(n_count, 3);
    }

    #[test]
    fn one_tick() {
        let mut game: Game = Game::new();
        game.on_tick();
        assert_eq!(
            Ok(game),
            Game::new_board(vec![0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
        );
    }

    #[test]
    fn two_ticks() {
        let mut game: Game = Game::new();
        game.on_tick();
        game.on_tick();
        assert_eq!(
            Ok(game),
            Game::new_board(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
        );
    }
}
