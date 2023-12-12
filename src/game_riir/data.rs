use bevy::{
    ecs::{bundle::Bundle, component::Component},
    sprite::SpriteSheetBundle,
};
use bevy_ecs_ldtk::{GridCoords, LdtkEntity};

#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct TravelPointBundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct ToiletBundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}
#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct TheracLinacBundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}
#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct TheracVTBundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}
#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct TheracPDP11Bundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}
#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct TranslucentShieldingBundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}
#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct ShowerBundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct RatBundle {
    actor: Actor,
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}
#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct BatBundle {
    actor: Actor,
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}
#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct CoinDoorBundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct BrokenCoinDoorBundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct SoiledMattressBundle {
    interactive: Interactive,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Component)]
pub(crate) struct Interactive;
#[derive(Default, Component)]
pub(crate) struct Actor;

#[derive(Default, Component)]
pub(crate) struct Player;
#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct PlayerBundle {
    actor: Actor,
    interactive: Interactive,
    player: Player,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Component)]
pub(crate) struct UICamera2D;

#[derive(Component)]
pub(crate) struct MyCamera2D;
#[derive(Component)]
pub(crate) struct MyCamera3D;
