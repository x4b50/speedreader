use std::{io::{stdout, Write}, process::ExitCode};

use crossterm::{execute, queue, cursor, terminal::{Clear, ClearType, size}, style::{Print, ResetColor, Stylize, SetForegroundColor, Color, SetBackgroundColor}};

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
    // let (cols, _) = size().unwrap();
    let mut stdout = stdout();

    for i in 0..n_pages {
        let mut curr_word = 0;
        let str = file.get_page(i).unwrap();
        let str = str.get_text().unwrap();
        // if str.lines().collect::<Vec<&str>>().len() > cols as usize {
            // let strings = str.splitn(cols, '\n');
        // }
        let words: Vec<&str> = str.split_whitespace().collect();

        while curr_word < words.len() {
            let w_ptr = words[curr_word].as_ptr();
            let str = str.as_bytes();
            let (beg, end) = unsafe { str.split_at(w_ptr.offset_from(str.as_ptr()) as usize) };
            let (cur, end) = end.split_at(words[curr_word].len());
            let beg = std::str::from_utf8(beg).unwrap();
            let cur = std::str::from_utf8(cur).unwrap();
            let end = std::str::from_utf8(end).unwrap();

            execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
            execute!(stdout, Clear(ClearType::All)).unwrap();
            match queue!(
                stdout,
                SetForegroundColor(Color::DarkGreen),
                Print(beg),
                ResetColor,
                // SetForegroundColor(Color::Black),
                // SetBackgroundColor(Color::White),
                Print(cur.bold()),
                ResetColor,
                Print(end),
                )
            {
                Ok(_) => {}
                Err(e) => { eprintln!("Error while printing: {e}"); return 1.into(); }
            };
            stdout.flush().unwrap();

            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            curr_word += 1;
        }
    }
    execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
    0.into()
}
