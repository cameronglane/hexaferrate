mod board;
mod ui;

use board::coordinates::HexBoard;
use ui::renderer::TerminalRenderer;
use std::io::{stdin, stdout, Read, Write, Result};

fn main() -> Result<()> {
    println!("Welcome to Hexaferrate - Hexagonal Chess in Rust!");
    println!("Press any key to display the board...");
    
    // Wait for a key press
    stdout().flush()?;
    stdin().read(&mut [0])?;
    
    // Create a hexagonal board with side length 6
    let board = HexBoard::new(6);
    
    // Debug: Print color of a few hexes
    let center = board::coordinates::HexCoord::new(0, 0);
    println!("Center hex color: {:?}", board.tile_color(&center));
    
    let top = board::coordinates::HexCoord::new(0, -5);
    println!("Top hex color: {:?}", board.tile_color(&top));
    
    // Create a renderer and display the board
    let renderer = TerminalRenderer::new(6);
    renderer.render_board(&board)?;
    
    println!("\nBoard displayed. Press any key to exit...");
    stdout().flush()?;
    stdin().read(&mut [0])?;
    
    Ok(())
}