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
                    .takes_value(true)
                    .possible_values(&[
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
                        "penrose",
                    ])
                    .default_value("parallel"),
            )
            .arg(Arg::new("variant").default_value_ifs(&[
                ("algorithm", Some("parallel"), Some("6")),
                ("algorithm", Some("aldousbroder"), Some("slow")),
                ("algorithm", Some("wilson"), Some("fast")),
                ("algorithm", Some("growingtree"), Some("middle")),
                ("algorithm", Some("bintree"), Some("random:NorthWest")),
                ("algorithm", Some("sidewinder"), Some("hard")),
                ("algorithm", Some("hexparallel"), Some("3")),
                ("algorithm", Some("penrose"), Some("king")),
                ("algorithm", None, Some("unused")),
            ]))
            .get_matches();
        let algorithm = matches.value_of("algorithm").unwrap().to_owned();
        let variant = match algorithm.as_str() {
            "bintree" => {
                let mut args = matches.value_of("variant").unwrap().splitn(2, ':');
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
            _ => matches.value_of("variant").unwrap().to_owned(),
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
