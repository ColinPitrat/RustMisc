use argh::FromArgs;

#[derive(Clone, Default, FromArgs)]
/// A clicker for Storm the House (on crazygames.com)
pub struct CommandLineOptions {
    /// which screen to catpure from
    #[argh(option, default="0")]
    pub screen: usize,

    /// verbose output
    #[argh(switch, short='v')]
    pub verbose: bool,

    /// which game to play
    #[argh(option)]
    pub game: String,
}
