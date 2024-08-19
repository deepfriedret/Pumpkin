use pumpkin_macros::packet;
use serde::Serialize;

use crate::{position::WorldPosition, VarInt};

#[derive(Serialize)]
#[packet(0x2B)]
pub struct CLogin<'a> {
    entity_id: i32,
    is_hardcore: bool,
    dimension_count: VarInt,
    dimension_names: &'a [&'a str],
    max_players: VarInt,
    view_distance: VarInt,
    simulated_distance: VarInt,
    reduced_debug_info: bool,
    enabled_respawn_screen: bool,
    limited_crafting: bool,
    dimension_type: VarInt,
    dimension_name: &'a str,
    hashed_seed: i64,
    game_mode: u8,
    previous_gamemode: i8,
    debug: bool,
    is_flat: bool,
    death_dimension_name: Option<(WorldPosition, i64)>,
    portal_cooldown: VarInt,
    enforce_secure_chat: bool,
}

impl<'a> CLogin<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        entity_id: i32,
        is_hardcore: bool,
        dimension_names: &'a [&'a str],
        max_players: VarInt,
        view_distance: VarInt,
        simulated_distance: VarInt,
        reduced_debug_info: bool,
        enabled_respawn_screen: bool,
        limited_crafting: bool,
        dimension_type: VarInt,
        dimension_name: &'a str,
        hashed_seed: i64,
        game_mode: u8,
        previous_gamemode: i8,
        debug: bool,
        is_flat: bool,
        death_dimension_name: Option<(WorldPosition, i64)>,
        portal_cooldown: VarInt,
        enforce_secure_chat: bool,
    ) -> Self {
        Self {
            entity_id,
            is_hardcore,
            dimension_count: VarInt(dimension_names.len() as i32),
            dimension_names,
            max_players,
            view_distance,
            simulated_distance,
            reduced_debug_info,
            enabled_respawn_screen,
            limited_crafting,
            dimension_type,
            dimension_name,
            hashed_seed,
            game_mode,
            previous_gamemode,
            debug,
            is_flat,
            death_dimension_name,
            portal_cooldown,
            enforce_secure_chat,
        }
    }
}
