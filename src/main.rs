// Draw some pixel art in the GitHub contributions graph.
// Inspired by https://github.com/hintjens/waka
// Font: http://fontstruct.com/fontstructions/show/840449/5x7_monospace_ce

extern crate chrono;

use std::process::Command;
use chrono::*;

fn poke_git(date: DateTime<Local>, count: u8) -> () {
    for n in 1..(count + 1) {
        let commit_msg = format!("Commit {} of {} for {}", n, count, date.format("%A, %b %d, %Y"));

        Command::new("git")
                .arg("commit")
                .arg("--quiet")
                .arg(format!("-m {}", commit_msg))
                .arg("--allow-empty")
                .arg(format!("--date={}", date))
                .status()
                .expect("git commit command failed to start");
    }
}


static PIXELS: [&'static str; 7] = [
    //  |-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|-----|
        "#...#........##....##.....................#...#..............##.......#...#...",
        "#...#.........#.....#.....................#...#...............#.......#...#...",
        "#...#..###....#.....#....###..............#...#..###..#.##....#....##.#...#...",
        "#####.#...#...#.....#...#...#.............#.#.#.#...#.##..#...#...#..##...#...",
        "#...#.#####...#.....#...#...#..##.........#.#.#.#...#.#.......#...#...#.......",
        "#...#.#.......#.....#...#...#...#.........#.#.#.#...#.#.......#...#...#.......",
        "#...#..###...###...###...###...#...........#.#...###..#......###...####...#...",
    ];


fn main() {
    let last_sunday = Local.ymd(2016, 8, 21).and_hms(12, 0, 0);
    let start_date = last_sunday - Duration::weeks(52);

    for (day_cnt, s) in PIXELS.iter().enumerate() {
        for (week_cnt, c) in s.chars().enumerate() {
            let date = start_date + Duration::weeks(week_cnt as i64) + Duration::days(day_cnt as i64);
            let delta : u8 = 32;
            match c {
                '.' | ' ' => poke_git(date, ((delta * 0) + 1)),
                '+'       => poke_git(date, ((delta * 1) + 1)),
                '*'       => poke_git(date, ((delta * 2) + 1)),
                '#'       => poke_git(date, ((delta * 3) + 1)),
                _         => (),
            }
        }
    }
}
