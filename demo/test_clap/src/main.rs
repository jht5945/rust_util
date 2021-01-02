use clap::{App, SubCommand, ArgMatches};
use rust_util::util_clap::Command;
use rust_util::util_clap::CommandError;
use rust_util::util_clap::CommandExecutor;

struct TestCommand{}
impl Command for TestCommand {
    fn name(&self) -> &str { "test" }
    
    fn subcommand<'a>(&self) -> App<'a, 'a> {
        SubCommand::with_name(self.name()).about("Test subcommand")
    }

    fn run(&self, _arg_matches: &ArgMatches, _: &ArgMatches) -> CommandError {
        println!("hello test!");
        Ok(None)
    }
}

fn main() {
    let mut c = CommandExecutor::new(None);
    c.add(Box::new(TestCommand{}));
    c.run().unwrap();
}
