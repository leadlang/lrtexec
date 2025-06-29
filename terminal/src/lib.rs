//! Colour-aware help-message macro for every Lead Runtime tool.
//! Supports global FLAGS *and* per-command FLAGS.

#![allow(unused_comparisons)]

pub use owo_colors;

/// Call once in `main()` before any output so Windows 10+ shows colours.
#[inline]
pub fn init_colors() {
  let _ = enable_ansi_support::enable_ansi_support();
}

/// Tiny helper: count token-trees at macro-expansion time.
#[macro_export]
macro_rules! _tt_count { () => {0usize}; ($h:tt $($t:tt)*) => {1usize + $crate::_tt_count!($($t)*)}; }

/// ### `forge_help!{ … }`
/// ```rust
/// println!(
///     "{}",
///     forge_help!(
///         name    = "smelt",
///         version = env!("CARGO_PKG_VERSION"),
///         about   = "Lead Runtime compiler — orchestrates the whole forge.",
///         usage   = "smelt <COMMAND> [OPTIONS] <ENTRY.lrt>",
///
///         // 1️⃣ Global flags
///         flags {
///             "-h, --help"          => "Show this help",
///             "--color <WHEN>"      => "always, auto (default) or never",
///         },
///
///         // 2️⃣ Commands, each may have its own flags
///         commands {
///             build => {
///                 desc:  "Compile project (default)",
///                 flags: {
///                     "--release"           => "Optimise codegen",
///                     "-o, --output <FILE>" => "Write .lrtexec to FILE",
///                 }
///             },
///             check => {
///                 desc: "Type-check only; no output"
///             },
///             run => {
///                 desc:  "Compile & execute ENTRY.lrt",
///                 flags: {
///                     "-p, --profile <NAME>" => "Run under given profile",
///                 }
///             },
///         }
///     )
/// );
/// ```
#[macro_export]
macro_rules! forge_help {
    (
        /* header */
        name      = $name:expr,
        version   = $ver:expr,
        about     = $about:expr,
        usage     = $usage:expr,

        /* global flags */
        flags { $( $gflag:expr => $gdesc:expr ),* $(,)? },

        /* commands with optional local flags */
        commands {
            $(
                $cmd:expr => {
                    desc:  $cdesc:expr,
                    usage: $cusage:expr
                    $(, flags: {
                        $( $cflag:expr => $cfdesc:expr ),* $(,)?
                    } )?
                    $(, examples: [
                        $($ceg:expr),+
                    ])?
                    $(, note: $cnote:expr)?
                }
            ),* $(,)?
        } $(,)?
    ) => {
      pastey::paste! {
        fn show_help_for_cmd(c: &str) {
            match c {
                $(
                    $cmd => {
                        /* Show help for command */
                        println!("{}", [<help_cmd_ $cmd>]());
                    }
                )*
                _ => {
                    /* Show just help */
                    println!("{}", help_string());
                }
            }
        }

        $(
            fn [<help_cmd_ $cmd>]() -> String {
                use std::fmt::Write;
                use terminal::owo_colors::OwoColorize;
                let mut s = String::new();

                /* ── HEADER ────────────────────────────────────────────── */
                writeln!(s, "{} {}", $name.bold().yellow(), $ver.dimmed()).unwrap();
                writeln!(s, "{}", $about).unwrap();
                writeln!(s).unwrap();

                /* --- COMMAND -------------------------------- */
                writeln!(s, "{}{} {}", $cmd.cyan().bold(), ":".cyan(), $cdesc).unwrap();
                writeln!(s).unwrap();

                /* --- USAGE -------------------------------- */
                writeln!(s, "{}", "Usage:".bold()).unwrap();
                writeln!(s, "╭───────────────────────────────────────────────────────────╮").unwrap();
                writeln!(s, "│ {:<57} │", $cusage).unwrap();

                /* --- EXAMPLES -------------------------------- */
                $(
                    writeln!(s, "│ {:<57} │", "").unwrap();
                    writeln!(s, "│ {:<57} │", "Examples:".dimmed()).unwrap();
                    $(
                        /* Example line */
                        writeln!(s, "│   {:<55} │", $ceg.dimmed()).unwrap();
                    )*
                )?

                /* --- FLAGS -------------------------------- */
                $(
                    writeln!(s, "│ {:<57} │", "").unwrap();
                    $(
                        /* flag line */
                        writeln!(s, "│ {:<20} {:<36} │", $cflag.yellow(), $cfdesc).unwrap();
                    )*
                )?

                /* ---- NOTE----------------- */
                $(
                    writeln!(s, "│ {:<57} │", "").unwrap();
                    writeln!(s, "│ {} {:<51} │", "Note:".green(), $cnote).unwrap();
                )?

                writeln!(s, "╰───────────────────────────────────────────────────────────╯").unwrap();

                s
            }
        )*
      }

      fn help_string() -> String {
        use std::fmt::Write;
        use terminal::owo_colors::OwoColorize;
        let mut s = String::new();

        /* ── HEADER ────────────────────────────────────────────── */
        writeln!(s, "{} {}", $name.bold().yellow(), $ver.dimmed()).unwrap();
        writeln!(s, "{}", $about).unwrap();
        writeln!(s).unwrap();

        /* ── USAGE ─────────────────────────────────────────────── */
        writeln!(s, "{} {}", "Usage:".bold(), $usage).unwrap();
        writeln!(s).unwrap();

        /* ── COMMANDS ──────────────────────────────────────────── */
        if $crate::_tt_count!($($cmd)*) > 0 {
            writeln!(s, "{}", "Commands:".bold()).unwrap();
            $(
                /* command line */
                writeln!(s, "  {:<24} {}", $cmd.cyan(), $cdesc).unwrap();

                /* optional per-command flags */
                $(
                    writeln!(s, "   {}", "Options:".dimmed()).unwrap();
                    $(
                        /* flag line */
                        writeln!(s, "    {:<23} {}", $cflag.yellow(), $cfdesc.dimmed()).unwrap();
                    )*
                    writeln!(s).unwrap();
                )?
            )*

            writeln!(s).unwrap();
        }

        /* ── GLOBAL OPTIONS ────────────────────────────────────── */
        if $crate::_tt_count!($($gflag)*) > 0 {
            writeln!(s, "{}", "Options:".bold()).unwrap();
            $(
                writeln!(s, "  {:<24} {}", $gflag.green(), $gdesc).unwrap();
            )*
        }

        s
      }
    };
}
