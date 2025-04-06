use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write, Result};

// Renders a single hexagon to test the appearance
fn main() -> Result<()> {
    let mut stdout = stdout();
    
    // Clear the screen
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    
    // Position of the hexagon
    let x = 5;
    let y = 3;
    
    // Colors
    let bg_color = Color::Rgb { r: 205, g: 170, b: 125 }; // Medium brown
    let text_color = Color::White;
    
    // Print the hexagon
    render_hexagon(&mut stdout, x, y, bg_color, text_color, "e4")?;
    
    // Move cursor down for prompt
    stdout.execute(cursor::MoveTo(0, 15))?;
    println!("Single hexagon rendered. How does it look?");
    
    stdout.flush()?;
    Ok(())
}

// Function to render a single hexagon
fn render_hexagon(
    stdout: &mut std::io::Stdout,
    x: u16,
    y: u16,
    bg_color: Color,
    text_color: Color,
    label: &str,
) -> Result<()> {
    // Top line
    stdout.execute(cursor::MoveTo(x + 4, y))?;
    stdout.execute(SetBackgroundColor(bg_color))?;
    stdout.execute(Print(" ___ "))?;
    stdout.execute(ResetColor)?;
    
    // Second line
    stdout.execute(cursor::MoveTo(x + 2, y + 1))?;
    stdout.execute(SetBackgroundColor(bg_color))?;
    stdout.execute(Print("/     \\"))?;
    stdout.execute(ResetColor)?;
    
    // Third line (middle with piece/content)
    stdout.execute(cursor::MoveTo(x + 1, y + 2))?;
    stdout.execute(SetBackgroundColor(bg_color))?;
    stdout.execute(SetForegroundColor(text_color))?;
    stdout.execute(Print("|       |"))?;
    stdout.execute(ResetColor)?;
    
    // Fourth line (with label)
    stdout.execute(cursor::MoveTo(x + 1, y + 3))?;
    stdout.execute(SetBackgroundColor(bg_color))?;
    stdout.execute(SetForegroundColor(text_color))?;
    let display = format!("   {}   ", label);
    stdout.execute(Print(format!("|{}|", display)))?;
    stdout.execute(ResetColor)?;
    
    // Fifth line
    stdout.execute(cursor::MoveTo(x + 1, y + 4))?;
    stdout.execute(SetBackgroundColor(bg_color))?;
    stdout.execute(Print("|       |"))?;
    stdout.execute(ResetColor)?;
    
    // Bottom line
    stdout.execute(cursor::MoveTo(x + 2, y + 5))?;
    stdout.execute(SetBackgroundColor(bg_color))?;
    stdout.execute(Print("\\_____ /"))?;
    stdout.execute(ResetColor)?;
    
    Ok(())
}