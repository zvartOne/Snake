use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{cursor, execute};
use rand::Rng;
use std::cmp::PartialEq;
use std::io;
use std::io::stdout;
use std::ops::Range;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

const WIDTH: i32 = 50;
const HEIGHT: i32 = 25;
const POSITION_HEAD: Point = Point {
    x: WIDTH / 2,
    y: HEIGHT / 2,
};
const POSITION_BODY: [Point; 2] = [
    Point {
        x: WIDTH / 2 - 2,
        y: HEIGHT / 2,
    },
    Point {
        x: WIDTH / 2 - 1,
        y: HEIGHT / 2,
    },
];
fn draw(buff: &Vec<Vec<char>>) {
    for i in buff {
        for j in i {
            print!("{j}");
        }
        println!("\r");
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

fn write_apple(apple: &Point, buff: &mut Vec<Vec<char>>) {
    buff[apple.y as usize][apple.x as usize] = '@';
}

fn update() {
    sleep(Duration::from_millis(150));
    Command::new("clear").status().expect("none");
}

fn clear(buff: &mut Vec<Vec<char>>) {
    for i in buff {
        for j in i {
            *j = '.';
        }
    }
}
#[derive(Copy, Clone, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

struct Snake {
    head: Point,
    body: Vec<Point>,
    health: u32,
}
fn gen_apple(range_x: Range<i32>, range_y: Range<i32>) -> Point {
    Point {
        x: rand::thread_rng().gen_range(range_x),
        y: rand::thread_rng().gen_range(range_y),
    }
}

#[derive(Debug)]
struct GameStatus {
    pub score: u32,
}

impl GameStatus {
    pub fn new() -> Self{
        Self{
            score: 0
        }
    }
    
    fn add_score(&mut self){
        self.score += 10;
    }
    
    fn get_score(&self) -> u32{
        self.score
    }
}

impl Snake {
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
    fn up_body(&mut self) {
        self.body.push(Point {
            x: self.body.last().unwrap().x,
            y: self.body.last().unwrap().y,
        });
    }
    fn eat(&self, apple: &Point) -> bool {
        self.head == *apple
    }

    fn spawn() -> Self {
        Self {
            head: POSITION_HEAD,
            body: POSITION_BODY.to_vec(),
            health: 2,
        }
    }

    fn respawn(&mut self) {
        self.head = POSITION_HEAD;
        self.body = POSITION_BODY.to_vec();
        self.health -= 1;
    }

    fn is_alive(&self) -> bool {
        self.health != 0
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
    enable_raw_mode()?;
    let mut play_area = vec![vec![' '; WIDTH as usize]; HEIGHT as usize];
    let mut current_key_code = Forward::Unknown;
    let mut term = stdout();
    execute!(term, cursor::Hide).expect("Не удалось скрыть курсор");
    let mut key_event = Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::empty()));
    let mut snake = Snake::spawn();
    let mut apple = gen_apple(0..WIDTH, 0..HEIGHT);
    let mut score = GameStatus::new();
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

        if snake.eat(&apple) {
            snake.up_body();
            apple = gen_apple(0..WIDTH, 0..HEIGHT);
            score.add_score();
        }

        if write(&snake, &mut play_area) {
            
            current_key_code = Forward::Unknown;
            if snake.is_alive(){
                snake.respawn();
            } else {
                println!("Game over!\r\n");
                execute!(term, cursor::Show).expect("Не удалось отобразить курсор");
                disable_raw_mode()?;
                return Ok(())  
            }
        }

        write_apple(&apple, &mut play_area);
        println!("Score: {:?}", score.get_score());
        draw(&play_area);
        update();
        clear(&mut play_area);
    }

}