use crate::util::Args;
use clap::{command, Arg};

pub struct Desktop {
    algorithm: String,
    variant: String,
}

impl Desktop {
    pub fn new() -> Self {
        let matches = command!("\n")
            .arg(
                Arg::new("algorithm")
                    .short('a')
                    .help("Which algorithm to run")
                    .long_help("Specify an algorithm to run.")
                    .value_parser([
                        "parallel",
                        "eller",
                        "kruskal",
                        "prim",
                        "recdiv",
                        "blobby",
                        "aldousbroder",
                        "fastaldousbroder",
                        "wilson",
                        "slowwilson",
                        "houston",
                        "huntandkill",
                        "growingtree",
                        "growingbintree",
                        "bintree",
                        "sidewinder",
                        "hexparallel",
                        "hexblobby",
                    ])
                    .default_value("parallel"),
            )
            .arg(
                Arg::new("variant")
                    .default_value_ifs([
                        ("algorithm", "parallel", Some("6")),
                        ("algorithm", "aldousbroder", Some("slow")),
                        ("algorithm", "wilson", Some("fast")),
                        ("algorithm", "growingtree", Some("middle")),
                        ("algorithm", "bintree", Some("random:NorthWest")),
                        ("algorithm", "sidewinder", Some("hard")),
                        ("algorithm", "hexparallel", Some("3")),
                    ])
                    .default_value("unused"),
            )
            .get_matches();
        let algorithm = matches.get_one::<String>("algorithm").unwrap().to_owned();
        let variant = match algorithm.as_str() {
            "bintree" => {
                let mut args = matches.get_one::<String>("variant").unwrap().splitn(2, ':');
                let random = args.next().unwrap_or("random");
                let random = if random.is_empty() || random == "random" {
                    "random"
                } else {
                    "ordered"
                };
                let bias = args.next().unwrap_or("NorthWest");
                let bias = if bias.is_empty() { "NorthWest" } else { bias };

                format!("{}:{}", random, bias)
            }
            _ => matches.get_one::<String>("variant").unwrap().to_owned(),
        };
        Self { algorithm, variant }
    }
}

impl Args for Desktop {
    fn get_algorithm(&self) -> String {
        self.algorithm.clone()
    }

    fn get_variant(&self) -> String {
        self.variant.clone()
    }

    fn needs_reset(&self) -> bool {
        false
    }
}
