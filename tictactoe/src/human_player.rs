use crate::player::Player;
use crate::board::Board;

pub struct HumanPlayer {
    listening: bool,
    played_pos: Option<(usize, usize)>,
}

impl HumanPlayer {
    pub fn new() -> HumanPlayer {
        HumanPlayer{
            listening: false,
            played_pos: None,
        }
    }
}

impl Player for HumanPlayer {
    fn turn_starts(&mut self, _board: &Board) {
        self.listening = true;
    }

    fn move_to_play(&mut self) -> Option<(usize, usize)> {
        //assert!(self.listening, "HumanPlayer asked for move when not listening!");
        if let None = self.played_pos {
            None
        } else {
            let result = self.played_pos;
            self.played_pos = None;
            self.listening = false;
            result
        }
    }

    fn mouse_clicked(&mut self, pos: (usize, usize)) {
        // TODO: What if played_pos was already set?
        if self.listening {
            self.played_pos = Some(pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignore_events_when_not_listening() {
        let mut p = HumanPlayer::new();
        assert_eq!(None, p.move_to_play());
        p.mouse_clicked((0, 0));
        assert_eq!(None, p.move_to_play());
    }

    #[test]
    fn register_last_event_when_not_listening() {
        let mut p = HumanPlayer::new();
        assert_eq!(None, p.move_to_play());
        p.mouse_clicked((0, 0));
        assert_eq!(None, p.move_to_play());
    }
}
