use clap::{arg, Command};
use std::io::{self};
use termion;
use termion::color;

use crate::config::*;
use crate::usecase::wpm;

pub struct CliArgs {
    pub timeout: i32,
    pub level: usize,
    pub freq: f32,
    pub sound: bool,
}

pub struct UiHandler;

impl UiHandler {
    // ヘルプと引数処理
    pub fn parse_args() -> CliArgs {
        let matches = Command::new("rtyping")
            .author("Tomokatsu Kumata")
            .about("R-Typing: A terminal-based typing app.")
            .arg(
                arg!(-t --timeout <TIMEOUT> "Seconds")
                    .default_value("60")
                    .value_parser(clap::value_parser!(i32)),
            )
            .arg(
                arg!(-l --level <LEVEL> "Number of words")
                    .default_value("30")
                    .value_parser(clap::value_parser!(usize)),
            )
            .arg(
                arg!(--freq <FREQUENCY> "Frequency e.g, 880.0 or 480.0")
                    .default_value("800.0")
                    .value_parser(clap::value_parser!(f32)),
            )
            .arg(arg!(-s --sound "Enable BGM"))
            .get_matches();

        CliArgs {
            timeout: *matches.get_one::<i32>("timeout").expect("expect number"),
            level: *matches.get_one::<usize>("level").expect("expect number"),
            freq: *matches.get_one::<f32>("freq").expect("expect frequency"),
            sound: matches.get_flag("sound"),
        }
    }

    // イントロ表示
    pub fn print_intro() {
        let title_s1 = format!(r"                       Let' begin typing!");
        let title_s2 = format!(r"                         Go for high WPM.");
        let title_r1 = format!(r" ____     _____            _    Credit 01");
        let title_r2 = format!(r"|  _ \   |_   _|   _ _ __ (_)_ __   __ _ ");
        let title_r3 = format!(r"| |_) | _  | || | | | '_ \| | '_ \ / _` |");
        let title_r4 = format!(r"|  _ < (_) | || |_| | |_) | | | | | (_| |");
        let title_r5 = format!(r"|_| \_\    |_| \__, | .__/|_|_| |_|\__, |");
        let title_r6 = format!(r"               |___/|_|            |___/ ");
        let title_r7 = format!(r"                               © 2025 kmt");

        print!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, Y_TITLE));
        println!(
            "{}{}{}",
            color::Fg(color::LightWhite),
            title_s1,
            color::Fg(color::Reset)
        );
        println!(
            "{}{}{}",
            color::Fg(color::LightWhite),
            title_s2,
            color::Fg(color::Reset)
        );
        println!(
            "{}{}{}",
            color::Fg(color::Blue),
            title_r1,
            color::Fg(color::Reset)
        );
        println!(
            "{}{}{}",
            color::Fg(color::LightBlue),
            title_r2,
            color::Fg(color::Reset)
        );
        println!(
            "{}{}{}",
            color::Fg(color::Cyan),
            title_r3,
            color::Fg(color::Reset)
        );
        println!(
            "{}{}{}",
            color::Fg(color::LightCyan),
            title_r4,
            color::Fg(color::Reset)
        );
        println!(
            "{}{}{}",
            color::Fg(color::LightGreen),
            title_r5,
            color::Fg(color::Reset)
        );
        println!(
            "{}{}{}",
            color::Fg(color::Green),
            title_r6,
            color::Fg(color::Reset)
        );
        println!(
            "{}{}{}",
            color::Fg(color::LightYellow),
            title_r7,
            color::Fg(color::Reset)
        );
        println!(
            "{}Press *ENTER* key to start.🚀{}",
            termion::style::Blink,
            termion::style::Reset
        );

        // ENTER 入力待ち
        let mut start: String = String::new();
        io::stdin()
            .read_line(&mut start)
            .expect("Failed to read line.");
    }

    // 状態表示
    pub fn print_stat(types: usize, misses: i32) {
        print!("{}", termion::cursor::Save);
        print!("{}", termion::cursor::Goto(X_STAT, Y_STAT));
        print!(
            "/ Types: {blue}{types:<03}{reset} chars / Misses: {red}{misses:<03}{reset} chars",
            blue = color::Fg(color::LightBlue),
            red = color::Fg(color::Red),
            reset = color::Fg(color::Reset)
        );
        print!("{}", termion::cursor::Restore);
    }

    // 結果表示
    pub fn print_result(timer: i32, types: usize, miesses: i32) {
        let wpm = wpm::calc_wpm(types, timer, miesses);
        let result_text = format!(
            "
,-----------------------------.\r
| 🏁 Result                   |\r
|-----------------------------|\r
| Total Time      : {timer:<03} sec   |\r
| Total Typing    : {types:<03} chars |\r
| Total Misses    : {red}{miesses:<03}{reset} chars |\r
| Words Per Minute: {green}{wpm:<05.1}{reset} wpm |\r
`-----------------------------'\r
",
            red = color::Fg(color::Red),
            green = color::Fg(color::Green),
            reset = color::Fg(color::Reset)
        );

        print!("{}", termion::cursor::Goto(1, Y_QUIT));
        print!("{}", termion::clear::AfterCursor);
        println!("{}", result_text);
        print!("{}", termion::cursor::BlinkingBlock);
    }
}
