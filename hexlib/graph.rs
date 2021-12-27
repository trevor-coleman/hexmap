use crate::connection::Connection;
use crate::{Hex, NodeBundle, Tile};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Directed;
use std::collections::HashMap;
use std::ops::Index;

pub struct HexGraph {
    graph: Graph<Tile, Connection, Directed, u32>,
    index_to_hex: HashMap<NodeIndex<u32>, Hex>,
    hex_to_index: HashMap<Hex, NodeIndex<u32>>,
}

impl HexGraph {
    pub fn get_hex_from_index(&self, index: NodeIndex<u32>) -> Option<&Hex> {
        self.index_to_hex.get(&index)
    }

    pub fn get_index_from_hex(&self, hex: Hex) -> Option<&NodeIndex<u32>> {
        self.hex_to_index.get(&hex)
    }

    pub fn add_tile_at_hex(&mut self, hex: Hex, tile: Tile) -> (Hex, Tile, NodeIndex<u32>) {
        let mut graph = &self.graph;
        let index = graph.add_node(tile);
        self.index_to_hex.insert(index, hex);
        self.hex_to_index.insert(hex, index);

        return (hex, tile, index);
    }

    pub fn set_tile_at_index(&mut self, tile: &Tile, index: NodeIndex<u32>) -> Option<Tile> {}

    pub fn set_tile_at_hex(&mut self) {}

    pub fn set_tile_at_hex(&mut self) {}

    // write interface
}
