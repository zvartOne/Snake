use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{cursor, execute};
use rand::{Rng};
use std::io;
use std::io::stdout;
use std::ops::Range;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn draw(buff: &Vec<Vec<char>>) {
    for i in buff {
        for j in i {
            print!("{j}");
        }
        println!();
    }
}

fn write(snake: &Snake, buff: &mut Vec<Vec<char>>) -> bool {
    if snake.head.y >= buff.len() as i32 || snake.head.y < 0 {
        return true;
    }

    if snake.head.x >= buff[0].len() as i32 || snake.head.x < 0 {
        return true;
    }

    buff[snake.get_head().y as usize][snake.get_head().x as usize] = '*';

    for p in &snake.body {
        buff[p.y as usize][p.x as usize] = '#';
    }

    false
}

fn write_apple(apple: &Point, buff: &mut Vec<Vec<char>>){
    buff[apple.y as usize][apple.x as usize] = '@';
}

fn update() {
    sleep(Duration::from_millis(150));
    Command::new("clear").status().expect("none");
}

fn clear(buff: &mut Vec<Vec<char>>) {
    for i in buff {
        for j in i {
            *j = ' ';
        }
    }
}
#[derive(Copy, Clone)]
struct Point {
    pub x: i32,
    pub y: i32,
}

struct Snake {
    head: Point,
    body: Vec<Point>,
}
fn gen_apple(range_x: Range<i32>, range_y: Range<i32>) -> Point {
    Point {
        x: rand::thread_rng().gen_range(range_x),
        y: rand::thread_rng().gen_range(range_y),
    }
}
impl Snake {
    fn new(head: Point, body: Vec<Point>) -> Self {
        Snake { head, body }
    }

    pub fn move_snake(&mut self, forward: Forward) {
        let len = self.body.len() - 1;
        for i in 0..len {
            self.body[len - i] = self.body[len - i - 1];
        }
        self.body[0] = self.head;
        match forward {
            Forward::Left => {
                self.head.x -= 1;
            }
            Forward::Right => {
                self.head.x += 1;
            }
            Forward::Up => {
                self.head.y -= 1;
            }
            Forward::Down => {
                self.head.y += 1;
            }
            _ => {}
        }
    }

    fn get_head(&self) -> &Point {
        &self.head
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Forward {
    Left,
    Right,
    Up,
    Down,
    Unknown,
}

fn main() -> io::Result<()> {
    const WIDTH: i32 = 50;
    const HEIGHT: i32 = 25;
    let mut play_area = vec![vec![' '; WIDTH as usize]; HEIGHT as usize];
    let mut snake = Snake::new(
        Point { x: 15, y: 15 },
        vec![Point { x: 14, y: 15 }, Point { x: 13, y: 15 }],
    );
    let mut current_key_code = Forward::Unknown;
    let mut hide_cursor = stdout();
    execute!(hide_cursor, cursor::Hide).expect("Не удалось скрыть курсор");
    let mut key_event = Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::empty()));
    loop {
        if poll(Duration::ZERO)? {
            key_event = read()?;
        }

        match key_event {
            Event::Key(key) => match key.code {
                KeyCode::Left => {
                    if current_key_code != Forward::Right {
                        current_key_code = Forward::Left;
                    }
                }
                KeyCode::Right => {
                    if current_key_code != Forward::Left {
                        current_key_code = Forward::Right;
                    }
                }
                KeyCode::Up => {
                    if current_key_code != Forward::Down {
                        current_key_code = Forward::Up;
                    }
                }
                KeyCode::Down => {
                    if current_key_code != Forward::Up {
                        current_key_code = Forward::Down;
                    }
                }
                _ => {}
            },
            _ => {}
        }

        match current_key_code {
            Forward::Left => snake.move_snake(Forward::Left),
            Forward::Right => snake.move_snake(Forward::Right),
            Forward::Up => snake.move_snake(Forward::Up),
            Forward::Down => snake.move_snake(Forward::Down),
            _ => {}
        }

        if write(&snake, &mut play_area) {
            println!("Game over!");
            snake.head = Point { x: 15, y: 15 };
            current_key_code = Forward::Unknown;
        }
        write_apple(&gen_apple(0..WIDTH, 0..HEIGHT), &mut play_area);
        draw(&play_area);
        update();
        clear(&mut play_area);
    }
}
