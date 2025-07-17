use std::env::args;
use std::process;
use terminal::{forge_help, init_colors};

use terminal::owo_colors::OwoColorize;

macro_rules! flagmanager {
  (
    flags {
      $(--$flag:ident),+
    }
    data {
      $(--$data:ident),+
    }
  ) => {
    #[derive(Debug, Default)]
    struct FlagManager<'a> {
      $(pub $flag: bool),*,
      $(pub $data: Option<&'a str>),*,
      pub args: Vec<&'a str>
    }

    impl<'a> FlagManager<'a> {
      pub fn parse(data: &'a [String]) -> Self {
        let mut man = Self::default();

        let mut data_iter = data.iter();

        while let Some(arg) = data_iter.next() {
          let data = arg.as_str();

          if data.len() >= 3 && &data[0..2] == "--" {
            match &data[2..] {
              $(stringify!($flag) => { man.$flag = true })*
              $(stringify!($data) => {
                let Some(x) = data_iter.next() else {
                  println!("{} missing value for flag '--{}'", "error:".red().bold(), stringify!($data).yellow().bold());
                  println!("\n{} run '{}' for help menu", "Tip:".blue().bold(), "smelt --help".yellow().bold());
                  std::process::exit(1);
                };

                man.$data = Some(x);
              })*
              flag => {
                println!("{} unexpected flag '--{}' found", "error:".red().bold(), flag.yellow().bold());
                println!("\n{} run '{}' for help menu", "Tip:".blue().bold(), "smelt --help".yellow().bold());
              }
            }
          } else {
            man.args.push(arg.as_str());
          }
        }

        man
      }
    }
  };
}

forge_help! {
  name = "smelt",
  version = env!("CARGO_PKG_VERSION"),
  about = "Lead Runtime Smelter - The one tool for lead runtime management",
  usage = "smelt [options]",
  flags {
    "--version" => "Prints the version",
    "--help" => "Prints this help menu"
  },
  commands {
    "new" => {
      desc: "Creates a new project",
      usage: "smelt new <directory>"
    },
    "addpkg" => {
      desc: "Adds a package to the project",
      usage: "smelt addpkg <package>"
    },
    "removepkg" => {
      desc: "Removes a package from the project",
      usage: "smelt removepkg <package>"
    },
    "list" => {
      desc: "Lists all packages in the project",
      usage: "smelt list"
    },
    "build" => {
      desc: "Build the project",
      usage: "smelt build [OPTIONS]",
      flags: {
        "--out <directory>" => "Change the output directory",
        "--release" => "Builds the binary for release mode",
        "--host" => "Compile only for this target",
      },
      examples: [
        "smelt build",
        "smelt build --host",
        "smelt build --out dist --release"
      ],
      note: "`smelt build --out dist` is default"
    },
    "run" => {
      desc: "Builds and then runs the project (same options as build)",
      usage: "smelt run [OPTIONS]",
      examples: [
        "smelt run",
        "smelt run --host",
        "smelt run --out dist --release"
      ],
      note: "`smelt run --out dist` is default"
    },
    "docs" => {
      desc: "Runs a TUI that shows library docs",
      usage: "smelt docs",
      note: "This command utilizes a full TUI with mouse support"
    }
  }
}

fn show_help() {
  let help = help_string();

  println!("{help}");
}

flagmanager! {
  flags {
    --help,
    --version,
    --release,
    --host
  }
  data {
    --out
  }
}

mod config;

mod build;
mod new;

fn main() {
  init_colors();

  let mut args = args();
  args.next();

  let args = args.collect::<Vec<_>>();

  if args.len() == 0 {
    show_help();
    process::exit(0);
  }

  let man = FlagManager::parse(&args);

  if man.help {
    if man.args.len() >= 1 {
      show_help_for_cmd(&man.args[0]);
    } else {
      show_help();
    }

    process::exit(0);
  }

  if man.args.len() == 0 || man.args[0] == "help" {
    show_help();
    process::exit(0);
  }

  match man.args[0] {
    "new" => {
      let Some(dir) = man.args.get(1) else {
        println!(
          "{} missing argument for command '{}'",
          "error:".red().bold(),
          "new".yellow().bold()
        );
        println!(
          "\n{} run '{}' for help menu",
          "Tip:".blue().bold(),
          "smelt --help".yellow().bold()
        );
        process::exit(1);
      };

      new::create(dir);
    }
    "build" => {
      build::build(man.release, man.host, man.out);
    }
    e => {
      // Invalid comment error, recommend smelt --help
      println!(
        "{} unexpected command '{}' found",
        "error:".red().bold(),
        e.yellow().bold()
      );
      println!(
        "\n{} run '{}' for help menu",
        "Tip:".blue().bold(),
        "smelt --help".yellow().bold()
      );
      process::exit(1);
    }
  }
}
