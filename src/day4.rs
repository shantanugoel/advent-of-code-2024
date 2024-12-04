use crate::utils::{self, Answer};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, time::Duration};

enum Direction {
    Up,
    Down,
    Left,
    Right,
    DiagonalUpForward,
    DiagonalUpBackward,
    DiagonalDownForward,
    DiagonalDownBackward,
}

#[derive(Debug, Clone, Copy)]
struct Coordinates {
    x: usize,
    y: usize,
    rows: usize,
    cols: usize,
}

impl Coordinates {
    fn step(self, direction: &Direction) -> Option<Self> {
        let mut target = self.clone();
        let updated = match direction {
            Direction::Up => {
                if target.y == 0 {
                    false
                } else {
                    target.y -= 1;
                    true
                }
            }
            Direction::Down => {
                if target.y == target.rows - 1 {
                    false
                } else {
                    target.y += 1;
                    true
                }
            }
            Direction::Left => {
                if target.x == 0 {
                    false
                } else {
                    target.x -= 1;
                    true
                }
            }
            Direction::Right => {
                if target.x == target.cols - 1 {
                    false
                } else {
                    target.x += 1;
                    true
                }
            }
            Direction::DiagonalUpForward => {
                if target.y == 0 || target.x == target.cols - 1 {
                    false
                } else {
                    target.y -= 1;
                    target.x += 1;
                    true
                }
            }
            Direction::DiagonalUpBackward => {
                if target.y == 0 || target.x == 0 {
                    false
                } else {
                    target.y -= 1;
                    target.x -= 1;
                    true
                }
            }
            Direction::DiagonalDownForward => {
                if target.y == target.rows - 1 || target.x == target.cols - 1 {
                    false
                } else {
                    target.y += 1;
                    target.x += 1;
                    true
                }
            }
            Direction::DiagonalDownBackward => {
                if target.y == target.rows - 1 || target.x == 0 {
                    false
                } else {
                    target.y += 1;
                    target.x -= 1;
                    true
                }
            }
        };
        if updated {
            Some(target)
        } else {
            None
        }
    }
}

fn get_input() -> Vec<String> {
    utils::read_lines("./inputs/day4")
}

fn search(input: Vec<String>, direction: &Direction, coords: Coordinates) -> bool {
    if let Some(next) = coords.step(direction) {
        if input[next.y].chars().nth(next.x).unwrap() == 'M' {
            if let Some(next) = next.step(direction) {
                if input[next.y].chars().nth(next.x).unwrap() == 'A' {
                    if let Some(next) = next.step(direction) {
                        if input[next.y].chars().nth(next.x).unwrap() == 'S' {
                            return true;
                        }
                    }
                }
            }
        }
    }
    return false;
}

pub fn part1() -> Answer {
    let lines = get_input();
    let cols = lines[0].len();
    let rows = lines.len();

    let mut result = 0;
    for row in 0..rows {
        for col in 0..cols {
            if lines[row].chars().nth(col).unwrap() != 'X' {
                continue;
            }
            let coords = Coordinates {
                x: col,
                y: row,
                rows,
                cols,
            };
            if search(lines.clone(), &Direction::Up, coords) {
                result += 1;
            }
            if search(lines.clone(), &Direction::Down, coords) {
                result += 1;
            }
            if search(lines.clone(), &Direction::Left, coords) {
                result += 1;
            }
            if search(lines.clone(), &Direction::Right, coords) {
                result += 1;
            }
            if search(lines.clone(), &Direction::DiagonalUpForward, coords) {
                result += 1;
            }
            if search(lines.clone(), &Direction::DiagonalUpBackward, coords) {
                result += 1;
            }
            if search(lines.clone(), &Direction::DiagonalDownForward, coords) {
                result += 1;
            }
            if search(lines.clone(), &Direction::DiagonalDownBackward, coords) {
                result += 1;
            }
        }
    }
    result.into()
}

