use colored::Colorize;
use regex::Regex;

pub fn highlight_line(line: &str, re: &Regex) -> String {
    let mut out: String = String::new();
    let mut last: usize = 0;

    for m in re.find_iter(line) {
        out.push_str(&line[last..m.start()]);
        out.push_str(&line[m.start()..m.end()].bright_red().to_string());
        last = m.end();
    }

    out.push_str(&line[last..]);
    out
}
