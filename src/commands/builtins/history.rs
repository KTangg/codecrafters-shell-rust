use super::{BuiltinCommand, ShellContext};
use std::path::PathBuf;
use std::usize;

pub struct History;

// Consider moving this out to be more generic function
impl History {
    fn parse_history_args(args: &[String]) -> Result<HistoryAction, String> {
        let mut it = args.iter().peekable();

        match it.next() {
            None => Ok(HistoryAction::List { limit: None }),

            Some(arg) => match arg.as_str() {
                "-c" => {
                    if it.next().is_some() {
                        Err("history: too many arguments".into())
                    } else {
                        Ok(HistoryAction::Clear)
                    }
                }

                "-r" => {
                    let file = it.next().ok_or("history: -r requires a filename")?;
                    if it.next().is_some() {
                        Err("history: too many arguments".into())
                    } else {
                        Ok(HistoryAction::Read(PathBuf::from(file)))
                    }
                }

                "-w" => {
                    let file = it.next().ok_or("history: -w requires a filename")?;
                    if it.next().is_some() {
                        Err("history: too many arguments".into())
                    } else {
                        Ok(HistoryAction::Write(PathBuf::from(file)))
                    }
                }

                "-a" => {
                    let file = it.next().ok_or("history: -a requires a filename")?;
                    if it.next().is_some() {
                        Err("history: too many arguments".into())
                    } else {
                        Ok(HistoryAction::Append(PathBuf::from(file)))
                    }
                }

                _ if arg.chars().all(|c| c.is_ascii_digit()) => {
                    if it.next().is_some() {
                        Err("history: too many arguments".into())
                    } else {
                        Ok(HistoryAction::List {
                            limit: Some(arg.parse().unwrap()),
                        })
                    }
                }

                _ => Err(format!("history: unknown argument {}", arg)),
            },
        }
    }
}

impl BuiltinCommand for History {
    fn name(&self) -> &str {
        "history"
    }
    fn execute(&self, args: &[String], ctx: &mut ShellContext) {
        let action = match Self::parse_history_args(args) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{e}");
                return;
            }
        };

        match action {
            HistoryAction::Clear => ctx.clear_history(),

            HistoryAction::Read(file) => {
                if let Err(e) = ctx.read_history(&file) {
                    eprintln!("{}: failed to read {}", self.name(), e)
                }
            }

            HistoryAction::Write(file) => {
                if let Err(e) = ctx.write_history(&file) {
                    eprintln!("{}: failed to write {}", self.name(), e)
                }
            }

            HistoryAction::Append(file) => {
                if let Err(e) = ctx.flush_history(&file) {
                    eprintln!("{}: failed to append {}", self.name(), e)
                }
            }

            HistoryAction::List { limit } => {
                let entries = ctx.historys();
                let start = entries.len().saturating_sub(limit.unwrap_or(usize::MAX));

                for (idx, entry) in (start + 1..).zip(entries.iter().skip(start)) {
                    println!("    {}  {}", idx, entry);
                }
            }
        }
    }
}

enum HistoryAction {
    Clear,
    Read(PathBuf),
    Write(PathBuf),
    Append(PathBuf),
    List { limit: Option<usize> },
}
