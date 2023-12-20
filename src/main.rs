use std::{io::{stdout, Write}, process::ExitCode, time::{Duration, Instant}, thread};
use crossterm::{execute, queue, cursor, terminal::{Clear, ClearType, size}, style::{Print, ResetColor, Stylize, SetForegroundColor, Color, SetBackgroundColor}};

fn main() -> ExitCode {
    let mut wpm = 600;
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("expected at least 1 argument");
        return 1.into();
    }

    let source = &args[1];
    if args.len() > 2 {
        if let Ok(v) = args[2].parse::<u64>() { wpm = v; }
    }

    let frame_time = Duration::new(0, (10u64.pow(9) *60/wpm) as u32);
    let file = poppler::PopplerDocument::new_from_file(source, "").unwrap();
    let n_pages = file.get_n_pages();
    let (_, rows) = size().unwrap();
    let mut stdout = stdout();

    for i in 14..n_pages {
        let mut curr_word = 0;
        let str = file.get_page(i).unwrap();
        let str = str.get_text().unwrap();
        let mut l_index = 0;
        while l_index < str.lines().collect::<Vec<&str>>().len() {
            println!("{str}");
            // let strings: Vec<&str> = str[l_index..]
                // .splitn(rows as usize, '\n')
                // .collect();
            let strings: Vec<&str> = str.split('\n').collect::<Vec<&str>>()[l_index..].to_owned();
            let (str, _) = unsafe {
                str.split_at(
                    strings[strings.len()-1].as_ptr()
                    .offset_from(str.as_ptr()) as usize)
            };
            // println!("{:#?}", strings);
            // println!("{:#?}", str);
            // println!("{l_index}");
            // return 0.into();
            let words: Vec<&str> = str.split_whitespace().collect();

            while curr_word < words.len() {
                let frame_start = Instant::now();
                let w_ptr = words[curr_word].as_ptr();
                let str = str.as_bytes();
                let (beg, end) = unsafe { str.split_at(w_ptr.offset_from(str.as_ptr()) as usize) };
                let (cur, end) = end.split_at(words[curr_word].len());

                let beg = std::str::from_utf8(beg).unwrap();
                let cur = std::str::from_utf8(cur).unwrap();
                let end = std::str::from_utf8(end).unwrap();

                execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
                execute!(stdout, Clear(ClearType::All)).unwrap();
                if let Err(e) = queue!(
                    stdout,
                    SetForegroundColor(Color::Blue),
                    Print(beg),
                    ResetColor,
                    Print(cur.bold()),
                    ResetColor,
                    Print(end),
                    )
                { eprintln!("Error while printing: {e}"); return 1.into() };
                stdout.flush().unwrap();

                let frame_d = frame_start.elapsed();
                if frame_d < frame_time {
                    thread::sleep(frame_time - frame_d);
                }
                curr_word += 1;
            }
            l_index += rows as usize;
        }
    }
    execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
    0.into()
}
