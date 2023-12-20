use std::{io::stdout, process::ExitCode};

use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor, Stylize}};

// struct Tsize {
    // col: u16,
    // row: u16,
// }

// impl Tsize {
    // fn from_tupl(v: (u16, u16)) -> Tsize {
        // Tsize { col: v.0, row: v.1 }
    // }
// }

fn main() -> ExitCode {
    let mut args = std::env::args();
    if args.len() < 2 {
        eprintln!("expected at least 1 argument");
        return 1.into();
    } args.next();

    let source = match args.next() {
        Some(v) => v,
        None => {eprintln!("Error getting arguments"); return 1.into();}
    };

    // let size = Tsize::from_tupl(crossterm::terminal::size().unwrap());

    let file = poppler::PopplerDocument::new_from_file(source, "").unwrap();
    let str = file.get_page(0).unwrap();
    let str = str.get_text().unwrap();
    
    match execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        // SetBackgroundColor(Color::Red),
        Print(str.bold()),
        ResetColor
    ) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error while printing: {e}"); return 1.into();
        }
    };
    0.into()
}
