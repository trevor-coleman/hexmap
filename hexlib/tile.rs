use crate::hexlib::hex::Hex;
use crate::tile_atlas::{BaseTileType, TileColor, TileType};
use crate::Color;
use bevy_inspector_egui::Inspectable;
use petgraph::graph::NodeIndex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Inspectable)]
pub struct Tile {
    pub node_index: u32,
    pub hex: Hex,
    #[inspectable(min = 0, max = 4)]
    pub population: u8,
    pub selected: bool,
}

impl Tile {
    pub const fn new(hex: Hex) -> Tile {
        Tile {
            hex,
            population: 0,
            selected: false,
            node_index: 0,
        }
    }

    pub fn tint_color(&self) -> Color {
        let population_percent = 1.0 - (self.population as f32 / 4.0);

        Color::rgb(1.0 - population_percent, 1.0, 1.0)
    }

    pub fn tile_code(&self) -> u32 {
        TileType(BaseTileType::Grass, TileColor::White).value()

        // let color: TileColor = match self.population {
        //     1 => TileColor::Yellow,
        //     2 => TileColor::Orange,
        //     3 => TileColor::Red,
        //     _ => TileColor::Green,
        // };
        // let base_tile_type: BaseTileType = BaseTileType::Grass;
        //
        // TileType(base_tile_type, color).value()
    }
}
