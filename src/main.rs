use std::{io::stdout, process::ExitCode};

use crossterm::{execute, style::{Print, ResetColor, Stylize}};

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

    let file = poppler::PopplerDocument::new_from_file(source, "").unwrap();
    let n_pages = file.get_n_pages();

    for i in 0..n_pages {
        let str = file.get_page(i).unwrap();
        let str = str.get_text().unwrap();

        match execute!(
            stdout(),
            // SetForegroundColor(Color::Blue),
            // SetBackgroundColor(Color::Red),
            // Print(str.bold()),
            Print(format!("{str}\n\n")),
            ResetColor
            )
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error while printing: {e}"); return 1.into();
            }
        };
    }
    0.into()
}
