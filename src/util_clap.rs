use std::process;
use clap::{App, Arg, ArgMatches};
use crate::XResult;
use crate::util_msg;

pub type CommandError = XResult<Option<i32>>;

pub trait Command {
    fn name(&self) -> &str;
    fn subcommand<'a>(&self) -> App<'a, 'a>;
    fn run(&self, arg_matches: &ArgMatches, sub_arg_matches: &ArgMatches) -> CommandError;
}

pub trait DefaultCommand {
    fn process_command<'a>(&self, app: App<'a, 'a>) -> App<'a, 'a>;
    fn run(&self, arg_matches: &ArgMatches) -> CommandError;
}

pub struct DefaultCommandImpl;
impl DefaultCommand for DefaultCommandImpl {
    fn process_command<'a>(&self, app: App<'a, 'a>) -> App<'a, 'a> {
        app.arg(Arg::with_name("verbose").long("verbose").short("v").multiple(true).help("Show verbose info"))
    }

    fn run(&self, arg_matches: &ArgMatches) -> CommandError {
        let verbose_count = arg_matches.occurrences_of("verbose");
        util_msg::print_info(&format!("Verbose count: {}", verbose_count));
        util_msg::print_info("This is default command cli, please run with help (--help)");
        Ok(None)
    }
}

pub struct CommandExecutor {
    default_cmd: Option<Box<dyn DefaultCommand>>,
    commands: Vec<Box<dyn Command>>,
}

impl CommandExecutor {
    pub fn new_default() -> Self {
        Self::new(None)
    }

    pub fn new(default_cmd: Option<Box<dyn DefaultCommand>>) -> Self {
        CommandExecutor{
            default_cmd,
            commands: Vec::new(),
        }
    }

    pub fn add(&mut self, cmd: Box<dyn Command>) -> &mut Self {
        self.commands.push(cmd);
        self
    }

    pub fn add_commands(&mut self, cmds: Vec<Box<dyn Command>>) -> &mut Self {
        for cmd in cmds.into_iter() {
            self.add(cmd);
        }
        self
    }

    pub fn run(&self) -> XResult<()> {
        let mut app = App::new(env!("CARGO_PKG_NAME"))
                .version(env!("CARGO_PKG_VERSION"))
                .about(env!("CARGO_PKG_DESCRIPTION"));
        if let Some(default_cmd) = &self.default_cmd {
            app = default_cmd.process_command(app);
        }
        for command in &self.commands {
            app = app.subcommand(command.subcommand());
        }
        let matches = app.get_matches();
        for command in &self.commands {
            if let Some(sub_cmd_matches) = matches.subcommand_matches(command.name()) {
                match command.run(&matches, sub_cmd_matches)? {
                    None => return Ok(()),
                    Some(code) => process::exit(code),
                }
            }
        }
        match &self.default_cmd {
            None => {
                util_msg::print_error("No default command, please try help (--help)");
                process::exit(1);
            },
            Some(default_cmd) => match default_cmd.run(&matches)? {
                None => return Ok(()),
                Some(code) => process::exit(code),
            },
        }
    }
}
