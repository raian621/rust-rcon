use std::io::{stdout, Stdout, Write};
use crossterm::{cursor, event, style, terminal, ExecutableCommand, QueueableCommand};

use crate::rcon::client::Client;
use crate::history::History;

const PROMPT: &str = "rcon> ";

#[allow(dead_code)] // TODO: use all fields
struct TerminalState {
    // cursor position on the terminal screen
    cur_pos_x: u16,
    cur_pos_y: u16,

    // current line, will probably be used to scroll through previous lines in
    // the terminal
    curr_line: usize,

    // position in the user input; characters will be inserted or deleted at
    // this position in the user input string
    input_pos: usize,

    // lines printed to the terminal; might be uesd to scroll through previous
    // lines in the terminal
    lines: Vec<String>,

    // command history; will be used to page through using the up and down arrow keys
    history: History,

    // standard output object we use to write to the terminal
    stdout: Stdout,
}

impl TerminalState {
    fn new() -> Self {
        Self { 
            cur_pos_x: 0,
            cur_pos_y: 0,
            input_pos: 0,
            stdout: stdout(),
            curr_line: 0,
            lines: Vec::new(),
            history: History::new(),
        }
    }
}

enum ReadStatus {
    Ok,
    Exit
}

// read, evaluate, print loop for the RCON client
pub fn repl(mut client: Client) {
    terminal::enable_raw_mode().unwrap();
    let mut ts = TerminalState::new();
    ts.stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();

    let mut user_input = String::new();
    while user_input != "q" {
        match repl_read(&mut user_input, &mut ts) {
            ReadStatus::Ok => (),
            ReadStatus::Exit => break,
        };

        ts.history.push(user_input.clone());

        match user_input.as_str() {
            "~history" => print_history(&mut ts),
            _ => {
                let response = client.run(user_input.as_str()).unwrap();
                ts.stdout.queue(cursor::MoveTo(0, ts.cur_pos_y)).unwrap();
                ts.lines.push(response.clone());
                if !response.is_empty() {
                    ts.stdout.execute(style::Print(response)).unwrap();
                    ts.cur_pos_y = cursor::position().unwrap().1 + 1;
                }
            }
        }
    }

    terminal::disable_raw_mode().unwrap();
}

fn repl_read(user_input: &mut String, ts: &mut TerminalState) -> ReadStatus {
    user_input.clear();
    ts.stdout.queue(cursor::MoveTo(0, ts.cur_pos_y)).unwrap();
    ts.stdout.execute(style::Print(PROMPT)).unwrap();

    loop {
        ts.cur_pos_x = (PROMPT.len() + ts.input_pos) as u16;
        ts.stdout.execute(cursor::MoveTo(ts.cur_pos_x, ts.cur_pos_y)).unwrap();
        let mut input_modified = false;
        if let event::Event::Key(event) = event::read().unwrap() {
            match event.code {
                event::KeyCode::Enter => { 
                    ts.cur_pos_y = cursor::position().unwrap().1 + 1;
                    ts.input_pos = 0;
                    break;
                },
                event::KeyCode::Esc => return ReadStatus::Exit,
                event::KeyCode::Char(c) => {
                    user_input.insert(ts.input_pos, c);
                    ts.input_pos += 1;
                    input_modified = true;
                },
                event::KeyCode::Backspace => {
                    if ts.input_pos > 0 {
                        ts.input_pos -= 1;
                        user_input.remove(ts.input_pos);
                        input_modified = true;
                    }
                },
                event::KeyCode::Delete => {
                    if ts.input_pos < user_input.len() {
                        user_input.remove(ts.input_pos);
                        input_modified = true;
                    }
                },
                event::KeyCode::Left => {
                    if ts.input_pos > 0 {
                        ts.input_pos -= 1;
                    }
                },
                event::KeyCode::Right => {
                    if ts.input_pos < user_input.len() {
                        ts.input_pos += 1;
                    }
                },
                event::KeyCode::Up => {
                    if let Some(cmd) = ts.history.prev() {
                        user_input.clone_from(cmd.as_ref());
                        ts.input_pos = user_input.len();
                        input_modified = true;
                    }
                },
                event::KeyCode::Down => {
                    if let Some(cmd) = ts.history.next() {
                        user_input.clone_from(cmd.as_ref());
                        ts.input_pos = user_input.len();
                        input_modified = true;
                    }
                },
                _ => (),
            }
        }

        if input_modified {
            ts.stdout.queue(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
            ts.stdout.queue(cursor::MoveTo(0, ts.cur_pos_y)).unwrap();
            ts.stdout.queue(style::Print(format!("{}{}", PROMPT, user_input))).unwrap();
        }
    }

    ReadStatus::Ok
}

fn print_history(ts: &mut TerminalState) {
    for h in ts.history.iter() {
        ts.stdout
            .queue(cursor::MoveTo(0, ts.cur_pos_y)).unwrap()
            .queue(style::Print(h)).unwrap();
        ts.cur_pos_y += 1;
    }
    ts.stdout.flush().unwrap();
}