pub fn part2() -> Answer {
    let lines = get_input();
    let cols = lines[0].len();
    let rows = lines.len();

    let mut result = 0;
    for row in 0..rows {
        for col in 0..cols {
            if lines[row].chars().nth(col).unwrap() != 'A' {
                continue;
            }
            let coords = Coordinates {
                x: col,
                y: row,
                rows,
                cols,
            };
            if let Some(left_top) = coords.step(&Direction::DiagonalUpBackward) {
                if let Some(left_bottom) = coords.step(&Direction::DiagonalDownBackward) {
                    if let Some(right_top) = coords.step(&Direction::DiagonalUpForward) {
                        if let Some(right_bottom) = coords.step(&Direction::DiagonalDownForward) {
                            let mas_1 = format!(
                                "{}{}",
                                lines[left_top.y].chars().nth(left_top.x).unwrap(),
                                lines[right_bottom.y].chars().nth(right_bottom.x).unwrap()
                            );
                            let mas_2 = format!(
                                "{}{}",
                                lines[right_top.y].chars().nth(right_top.x).unwrap(),
                                lines[left_bottom.y].chars().nth(left_bottom.x).unwrap()
                            );
                            if (mas_1 == "MS" || mas_1 == "SM") && (mas_2 == "MS" || mas_2 == "SM")
                            {
                                result += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    result.into()
}

pub fn visualize_part2() -> Answer {
    let lines = get_input();

    match visualize_impl(&lines) {
        Ok(_) => 0.into(),
        Err(e) => {
            println!("Visualization error: {}", e);
            0.into()
        }
    }
}

fn visualize_impl(lines: &[String]) -> io::Result<()> {
    let cols = lines[0].len();
    let rows = lines.len();

    // Initialize cell states
    let mut cell_states = vec![vec![CellState::Normal; cols]; rows];

    // Setup terminal
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut stars_found = 0;
    let mut current_row = 0;
    let mut current_col = 0;

    loop {
        // Process entire row before redrawing
        while current_col < cols && current_row < rows {
            if lines[current_row].chars().nth(current_col).unwrap() == 'A' {
                cell_states[current_row][current_col] = CellState::Current;

                // Check for star pattern
                let coords = Coordinates {
                    x: current_col,
                    y: current_row,
                    rows,
                    cols,
                };

                if let Some(left_top) = coords.step(&Direction::DiagonalUpBackward) {
                    if let Some(left_bottom) = coords.step(&Direction::DiagonalDownBackward) {
                        if let Some(right_top) = coords.step(&Direction::DiagonalUpForward) {
                            if let Some(right_bottom) = coords.step(&Direction::DiagonalDownForward)
                            {
                                let mas_1 = format!(
                                    "{}{}",
                                    lines[left_top.y].chars().nth(left_top.x).unwrap(),
                                    lines[right_bottom.y].chars().nth(right_bottom.x).unwrap()
                                );
                                let mas_2 = format!(
                                    "{}{}",
                                    lines[right_top.y].chars().nth(right_top.x).unwrap(),
                                    lines[left_bottom.y].chars().nth(left_bottom.x).unwrap()
                                );
                                if (mas_1 == "MS" || mas_1 == "SM")
                                    && (mas_2 == "MS" || mas_2 == "SM")
                                {
                                    stars_found += 1;
                                    // Mark star pattern cells
                                    cell_states[current_row][current_col] = CellState::Star;
                                    cell_states[left_top.y][left_top.x] = CellState::Star;
                                    cell_states[left_bottom.y][left_bottom.x] = CellState::Star;
                                    cell_states[right_top.y][right_top.x] = CellState::Star;
                                    cell_states[right_bottom.y][right_bottom.x] = CellState::Star;
                                }
                            }
                        }
                    }
                }
            }

            current_col += 1;
            if current_col >= cols {
                current_col = 0;
                current_row += 1;
                break;
            }

            // Only redraw when we've processed a full row
            if current_col % 1 == 0 {
                terminal.draw(|frame| {
                    let size = frame.area();

                    // Create a border around the entire screen
                    let main_block = Block::default()
                        .title("Day 4 Part 2 Visualization")
                        .borders(Borders::ALL);
                    let inner_area = main_block.inner(size);
                    frame.render_widget(main_block, size);

                    // Calculate cell size - now accounting for border
                    let cell_width = 1; // Single character width for compactness

                    // Calculate how many columns we can fit
                    let max_cols_per_row = (inner_area.width as usize - 2) / cell_width;

                    // Add debug information
                    let debug_info = format!(
                        "Grid: {}x{} | Pos: ({},{}) | Stars Found: {}",
                        rows, cols, current_row, current_col, stars_found
                    );
                    let debug_text =
                        Paragraph::new(debug_info).style(Style::default().fg(Color::White));
                    frame.render_widget(
                        debug_text,
                        ratatui::layout::Rect::new(inner_area.x, inner_area.y, inner_area.width, 1),
                    );

                    // Adjust starting y position to account for debug line
                    let grid_start_y = inner_area.y + 1;

                    // Only proceed if we have space for cells
                    for row in 0..rows {
                        for col in 0..cols {
                            let char = lines[row].chars().nth(col).unwrap();
                            let cell_state = cell_states[row][col];

                            let (fg_color, bg_color) = match cell_state {
                                CellState::Normal => (Color::White, Color::DarkGray),
                                CellState::Current => (Color::Black, Color::Red),
                                CellState::Star => (Color::Black, Color::Green),
                            };

                            // Show actual character for A's and star pattern cells, dots for others
                            let display_char = match cell_state {
                                CellState::Normal => ".".to_string(),
                                _ => char.to_string(),
                            };

                            let cell = Paragraph::new(display_char)
                                .style(Style::default().fg(fg_color).bg(bg_color));

                            // Calculate position in wrapped grid
                            let visual_row = row + (col / max_cols_per_row);
                            let visual_col = col % max_cols_per_row;

                            let cell_area = ratatui::layout::Rect::new(
                                inner_area.x + (visual_col * cell_width) as u16,
                                grid_start_y + visual_row as u16,
                                cell_width as u16,
                                1,
                            );

                            if visual_row < inner_area.height as usize {
                                frame.render_widget(cell, cell_area);
                            }
                        }
                    }
                })?;
            }
        }

        // Check for exit
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Exit if we've processed everything
        if current_row >= rows {
            break;
        }
    }

    // Cleanup
    disable_raw_mode()?;
    terminal.clear()?;
    Ok(())
}

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Normal,
    Current,
    Star,
}
