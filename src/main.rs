mod state;

use std::env;
use std::error;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use crossterm::{
    cursor::{EnableBlinking, MoveTo, RestorePosition, SavePosition},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    queue,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use state::EditorState;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut editor_state = EditorState::new();

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Help here");
            return Ok(());
        }
        2 => {
            let path = Path::new(&args[1]);
            if path.exists() {
                let contents = fs::read_to_string(path)?;
                editor_state.add_file_contents(contents);
            } else {
                File::create(path)?;
            }
        }
        _ => panic!("invalid number of arguments!"),
    }

    // queue commands to enter raw mode, alternate screen, and cursor at the top
    let mut stdout = io::stdout();
    let (term_w, term_h) = terminal::size()?;
    editor_state.update_dimensions(term_w, term_h);

    queue!(
        stdout,
        SavePosition,
        EnterAlternateScreen,
        EnableBlinking,
        Clear(ClearType::All),
        MoveTo(0, 0),
        MoveTo(5, 0)
    )?;
    terminal::enable_raw_mode()?;

    let rows = editor_state.get_rows();
    for (i, line) in rows.iter().enumerate() {
        write!(stdout, "{:>4} {}\n\r", i + 1, line)?;
    }

    stdout.flush()?;

    // this loop is responsible for handling all key events that cause
    // effects on the text editor.
    loop {
        match event::read()? {
            Event::Key(ev) => {
                match ev.code {
                    KeyCode::Char('q') => {
                        if has_ctrl_modifier(&ev) {
                            break;
                        }

                        editor_state.insert_at_cursor('q');
                    }
                    KeyCode::Char('s') => {
                        if has_ctrl_modifier(&ev) {
                            // save the file
                            let rows = editor_state.get_rows();
                            for r in rows.iter() {
                                fs::write(&args[1], r)?;
                            }
                            continue;
                        }
                        editor_state.insert_at_cursor('s');
                    }
                    KeyCode::Char(kc) => editor_state.insert_at_cursor(kc),
                    KeyCode::Up => {
                        editor_state.move_up();
                    }
                    KeyCode::Down => {
                        editor_state.move_down();
                    }
                    KeyCode::Right => {
                        editor_state.move_right();
                    }
                    KeyCode::Left => {
                        editor_state.move_left();
                    }
                    KeyCode::Backspace => {
                        editor_state.remove_at_cursor();
                    }
                    KeyCode::Enter => {
                        editor_state.move_to_next_line();
                    }
                    _ => (),
                }
            }
            Event::Resize(new_term_w, new_term_h) => {
                editor_state.update_dimensions(new_term_w, new_term_h)
            }
            _ => continue,
        }

        queue!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;

        for (i, line) in editor_state.get_rows().iter().enumerate() {
            write!(stdout, "{:>4} {}\n\r", i + 1, line)?;
        }

        queue!(
            stdout,
            MoveTo(editor_state.get_x() + 5, editor_state.get_y())
        )?;

        stdout.flush()?;
    }

    // exit the program, cleanup
    terminal::disable_raw_mode()?;
    queue!(
        stdout,
        Clear(ClearType::All),
        LeaveAlternateScreen,
        RestorePosition
    )?;
    stdout.flush()?;

    Ok(())
}

fn has_ctrl_modifier(ev: &KeyEvent) -> bool {
    if ev.modifiers == KeyModifiers::CONTROL {
        true
    } else {
        false
    }
}
