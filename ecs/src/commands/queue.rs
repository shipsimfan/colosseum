use crate::Commands;

impl Commands {
    /// Queue a command to be executed later
    pub fn queue<Command: crate::Command>(&mut self, command: Command) {
        self.commands.push(Box::new(command));
    }
}
