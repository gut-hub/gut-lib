use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, cursor};
use unicode_segmentation::UnicodeSegmentation;

use std::io;
use std::io::Write;
use std::{thread, time};

/// Displays a selection list in the terminal
///
/// The function will use the provided String array and list the items in the terminal.
/// The user can then make a selection from the list using the arrow keys.
/// The index of the provided array is returned as the selection.
///
/// This function will panic if it fails to write to the terminal.
pub fn select_from_list(list: &[String], selected: Option<usize>) -> usize {
  let mut selection = selected.unwrap_or(0);

  // get selection color
  let selected = get_color();
  let color = color::Fg(selected.as_ref());

  // set terminal to raw mode to allow reading stdin one key at a time
  let mut stdout = io::stdout()
    .into_raw_mode()
    .expect("Failed to get std out raw mode");

  // use asynchronous stdin
  let mut stdin = termion::async_stdin().keys();

  // hide cursor
  write!(stdout, "\x1B[?25l",).expect("Failed to write hide cursor");

  loop {
    // write list
    write_list(&list, selection, color);

    // read user input
    let input = stdin.next();

    // logic on input
    if let Some(Ok(key)) = input {
      match key {
        // exit on Esc or 'q'
        termion::event::Key::Esc => break,
        termion::event::Key::Char('q') => break,
        termion::event::Key::Char('\n') => break,
        termion::event::Key::Up => {
          if selection > 0 {
            selection -= 1;
          }
        }
        termion::event::Key::Down => {
          if selection < (list.len() - 1) {
            selection += 1;
          }
        }
        _ => {}
      }
    }

    // clear list
    clear_list(&list);

    thread::sleep(time::Duration::from_millis(50));
  }

  // show cursor
  write!(stdout, "\x1B[?25h",).expect("Failed to write show cursor");

  selection
}

/// Displays a two column text in the terminal
///
/// The function will use the provided Strings and output them in two columns.
///
/// This function will panic if it fails to write to the terminal.
pub fn write_column(first: String, second: String, offset: Option<usize>) {
  // calculate offset
  let offset = offset.unwrap_or(20);
  let diff = offset - first.graphemes(true).count();

  let mut stdout = io::stdout()
    .into_raw_mode()
    .expect("Failed to get std out raw mode");

  write!(stdout, "\x1B[{}C{}", -1, first).expect("Failed to write first column");
  write!(stdout, "\x1B[{}C{}\n\r", diff, second).expect("Failed to write second column");
}

/// Displays the provided String array and current selection
fn write_list(list: &[String], selection: usize, color: color::Fg<&dyn termion::color::Color>) {
  let mut stdout = io::stdout()
    .into_raw_mode()
    .expect("Failed to get std out raw mode");

  // print the list out
  for (i, item) in list.iter().enumerate() {
    if i == selection {
      write!(stdout, "{}{}\n\r", color, item).expect("Failed to write list");
    } else {
      write!(stdout, "{}{}\n\r", color::Fg(color::Reset), item).expect("Failed to write list");
    }
  }

  // reset color
  write!(stdout, "{}", color::Fg(color::Reset)).expect("Failed to write reset color");
}

/// Moves the terminal cursor back up to be able re-display the list
fn clear_list(list: &[String]) {
  let mut stdout = io::stdout()
    .into_raw_mode()
    .expect("Failed to get std out raw mode");

  // move cursor up
  for _ in list {
    write!(stdout, "{}", cursor::Up(1)).expect("Failed to write cursor up");
  }
}

fn get_color() -> Box<dyn color::Color> {
  let conf = crate::config::get_gut_config();

  let selected = match conf.get("color") {
    Some(color) => color.to_string(),
    None => "Red".to_string(),
  };

  if selected.contains("Black") {
    return Box::new(color::Black);
  } else if selected.contains("Red") {
    return Box::new(color::Red);
  } else if selected.contains("Green") {
    return Box::new(color::Green);
  } else if selected.contains("Yellow") {
    return Box::new(color::Yellow);
  } else if selected.contains("Blue") {
    return Box::new(color::Blue);
  } else if selected.contains("Magenta") {
    return Box::new(color::Magenta);
  } else if selected.contains("Cyan") {
    return Box::new(color::Cyan);
  } else if selected.contains("White") {
    return Box::new(color::White);
  } else {
    return Box::new(color::Red);
  }
}
