use argh::FromArgs;
use std::sync::{LazyLock,RwLock};

pub fn get_opts() -> EulerOpts {
    EulerOpts::get_opts()
}

pub fn set_opts(opts: EulerOpts) {
    EulerOpts::set_opts(opts)
}

#[derive(Clone, Default, FromArgs)]
/// Solution for Project Euler (http://projecteuler.net).
pub struct EulerOpts {
    /// the file to use as input
    #[argh(option)]
    pub filename: Option<String>,

    /// the problem to solve
    #[argh(option)]
    pub problem: i64,

    /// verbose output
    #[argh(switch, short = 'v')]
    pub verbose: bool,
}

impl EulerOpts {
    fn get_opts() -> Self {
        let o = OPTIONS.read().unwrap();
        if let Some(opts) = o.as_ref() {
            opts.clone()
        } else {
            Self{
                ..Default::default()
            }
        }
    }

    fn set_opts(opts: Self) {
        let mut o = OPTIONS.write().unwrap();
        *o = Some(opts);
    }
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
static OPTIONS: LazyLock<RwLock<Option<EulerOpts>>> = std::sync::LazyLock::new(|| RwLock::new(None));
