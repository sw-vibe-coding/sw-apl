//! `)HELP`: the pages in `data/help.txt`, for the mode in hand.

use apl_eval::Mode;

use crate::dialect::dialect;

/// The pages, as the repository keeps them. See the file's head for
/// its layout.
const PAGES: &str = include_str!("../../../../../data/help.txt");

/// The groups the overview lists, in the manual's order, then
/// sw-apl's own.
const GROUPS: [(&str, &str); 5] = [
    ("WORKSPACE", "WORKSPACE"),
    ("LIBRARY", "LIBRARY"),
    ("INQUIRY", "INQUIRY"),
    ("TERMINAL", "TERMINAL"),
    ("SW-APL", "SW-APL'S OWN"),
];

/// One page: its name, the modes it is for, its group, and its lines.
struct Page<'a> {
    name: &'a str,
    modes: &'a str,
    group: &'a str,
    lines: Vec<&'a str>,
}

/// Every page in the file.
fn pages() -> Vec<Page<'static>> {
    let mut all: Vec<Page> = Vec::new();
    for line in PAGES.lines().filter(|l| !l.starts_with('#')) {
        if let Some(head) = line.strip_prefix("== ") {
            let mut words = head.split_whitespace();
            let mut next = || words.next().unwrap_or("");
            let (name, modes, group) = (next(), next(), next());
            all.push(Page {
                name,
                modes,
                group,
                lines: Vec::new(),
            });
        } else if let Some(page) = all.last_mut() {
            page.lines.push(line);
        }
    }
    for page in &mut all {
        while page.lines.last().is_some_and(|l| l.is_empty()) {
            page.lines.pop();
        }
    }
    all
}

/// `)HELP` with no name: the commands `mode` has, by group, then how
/// to ask more. `)HELP NAME`: that page; a command the mode has not
/// got, or a name with no page, says so.
#[must_use]
pub fn help(mode: Mode, name: Option<&str>) -> Vec<String> {
    let letter = if mode == Mode::A { 'A' } else { 'B' };
    let all = pages();
    let here: Vec<&Page> = all.iter().filter(|p| p.modes.contains(letter)).collect();
    let Some(name) = name else {
        return overview(&here, mode);
    };
    let name = name.trim_start_matches(')').to_ascii_uppercase();
    if let Some(page) = here.iter().find(|p| p.name == name) {
        return page.lines.iter().map(|l| (*l).to_string()).collect();
    }
    if all.iter().any(|p| p.name == name && p.group != "TOPIC") {
        return vec![format!("{} HAS NO ){name}.", dialect(mode))];
    }
    vec![
        format!("NO HELP FOR {name}."),
        ")HELP TOPICS LISTS THE TOPICS.".to_string(),
    ]
}

/// The overview: a line or two per group, each command with its
/// parenthesis, wrapped at 64 columns under the group's name.
fn overview(here: &[&Page], mode: Mode) -> Vec<String> {
    let mut lines = vec![format!(
        "SYSTEM COMMANDS IN {}. )HELP NAME FOR ONE.",
        dialect(mode)
    )];
    for (group, label) in GROUPS {
        let names = here
            .iter()
            .filter(|p| p.group == group)
            .map(|p| format!("){}", p.name));
        let mut line = format!("{label:<13}");
        for name in names {
            if line.chars().count() + name.len() + 1 > 64 {
                let full = std::mem::replace(&mut line, " ".repeat(13));
                lines.push(full.trim_end().to_string());
            }
            line = format!("{line}{name} ");
        }
        lines.push(line.trim_end().to_string());
    }
    lines.push(")HELP TOPICS FOR MORE: THE MODES, AND SW-APL'S OWN.".to_string());
    lines
}
