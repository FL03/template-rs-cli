/*
    Appellation: cli <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{args::Cmd, interface::Cli};

mod interface;

pub mod args;

pub struct App {
    interface: Cli,
}

impl App {
    pub fn new() -> Self {
        Self {
            interface: Cli::new(),
        }
    }

    pub fn command(&self) -> Option<&Cmd> {
        self.interface.command()
    }

    pub fn update(&self) -> bool {
        self.interface.update()
    }

    pub fn verbose(&self) -> bool {
        self.interface.verbose()
    }

    pub fn handle_update(&self) {
        if self.update() {
            println!("Updating the project...");
        }
    }

    pub fn handle_verbose(&self) {
        if self.verbose() {
            tracing::debug!("Verbose mode enabled...");
        }
    }

    pub fn handle_command(&self, cmd: &Cmd) {
        match cmd {
            Cmd::Build => {
                println!("Building the project...");
            }
            Cmd::Platform(platform) => match &platform.args {
                Some(args) => match args {
                    args::PlatformOpts::Connect { target } => {
                        println!("Connecting to target: {:?}", target);
                    }
                },
                None => {
                    println!("No arguments provided for the platform command...");
                }
            },
            Cmd::System(system) => {
                if let Some(args) = &system.args {
                    match args {
                        args::SystemOpts::Config { path } => {
                            println!("Configuring the system with path: {:?}", path);
                        }
                    }
                }
            }
        }
    }

    pub fn handle(&self) {
        self.handle_update();
        self.handle_verbose();
        if let Some(cmd) = self.command() {
            self.handle_command(cmd);
        }
    }
}
