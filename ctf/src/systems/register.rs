use component::{FlagCarrier, IsFlag, LastDrop};

use server::{Builder, Position, Team};
use specs::*;

use std::time::Instant;

use super::*;
use config;

pub fn register<'a, 'b>(world: &mut World, disp: Builder<'a, 'b>) -> Builder<'a, 'b> {
    world.register::<Team>();
    world.register::<Position>();
    world.register::<IsFlag>();
    world.register::<FlagCarrier>();
    world.register::<LastDrop>();

    let lastdrop = LastDrop {
        player: None,
        time: Instant::now(),
    };

    world
        .create_entity()
        .with(Team(1))
        .with(config::FLAG_POS[&Team(1)])
        .with(IsFlag {})
        .with(FlagCarrier(None))
        .with(lastdrop)
        .build();
    world
        .create_entity()
        .with(Team(2))
        .with(config::FLAG_POS[&Team(2)])
        .with(IsFlag {})
        .with(FlagCarrier(None))
        .with(lastdrop)
        .build();

    disp.with::<LoginUpdateSystem>()
        .with::<PickupFlagSystem>()
        .with::<SendFlagMessageSystem>()
        .with::<LeaveUpdateSystem>()
        .with::<DropSystem>()
        .with::<ReturnFlagSystem>()
        .with::<PosUpdateSystem>()
        .with::<PickupMessageSystem>()
        .with::<FlagSpeedSystem>()
        .with::<UpdateGameModeOnPlayerLeave>()
}