use crate::utils::{self, Answer};
use crossterm::{
    event::{self, Event, KeyCode, MouseEvent, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use regex::Regex;
use std::{
    io,
    time::{Duration, Instant},
};

fn get_input() -> Vec<(i64, i64)> {
    let lines = utils::read_lines("./inputs/day3");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut input: Vec<(i64, i64)> = Vec::new();
    for line in lines.iter() {
        for (_, [a, b]) in re.captures_iter(line).map(|c| c.extract()) {
            input.push((a.parse().unwrap(), b.parse().unwrap()));
        }
    }
    input
}

pub fn part1() -> Answer {
    let input = get_input();
    let mut sum: i64 = 0;
    for (a, b) in input.iter() {
        sum += a * b;
    }
    sum.into()
}

#[derive(Debug, PartialEq, Eq)]
enum MulState {
    Enabled,
    Disabled,
}

fn get_input_part2() -> Vec<(i64, i64)> {
    let lines = utils::read_lines("./inputs/day3");
    let re = Regex::new(r"(do)\(\)|(don)\'t\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut input: Vec<(i64, i64)> = Vec::new();
    let mut state = MulState::Enabled;
    for line in lines.iter() {
        for capture in re.captures_iter(line) {
            if capture.get(1).is_some() {
                state = MulState::Enabled;
            } else if capture.get(2).is_some() {
                state = MulState::Disabled;
            } else if state == MulState::Enabled {
                let a = capture.get(3).unwrap().as_str();
                let b = capture.get(4).unwrap().as_str();
                input.push((a.parse().unwrap(), b.parse().unwrap()));
            }
        }
    }
    input
}

pub fn part2() -> Answer {
    let input = get_input_part2();
    let mut sum: i64 = 0;
    for (a, b) in input.iter() {
        sum += a * b;
    }
    sum.into()
}

#[derive(Debug)]
struct VisState {
    text: String,
    current_pos: usize,
    scroll_offset: usize,
    muls: Vec<(i64, i64, i64)>, // (a, b, result)
    current_highlight: Option<(usize, usize)>,
    turbo_mode: bool,
    running_sum: i64,
}

pub fn visualize_part2() -> Answer {
    // Terminal initialization
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();

    // Enable mouse support before creating terminal
    execute!(
        stdout,
        EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )
    .unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // Get input text
    let lines = utils::read_lines("./inputs/day3");
    let text = lines.join("\n");

    let mut state = VisState {
        text,
        current_pos: 0,
        scroll_offset: 0,
        muls: Vec::new(),
        current_highlight: None,
        turbo_mode: false,
        running_sum: 0,
    };

    let re = Regex::new(r"(do)\(\)|(don)\'t\(\)|mul\((\d+),(\d+)\)").unwrap();
    let mut mul_state = MulState::Enabled;
    let mut last_update = Instant::now();
    let base_scroll_speed = Duration::from_millis(50);
    let mut paused_until = None;
    let mut display_width;

    loop {
        // Calculate display width based on terminal size
        let terminal_width = terminal.size().unwrap().width as usize;
        display_width = terminal_width.saturating_sub(24); // Account for borders and sum display

        terminal
            .draw(|f| {
                let total_size = f.area();

                // Create a vertical split for the main content and sum display
                let main_and_sum = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [
                            Constraint::Min(30),    // Main content
                            Constraint::Length(20), // Sum display
                        ]
                        .as_ref(),
                    )
                    .split(total_size);

                // Split the main content vertically
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(3), // Text display
                            Constraint::Min(0),    // Results
                            Constraint::Length(3), // Turbo button
                        ]
                        .as_ref(),
                    )
                    .split(main_and_sum[0]);

                // Calculate visible portion of text
                let text = state.text.as_str();
                let start_idx = state.scroll_offset;
                let end_idx = (start_idx + display_width).min(text.len());
                let visible_text = &text[start_idx..end_idx];

                let display_text =
                    if let Some((highlight_start, highlight_end)) = state.current_highlight {
                        if highlight_start >= start_idx && highlight_start < end_idx {
                            let rel_start = highlight_start - start_idx;
                            let rel_end = (highlight_end - start_idx).min(display_width);
                            vec![
                                Span::raw(&visible_text[..rel_start]),
                                Span::styled(
                                    &visible_text[rel_start..rel_end],
                                    Style::default().fg(Color::Yellow),
                                ),
                                Span::raw(&visible_text[rel_end..]),
                            ]
                        } else {
                            vec![Span::raw(visible_text)]
                        }
                    } else {
                        vec![Span::raw(visible_text)]
                    };

                let progress = format!(" [{}/{}]", state.current_pos, text.len());
                let text_widget = Paragraph::new(Line::from(display_text)).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("Input{}", progress)),
                );
                f.render_widget(text_widget, chunks[0]);

                // Display multiplication results
                let results: Vec<ListItem> = state
                    .muls
                    .iter()
                    .rev() // Reverse the iterator to show newest items first
                    .map(|(a, b, result)| ListItem::new(format!("mul({}, {}) = {}", a, b, result)))
                    .collect();

                let results_widget = List::new(results)
                    .block(Block::default().borders(Borders::ALL).title("Results"));
                f.render_widget(results_widget, chunks[1]);

                // Turbo mode button
                let turbo_style = if state.turbo_mode {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                let turbo_text = if state.turbo_mode {
                    "TURBO MODE: ON"
                } else {
                    "TURBO MODE: OFF"
                };
                let turbo_widget = Paragraph::new(turbo_text)
                    .style(turbo_style)
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(ratatui::layout::Alignment::Center);
                f.render_widget(turbo_widget, chunks[2]);

                // Display running sum
                let sum_style = Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD);
                let sum_text = format!("{}", state.running_sum);
                let sum_widget = Paragraph::new(sum_text)
                    .style(sum_style)
                    .block(Block::default().borders(Borders::ALL).title("Running Sum"))
                    .alignment(ratatui::layout::Alignment::Center);
                f.render_widget(sum_widget, main_and_sum[1]);
            })
            .unwrap();

        // Process input
        if event::poll(Duration::from_millis(10)).unwrap() {
            match event::read().unwrap() {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('t') | KeyCode::Char('T') => {
                        state.turbo_mode = !state.turbo_mode;
                    }
                    _ => {}
                },
                Event::Mouse(MouseEvent {
                    kind: MouseEventKind::Down(event::MouseButton::Left),
                    row,
                    column: _,
                    ..
                }) => {
                    // Check if click is in turbo button area
                    if row == terminal.get_frame().area().height - 2 {
                        state.turbo_mode = !state.turbo_mode;
                    }
                }
                _ => {}
            }
        }

        // Update visualization state
        let now = Instant::now();

        // If we're paused, check if we should resume
        if let Some(pause_end) = paused_until {
            if now < pause_end {
                continue;
            }
            paused_until = None;
        }

        // Calculate current scroll speed based on turbo mode
        let scroll_speed = if state.turbo_mode {
            base_scroll_speed / 1000 // 1000x faster in turbo mode
        } else {
            base_scroll_speed
        };

        // Only update if enough time has passed since last update
        if now.duration_since(last_update) >= scroll_speed {
            if state.current_pos < state.text.len() {
                state.current_pos += 1;

                // Update scroll offset to keep the current position visible
                if state.current_pos > state.scroll_offset + (display_width / 2) {
                    state.scroll_offset = (state.current_pos - (display_width / 2))
                        .min(state.text.len().saturating_sub(display_width));
                }

                state.current_highlight = None;

                let current_text = &state.text[..state.current_pos];
                for capture in re.captures_iter(current_text) {
                    if capture.get(1).is_some() {
                        mul_state = MulState::Enabled;
                    } else if capture.get(2).is_some() {
                        mul_state = MulState::Disabled;
                    } else if mul_state == MulState::Enabled {
                        if let (Some(a_match), Some(b_match)) = (capture.get(3), capture.get(4)) {
                            let full_match = capture.get(0).unwrap();
                            let match_end = full_match.end();

                            // Only process if this mul operation ends at our current position
                            if match_end == state.current_pos {
                                let a: i64 = a_match.as_str().parse().unwrap();
                                let b: i64 = b_match.as_str().parse().unwrap();
                                let result = a * b;

                                // Update running sum
                                state.running_sum += result;

                                // Highlight the current mul
                                state.current_highlight =
                                    Some((full_match.start(), full_match.end()));

                                // Add to results if not already present
                                if !state.muls.iter().any(|&(x, y, _)| x == a && y == b) {
                                    state.muls.push((a, b, result));
                                    // Always pause for a second when finding multiplication
                                    paused_until = Some(now + Duration::from_millis(200));
                                }
                            }
                        }
                    }
                }
                last_update = now;
            }
        }
    }

    // Cleanup
    execute!(
        terminal.backend_mut(),
        crossterm::event::DisableMouseCapture
    )
    .unwrap();
    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();

    0.into()
}
