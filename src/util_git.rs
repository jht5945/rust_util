use std::process::Command;

use crate::{util_cmd, util_msg, XResult};

const LANG: &str = "LANG";
const EN_US: &str = "en_US";

#[derive(Default, Debug, Clone)]
pub struct GitStatusChange {
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub renamed: Vec<(String, String)>,
    pub deleted: Vec<String>,
    pub untracked: Vec<String>,
}

impl GitStatusChange {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.modified.is_empty()
            && self.deleted.is_empty() && self.untracked.is_empty()
    }
}

pub fn git_status_change(working_dir: Option<&str>) -> XResult<GitStatusChange> {
    let git_status = git_status(working_dir)?;
    parse_git_status_change(&git_status)
}

pub fn git_rev_parse_head(working_dir: Option<&str>) -> XResult<String> {
    let mut cmd = new_git_command(working_dir);
    cmd.args(vec!["rev-parse", "HEAD"]);
    util_msg::print_info(&format!("Exec: {:?}", cmd));
    let output = cmd.output()?;
    let rev_parse_head = String::from_utf8(output.stdout)?;
    Ok(rev_parse_head.trim().to_string())
}

pub fn git_fetch_dry_run(working_dir: Option<&str>) -> XResult<bool> {
    let mut cmd = new_git_command(working_dir);
    cmd.args(vec!["fetch", "--dry-run"]);
    util_msg::print_info(&format!("Exec: {:?}", cmd));
    let output = cmd.output()?;
    let fetch_dry_run = String::from_utf8(output.stdout)?;
    Ok(fetch_dry_run.trim().is_empty())
}

pub fn git_status(working_dir: Option<&str>) -> XResult<String> {
    let mut cmd = new_git_command(working_dir);
    cmd.arg("status");
    util_msg::print_info(&format!("Exec: {:?}", cmd));
    let output = cmd.output()?;
    let git_status = String::from_utf8(output.stdout)?;
    Ok(git_status)
}

pub fn git_branch(working_dir: Option<&str>) -> XResult<Option<String>> {
    let mut cmd = new_git_command(working_dir);
    cmd.arg("branch");
    util_msg::print_info(&format!("Exec: {:?}", cmd));
    let output = cmd.output()?;
    let git_branch = String::from_utf8(output.stdout)?;
    let current_branch = git_branch.lines().find(|ln| ln.trim().starts_with('*'));
    Ok(current_branch.map(|ln| ln.trim()[1..].trim().into()))
}

pub fn git_push(working_dir: Option<&str>) {
    let mut cmd = new_git_command(working_dir);
    cmd.arg("push");
    util_msg::print_info(&format!("Exec: {:?}", cmd));
    if let Err(e) = util_cmd::run_command_and_wait(&mut cmd) {
        util_msg::print_error(&format!("Run git push failed: {}", e));
    }
}

pub fn git_add(working_dir: Option<&str>, files: &[String]) {
    let mut cmd = new_git_command(working_dir);
    cmd.arg("add");
    for f in files {
        cmd.arg(f);
    }
    util_msg::print_info(&format!("Exec: {:?}", cmd));
    if let Err(e) = util_cmd::run_command_and_wait(&mut cmd) {
        util_msg::print_error(&format!("Run git add failed: {}", e));
    }
}

pub fn git_commit(working_dir: Option<&str>, message: &str, files: &[String]) {
    let mut cmd = new_git_command(working_dir);
    cmd.arg("commit");
    cmd.arg("-m");
    cmd.arg(message);
    for f in files {
        cmd.arg(f);
    }
    util_msg::print_info(&format!("Exec: {:?}", cmd));
    if let Err(e) = util_cmd::run_command_and_wait(&mut cmd) {
        util_msg::print_error(&format!("Run git commit failed: {}", e));
    }
}

fn parse_git_status_change(git_status: &str) -> XResult<GitStatusChange> {
    let mut git_status_change: GitStatusChange = Default::default();
    for ln in git_status.lines() {
        if ln.starts_with('\t') {
            let ln = ln.trim();
            if let Some(new_file) = ln.strip_prefix("new file:") {
                let f = new_file.trim();
                git_status_change.added.push(f.to_owned());
            } else if let Some(deleted) = ln.strip_prefix("deleted:") {
                let f = deleted.trim();
                git_status_change.deleted.push(f.to_owned());
            } else if let Some(modified) = ln.strip_prefix("modified:") {
                let f = modified.trim();
                git_status_change.modified.push(f.to_owned());
            } else if let Some(renamed) = ln.strip_prefix("renamed:") {
                let f = renamed.trim();
                let mut fs = f.split("->");
                let fa = fs.next();
                let fb = fs.next();
                if let (Some(fa), Some(fb)) = (fa, fb) {
                    git_status_change.renamed.push((fa.trim().to_owned(), fb.trim().to_owned()));
                }
            } else {
                git_status_change.untracked.push(ln.to_owned());
            }
        }
    }
    Ok(git_status_change)
}

fn new_git_command(working_dir: Option<&str>) -> Command {
    let mut cmd = Command::new("git");
    cmd.env(LANG, EN_US);
    if let Some(working_dir) = working_dir {
        cmd.current_dir(working_dir);
    }
    cmd
}


#[test]
fn test_git_status() {
    let git_status = r#"On branch master
Your branch is up to date with 'origin/master'.

Changes to be committed:
  (use "git reset HEAD <file>..." to unstage)

	new file:   src/util_git.rs
	renamed:    src/template_regex.rs -> src/chk_regex.rs

Changes not staged for commit:
  (use "git add/rm <file>..." to update what will be committed)
  (use "git checkout -- <file>..." to discard changes in working directory)

	deleted:    README.md
	modified:   src/lib.rs

Untracked files:
  (use "git add <file>..." to include in what will be committed)

	Test

H"#;
    let gsc = parse_git_status_change(git_status).unwrap();
    println!("{:#?}", gsc);
    assert_eq!(1, gsc.added.len());
    assert_eq!("src/util_git.rs", gsc.added[0]);
    assert_eq!(1, gsc.modified.len());
    assert_eq!("src/lib.rs", gsc.modified[0]);
    assert_eq!(1, gsc.renamed.len());
    assert_eq!(("src/template_regex.rs".into(), "src/chk_regex.rs".into()), gsc.renamed[0]);
    assert_eq!(1, gsc.deleted.len());
    assert_eq!("README.md", gsc.deleted[0]);
    assert_eq!(1, gsc.untracked.len());
    assert_eq!("Test", gsc.untracked[0]);
}