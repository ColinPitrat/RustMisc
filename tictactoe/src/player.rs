use crate::board::Board;

pub trait Player {
    // Notify the player that it's their turn to play.
    fn turn_starts(&mut self, board: &Board);

    // Query the player if they know what they want to play yet.
    // Returns None if the player is not yet decided, a position otherwise.
    fn move_to_play(&mut self) -> Option<(usize, usize)>;

    // Needed for human players only, to be aware of what to play.
    fn mouse_clicked(&mut self, _pos: (usize, usize)) {}

    // For models which need to save their learning at the end of the game
    fn save_model(&self) {}
}
