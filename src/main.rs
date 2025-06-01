use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use std::{io::stdout, time::Duration};

const FRAME_RATE_MS: u64 = 100;
const GRID_HEIGHT: u16 = 10;
const GRID_WIDTH: u16 = 20;

#[derive(PartialEq, Eq)]
struct Position {
    x: u16,
    y: u16,
}

fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide)
        .expect("Failed to clear terminal");

    // Game logic
    let mut snake: Vec<Position> = vec![Position { x: 0, y: 0 }, Position { x: 1, y: 0 }];

    loop {
        // Handle input
        if event::poll(Duration::from_millis(FRAME_RATE_MS)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        // Render grid
        for y in 0..GRID_HEIGHT {
            execute!(stdout, cursor::MoveTo(0, y)).unwrap();
            for x in 0..GRID_WIDTH {
                let pos = Position { x, y };
                if snake.contains(&pos) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }

        // Update/render here
        execute!(stdout, cursor::MoveTo(0, GRID_HEIGHT + 1)).unwrap();
        println!("Press q to quit");
    }

    execute!(stdout, cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
}
