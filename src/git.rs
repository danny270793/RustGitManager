use crate::terminal;

pub const NO_GIT_REPO: &str = "NO_GIT_REPO";
pub const PENDING_ADD: &str = "PENDING_ADD";
pub const PENDING_COMMIT: &str = "PENDING_COMMIT";
pub const PENDING_PUSH: &str = "PENDING_PUSH";
//pub const UP_TO_DATE: &str          =   "UP_TO_DATE";
//pub const CREATED_NEWS: &str        =   "CREATED_NEWS";
pub const NOTHING_TO_COMMIT: &str = "NOTHING_TO_COMMIT";
pub const UNKNOW_STATE: &str = "UNKNOW_STATE";

fn parse_response(response: &str) -> &str {
    if response
        .to_lowercase()
        .contains("fatal: not a git repository")
    {
        return NO_GIT_REPO;
    }

    let mut pending_to_commit: bool = false;
    let response_lines: Vec<&str> = response.lines().collect();

    for line in response_lines {
        if line.trim().is_empty() {
            continue;
        }

        let line_no_spaces: &str = line.trim();
        let line_items: Vec<&str> = line_no_spaces.split_whitespace().collect();

        if let Some(first_item) = line_items.first() {
            match *first_item {
                "??" => return PENDING_ADD,
                "A" | "D" | "M" => pending_to_commit = true,
                _ => (),
            }
        }
    }

    if pending_to_commit {
        return PENDING_COMMIT;
    }

    UNKNOW_STATE
}

pub fn status(path: &str) -> String {
    let porcelain_command: String = format!("cd \"{}\" && git status --porcelain", path);
    let porcelain_status: String = terminal::run(porcelain_command.as_str()).unwrap();
    let parsed_porcelain_status: &str = parse_response(porcelain_status.as_str());
    if parsed_porcelain_status == UNKNOW_STATE {
        let command: String = format!("cd \"{}\" && git status", path);
        let output: String = terminal::run(command.as_str()).unwrap().to_lowercase();
        if output.contains("your branch is ahead of") {
            return PENDING_PUSH.to_string();
        } else if output.contains("your branch is up") {
            return NOTHING_TO_COMMIT.to_string();
        } else if output.contains("nothing to commit") {
            return NOTHING_TO_COMMIT.to_string();
        }
        return UNKNOW_STATE.to_string();
    }
    parsed_porcelain_status.to_string()
}
