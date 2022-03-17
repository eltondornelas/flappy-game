use crate::states::State;
use bracket_lib::prelude::*;

mod player;
mod states;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
