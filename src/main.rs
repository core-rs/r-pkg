use clap::{Arg, Command};
use pkg::{aur::Aur, core};

fn sync(packages: Vec<String>) {
    let aur = Aur { packages };

    core::init();

    core::download_package(&aur);

    core::install_packages(&aur);

    core::delete_from_tmp_packages(&aur);
}

fn search(packages: Vec<String>) {
    let aur = Aur { packages };

    core::init();

    core::search_package(&aur);
}

fn main() {
    let matches = Command::new("pkg")
        .about("package manager utility")
        .version("1.2.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("PKG Development Team")
        .subcommand(
            Command::new("query")
                .short_flag('Q')
                .long_flag("query")
                .about("Query the package database.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .help("search locally installed packages for matching strings")
                        .conflicts_with("info")
                        .takes_value(true)
                        .multiple_values(true),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .conflicts_with("search")
                        .help("view package information")
                        .takes_value(true)
                        .multiple_values(true),
                ),
        )
        .subcommand(
            Command::new("sync")
                .short_flag('S')
                .long_flag("sync")
                .about("Synchronize packages.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .conflicts_with("info")
                        .takes_value(true)
                        .multiple_values(true)
                        .help("search remote repositories for matching strings"),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .conflicts_with("search")
                        .short('i')
                        .help("view package information"),
                )
                .arg(
                    Arg::new("package")
                        .help("packages")
                        .required_unless_present("search")
                        .takes_value(true)
                        .multiple_values(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("sync", sync_matches)) => {
            if sync_matches.is_present("search") {
                let packages: Vec<_> = match sync_matches.values_of("search") {
                    Some(packages) => packages.collect(),
                    None => vec![],
                };
                if packages.is_empty() {
                    println!("No packages found");
                } else {
                    let values = packages.join(", ");
                    search(values.split(", ").map(|s| s.to_string()).collect());
                }
                return;
            }

            let packages: Vec<_> = match sync_matches.values_of("package") {
                Some(packages) => packages.collect(),
                None => vec![],
            };
            if packages.is_empty() {
                println!("No packages found");
            } else {
                let values = packages.join(", ");
                if sync_matches.is_present("info") {
                    search(values.split(", ").map(|s| s.to_string()).collect());
                } else {
                    sync(values.split(", ").map(|s| s.to_string()).collect());
                }
            }
        }
        Some(("query", query_matches)) => {
            if let Some(packages) = query_matches.values_of("info") {
                let comma_sep = packages.collect::<Vec<_>>().join(", ");
                println!("Retrieving info for {}...", comma_sep);
            } else if let Some(queries) = query_matches.values_of("search") {
                let comma_sep = queries.collect::<Vec<_>>().join(", ");
                println!("Searching Locally for {}...", comma_sep);
            } else {
                println!("Displaying all locally installed packages...");
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
