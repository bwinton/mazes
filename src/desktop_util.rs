use clap::Arg;
use quicksilver::Result;

pub fn get_args() -> Result<String> {
    let matches = app_from_crate!("\n")
        .arg(
            Arg::with_name("algorithm")
                .short('a')
                .about("Which algorithm to run")
                .long_about("Specify an algorithm to run.")
                .takes_value(true)
                .possible_values(&[
                    "backtrack",
                    "parallel",
                    "eller",
                    "kruskal",
                    "prim",
                    "recdiv",
                    "blobby",
                    "aldousbroder",
                    "wilson",
                    // "houston",
                    "huntandkill",
                    // "tree",
                    "growingbintree",
                    "bintree",
                    "sidewinder",
                ])
                .default_value("backtrack"),
        )
        .get_matches();

    Ok(matches.value_of("algorithm").unwrap().to_owned())
}
