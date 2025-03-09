use crossterm::{
  cursor,
  style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
  terminal::{Clear, ClearType},
  ExecutableCommand,
};
use std::io::{stdout, Write, Result};

use crate::board::coordinates::{HexBoard, HexCoord, TileColor};

pub struct TerminalRenderer {
  board_size: i32,
}

impl TerminalRenderer {
  pub fn new(board_size: i32) -> Self {
      Self { board_size }
  }

  // Render the entire hexagonal board
  pub fn render_board(&self, board: &HexBoard) -> Result<()> {
      let mut stdout = stdout();

      // Clear the screen
      stdout.execute(Clear(ClearType::All))?;
      stdout.execute(cursor::MoveTo(0, 0))?;

      // Fixed dimensions for each hexagon
      let hex_width = 9;  // Increased for better spacing
      let hex_height = 3; 

      // Get all valid coordinates for this board
      let coords = board.all_coordinates();
      
      // Find coordinate bounds
      let min_q = coords.iter().map(|c| c.q).min().unwrap_or(0);
      let max_q = coords.iter().map(|c| c.q).max().unwrap_or(0);
      let min_r = coords.iter().map(|c| c.r).min().unwrap_or(0);
      let max_r = coords.iter().map(|c| c.r).max().unwrap_or(0);
      let min_s = coords.iter().map(|c| c.s()).min().unwrap_or(0);
      let max_s = coords.iter().map(|c| c.s()).max().unwrap_or(0);
      
      // Calculate base offsets (flattop orientation)
      let x_offset = 4;  // Starting X margin
      let y_offset = 2;  // Starting Y margin
      
      // Sort coordinates for rendering order (top to bottom)
      let mut sorted_coords = coords.clone();
      sorted_coords.sort_by_key(|c| (c.r, c.q));  // Sort by row then column
      
      // Render each hexagon using flat-top orientation
      for coord in &sorted_coords {
          // Calculate position
          // For flat-top hex grid:
          // - q axis goes right
          // - r axis goes down-right
          let q_pos = coord.q - min_q;
          let r_pos = coord.r - min_r;
          
          let x = x_offset + (hex_width * 3/4 * q_pos) as u16;
          let y = y_offset + (hex_height * r_pos + hex_height/2 * q_pos) as u16;
          
          if x < 200 && y < 50 {  // Safety bounds
              self.render_hex(&mut stdout, &board, coord, x, y)?;
          }
      }

      // Move cursor below the board
      let max_y = sorted_coords.iter()
          .map(|c| {
              let y_pos = (y_offset as i32) + 
                         (hex_height * (c.r - min_r)) + 
                         (hex_height/2 * (c.q - min_q)) + 
                         hex_height;
              y_pos as u16
          })
          .max()
          .unwrap_or(y_offset as u16);
          
      stdout.execute(cursor::MoveTo(0, max_y + 1))?;
      stdout.flush()?;
      
      Ok(())
  }

  // Render a single hexagon at the given screen coordinates
  fn render_hex(
      &self,
      stdout: &mut std::io::Stdout,
      board: &HexBoard,
      coord: &HexCoord,
      x: u16,
      y: u16,
  ) -> Result<()> {
      // Get the color for this hex
      let color = match board.tile_color(coord) {
          Some(TileColor::Light) => Color::Rgb { r: 240, g: 217, b: 181 },  // Light tan
          Some(TileColor::Medium) => Color::Rgb { r: 205, g: 170, b: 125 }, // Medium brown
          Some(TileColor::Dark) => Color::Rgb { r: 170, g: 136, b: 85 },    // Dark brown
          None => Color::Reset,
      };

      // Text color for better contrast
      let text_color = match board.tile_color(coord) {
          Some(TileColor::Light) => Color::Black,
          _ => Color::White,
      };

      // Draw the hexagon (flat-topped)
      // First line (top)
      stdout.execute(cursor::MoveTo(x, y))?;
      stdout.execute(SetBackgroundColor(color))?;
      stdout.execute(Print(" ___ "))?;
      stdout.execute(ResetColor)?;

      // Second line (middle with coordinates)
      stdout.execute(cursor::MoveTo(x, y + 1))?;
      stdout.execute(SetBackgroundColor(color))?;
      stdout.execute(SetForegroundColor(text_color))?;
      let display = format!("{},{}", coord.q, coord.r);
      stdout.execute(Print(format!("/{:^5}\\", display)))?;
      stdout.execute(ResetColor)?;
      
      // Third line (bottom)
      stdout.execute(cursor::MoveTo(x, y + 2))?;
      stdout.execute(SetBackgroundColor(color))?;
      stdout.execute(Print("\\___/"))?;
      stdout.execute(ResetColor)?;

      Ok(())
  }
}