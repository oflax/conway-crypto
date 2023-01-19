#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

use colored::Colorize;

use std::io;
use std::io::Write;
use std::process;

fn data(vec: Vec<Vec<i32>>, px: usize, py: usize, _gen: i32) -> (String, u8, String) {
    let mut tup: (String, u8, String) = ("".to_owned(), 0, "".to_owned());
    for (i, el) in vec.iter().enumerate() {
        for (j, _val) in el.iter().enumerate() {
            if i == py && j == px {
                tup = calculate(i, j);
            }
        }
    }
    tup
}

static CHARS: &str = r"0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz";

fn calculate(i: usize, j: usize) -> (String, u8, String) {
    let i = i + 3;
    let j_find = "0123456789ABCDEF";
    let mut ltr = i.to_string();
    ltr.push_str(&j_find.chars().nth(j).unwrap().to_string());

    let decimal = hex::decode(ltr.clone()).unwrap()[0];
    let value = CHARS
        .chars()
        .nth((decimal - 48).into())
        .unwrap()
        .to_string();
    (ltr, decimal, value)
}

fn table_to_decimals(vec: Vec<Vec<i32>>) -> Vec<i32> {
    let mut decs: Vec<i32> = vec![];
    for (i, el) in vec.iter().enumerate() {
        for (j, val) in el.iter().enumerate() {
            if val == &1 {
                let tuple = calculate(i, j);
                decs.push(tuple.1 as i32);
            }
        }
    }
    decs
}

fn encode(data: String, vec: Vec<Vec<i32>>, gen: i32) {
    let decimal_data = hex::decode(hex::encode(data.clone())).unwrap();
    let key = table_to_decimals(vec);
    let mut new_decs: Vec<i32> = vec![];
    let mut new_dec: Vec<i32> = vec![];
    let mut password: String = "".to_owned();
    if key.len() > decimal_data.len() {
        for i in 0..key.len() {
            let j = i % decimal_data.len();
            new_decs.push(key[i] * (gen + 1) * (j as i32+1) + decimal_data[j] as i32)
        }
    } else {
        for i in 0..decimal_data.len() {
            let j = i % key.len();
            new_decs.push(key[j] * (gen + 1) * (i as i32+1) + decimal_data[i] as i32)
        }
    }
    for i in new_decs.clone() {
        let mut nth = i;
        while nth > 74 {
            nth -= 75;
        }
        new_dec.push(nth);
        let new_char = CHARS.chars().nth(nth as usize).unwrap().to_string();
        password.push_str(&new_char)
    }
    println!(
        "referans: {:?}\n\rgirilen değer: {:?}\n\ranahtar: {:?}\n\rnesil: {}\n\rgirilen metin: {}\n\rşifre: {}",
        key, decimal_data, new_decs, gen, data, password
    );
}

fn decode(key: String, gen: i32, password: String) {
    let chars = password.split("").collect::<Vec<&str>>();
    let ks = key.split("-").collect::<Vec<&str>>();
    let chars = &chars[1..chars.len() - 1];

    let mut decs = vec![];
    let mut decoded = "".to_owned();

    for i in chars {
        let dec = CHARS.find(i).unwrap();
        decs.push(dec);
    }
    
    let mut keys: Vec<i32> = vec![];
    
    for i in ks {
        let k = i.parse::<i32>().unwrap();
        keys.push(k);
    }

    if decs.len() >= keys.len() {
        for i in 0..decs.len() {
            let nth = 0;
            let ascii = CHARS.chars().nth(nth).unwrap().to_string();
            decoded.push_str(&ascii);
        }
    } else {
        println!("a");
    }

    println!("şifresiz metin: {}", decoded)
}

/*
let mut gr_size = String::new().to_string();
execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

io::stdin().read_line(&mut gr_size);

gr_size.pop();

let coor = gr_size.split("x").collect::<Vec<&str>>();

let x = coor[0].parse().unwrap();
let y = coor[1].parse().unwrap();
*/

