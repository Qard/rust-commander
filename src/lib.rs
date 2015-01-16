use std::collections::HashMap;
use std::os;

pub enum CommanderError {
  NotFound
}

pub struct Commander <'a> {
  commands: HashMap<String, Box<Fn(Vec<String>) + 'a>>
}

impl <'a> Commander <'a> {
  /// Create a named app
  pub fn new() -> Commander<'a> {
    Commander {
      commands: HashMap::new()
    }
  }

  /// Define a named command handler
  pub fn command(&mut self, name: &str, action: Box<Fn(Vec<String>) + 'a>) {
    self.commands.insert(name.to_string(), action);
  }

  /// Get a list of available commands
  pub fn commands(&self) -> Vec<String> {
    self.commands.keys().map(|v| v.to_string()).collect()
  }

  /// Run a named command handler with the supplied args
  pub fn run_command(&mut self, name: String, args: Vec<String>) -> Result<(), CommanderError> {
    match self.commands.get(name.as_slice()) {
      Some(command) => {
        command(args);
        Ok(())
      },
      None => Err(CommanderError::NotFound)
    }
  }

  /// Run command from arg set
  pub fn run_with_args(&mut self, args: Vec<String>) -> Result<(), CommanderError> {
    let mut argv = args.clone();
    let name = argv.remove(0);
    self.run_command(name, argv)
  }

  /// Determine command from os::args() and run handler
  pub fn run(&mut self) -> Result<(), CommanderError> {
    let mut args = os::args();
    args.remove(0);
    self.run_with_args(args)
  }
}
