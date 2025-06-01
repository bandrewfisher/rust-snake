use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use rand::Rng;
use std::{io::stdout, time::Duration};

const FRAME_RATE_MS: u64 = 100;
const GRID_HEIGHT: u16 = 15;
const GRID_WIDTH: u16 = 40;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i16,
    y: i16,
}

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum GameState {
    Running,
    Over,
}

struct Game {
    state: GameState,
    snake: Vec<Position>,
    direction: Direction,
    food: Position,
    score: u16
}

impl Game {
    fn new() -> Self {
        Game {
            state: GameState::Running,
            snake: vec![Position { x: 0, y: 0 }, Position { x: 1, y: 0 }],
            direction: Direction::Right,
            food: Position { x: 5, y: 5 },
            score: 0
        }
    }

    fn get_state(&self) -> GameState {
        self.state
    }

    fn update(&mut self) {
        let head = self.snake.last().unwrap();
        let new_head = match self.direction {
            Direction::Up => Position {
                x: head.x,
                y: head.y - 1,
            },
            Direction::Down => Position {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Left => Position {
                x: head.x - 1,
                y: head.y,
            },
            Direction::Right => Position {
                x: head.x + 1,
                y: head.y,
            },
        };

        self.snake.push(new_head);
        // Grow length if eating food
        if new_head == self.food {
            self.score += 1;
            self.move_food();
        }
        // Check for self collision
        else if self.self_collision() || self.out_of_bounds() {
            self.state = GameState::Over;
        }
        // Normal movement
        else {
            self.snake.remove(0);
        }
    }
    
    fn self_collision(&self) -> bool {
        self.snake[..self.snake.len() - 1].contains(self.snake.last().unwrap())
    }

    fn out_of_bounds(&mut self) -> bool {
        let head = self.snake.last().unwrap();

        if head.x < 0 || head.x >= (GRID_WIDTH as i16) || head.y < 0 || head.y >= (GRID_HEIGHT as i16) {
            return true
        }
        false
    }

    fn move_food(&mut self) {
        // Generate a piece of food that's not in the same position as the snake
        loop {
            let mut rng = rand::rng();
            let food_pos = Position {
                x: rng.random_range(0..GRID_WIDTH) as i16,
                y: rng.random_range(0..GRID_HEIGHT) as i16,
            };

            if self.snake.contains(&food_pos) {
                continue;
            }

            self.food = food_pos;
            break;
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        match new_direction {
            Direction::Up => {
                if self.direction != Direction::Down {
                    self.direction = Direction::Up;
                }
            }
            Direction::Down => {
                if self.direction != Direction::Up {
                    self.direction = Direction::Down;
                }
            }
            Direction::Left => {
                if self.direction != Direction::Right {
                    self.direction = Direction::Left;
                }
            }
            Direction::Right => {
                if self.direction != Direction::Left {
                    self.direction = Direction::Right;
                }
            }
        }
    }
}

fn main() {
    // Set up terminal
    let mut stdout = stdout();
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide)
        .expect("Failed to clear terminal");

    // Game loop
    let mut game = Game::new();
    loop {
        // Handle input
        if event::poll(Duration::from_millis(FRAME_RATE_MS)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Up => {
                        game.change_direction(Direction::Up);
                    }
                    KeyCode::Down => {
                        game.change_direction(Direction::Down);
                    }
                    KeyCode::Left => {
                        game.change_direction(Direction::Left);
                    }
                    KeyCode::Right => {
                        game.change_direction(Direction::Right);
                    }
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => {
                        if let GameState::Over = game.get_state() {
                            game = Game::new();
                        }
                    }
                    _ => {}
                }
            }
        }

        // Render grid
        for y in 0..GRID_HEIGHT {
            execute!(stdout, cursor::MoveTo(0, y)).unwrap();
            for x in 0..GRID_WIDTH {
                let pos = Position {
                    x: x as i16,
                    y: y as i16,
                };
                if game.snake.contains(&pos) {
                    print!("#");
                } else if game.food == pos {
                    print!("@");
                } else {
                    print!(".");
                }
            }
        }

        execute!(stdout, cursor::MoveTo(0, GRID_HEIGHT + 1)).unwrap();
        println!("Score: {}", game.score);
        match game.get_state() {
            GameState::Running => {
                game.update();
            }
            GameState::Over => {
                execute!(stdout, cursor::MoveTo(0, GRID_HEIGHT + 2)).unwrap();
                println!("Game over! Press r to restart");
            }
        }
        execute!(stdout, cursor::MoveTo(0, GRID_HEIGHT + 3)).unwrap();
        println!("Press q to quit");
    }

    execute!(stdout, cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    println!();
}