fn opt1() {
    println!("şifrelenecek metin: ");
    let mut str = "".to_string();
    io::stdin().read_line(&mut str).ok();

    let mut stdout = io::stdout();
    str.pop();

    let x = 16;
    let y = 5;

    let lim1 = 5;

    let mut vec: Vec<Vec<i32>> = Vec::new();
    let mut xdata: Vec<i32> = Vec::new();

    for _i in 0..y - 1 {
        for _j in 0..x {
            xdata.push(0);
        }
        vec.push(xdata.clone());
        xdata.clear();
    }

    for _j in 0..x - lim1 {
        xdata.push(0);
    }
    for _j in 0..lim1 {
        xdata.push(-1);
    }
    vec.push(xdata.clone());

    let mut posx = 0;
    let mut posy = 0;

    let mut gen = 0;

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    display(vec.clone(), posx, posy);
    println!(
        "\nDeğer: (0, 0, 0)\nNesil: {}\na: canlandır\nr: temizle\nspace: yeni nesil\nq: çık",
        gen
    );

    loop {
        enable_raw_mode().unwrap();
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
        execute!(stdout, cursor::Hide).unwrap();

        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => {
                let max = if posy != vec.len() - 1 {
                    xdata.len() - 1
                } else {
                    xdata.len() - lim1 - 1
                };
                if posx < max {
                    posx += 1
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                if posx > 0 {
                    posx -= 1
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                if posy > 0 {
                    posy -= 1
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                let max = if posx < xdata.len() - lim1 {
                    vec.len() - 1
                } else {
                    vec.len() - 2
                };
                if posy < max {
                    posy += 1
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('a'),
                ..
            }) => {
                if vec[posy][posx] == 0 {
                    vec[posy][posx] = 1
                } else {
                    vec[posy][posx] = 0
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(' '),
                ..
            }) => {
                execute!(stdout, Clear(ClearType::All), cursor::Hide).unwrap();
                disable_raw_mode().unwrap();
                vec = simulate(vec);
                gen += 1
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('r'),
                ..
            }) => {
                gen = 0;
                vec.clear();
                xdata.clear();
                for _i in 0..y - 1 {
                    for _j in 0..x {
                        xdata.push(0);
                    }
                    vec.push(xdata.clone());
                    xdata.clear();
                }

                for _j in 0..x - lim1 {
                    xdata.push(0);
                }
                for _j in 0..lim1 {
                    xdata.push(-1);
                }
                vec.push(xdata.clone());
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => {
                execute!(stdout, Clear(ClearType::All), cursor::Show).unwrap();
                encode(str, vec, gen);
                process::exit(1)
            }
            _ => (),
        }
        disable_raw_mode().unwrap();
        display(vec.clone(), posx, posy);
        let (hexa, dec, ascii) = data(vec.clone(), posx, posy, gen);
        println!("\nDeğer: ({}, {}, {}) (Hexadecimal, Decimal, ASCII)\nNesil: {}\na: canlandır\nr: temizle\nspace: yeni nesil\nq: çık", hexa, dec, ascii, gen);
    }
}

fn opt2() {
    println!("anahtar(her iki sayının arasına kısa çizgi (-)): ");
    let mut key = "".to_string();
    io::stdin().read_line(&mut key).ok();
    println!("nesil: ");
    let mut gen = "".to_string();
    io::stdin().read_line(&mut gen).ok();
    println!("şifre: ");
    let mut pass = "".to_string();
    io::stdin().read_line(&mut pass).ok();
    key.pop();
    pass.pop();
    decode(key.clone(), 0, pass);
}

fn main() {
    println!("1: encode");
    println!("2: decode");
    let mut opt = "".to_string();
    io::stdin().read_line(&mut opt).ok();

    opt.pop();

    if opt == "1" {
        opt1();
    } else if opt == "2" {
        opt2();
    }
}

fn simulate(vec: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut next = vec.clone();
    for (i, row) in vec.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            let (mut u, mut ul, mut ur, mut l, mut r, mut b, mut bl, mut br) =
                (0, 0, 0, 0, 0, 0, 0, 0);
            if i > 0 {
                u = vec[i - 1][j];
                if j > 0 {
                    ul = vec[i - 1][j - 1]
                }
                if j < row.len() - 1 {
                    ur = vec[i - 1][j + 1]
                }
            }
            if i < vec.len() - 1 {
                b = vec[i + 1][j];
                if j > 0 {
                    bl = vec[i + 1][j - 1]
                }
                if j < row.len() - 1 {
                    br = vec[i + 1][j + 1]
                }
            }
            if j > 0 {
                l = vec[i][j - 1]
            }
            if j < row.len() - 1 {
                r = vec[i][j + 1]
            }
            let neighbours = u + ul + ur + l + r + b + bl + br;

            let node: i32 = *node;
            if (node == 1 && (neighbours == 2 || neighbours == 3)) || node == 0 && neighbours == 3 {
                next[i][j] = 1;
            } else if node != -1 {
                next[i][j] = 0;
            }
        }
    }
    next
}

fn display(vec: Vec<Vec<i32>>, px: usize, py: usize) {
    println!("  0 1 2 3 4 5 6 7 8 9 A B C D E F");

    for (y, row) in vec.iter().enumerate() {
        print!("{} ", y + 3);
        for (x, item) in row.iter().enumerate() {
            let sym = if x == px && y == py {
                item.to_string().yellow()
            } else if *item == 1 {
                item.to_string().blue()
            } else if *item == -1 {
                "0".red()
            } else {
                item.to_string().white()
            };
            print!("{} ", sym);
        }
        println!();
    }
}
