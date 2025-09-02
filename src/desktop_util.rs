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
                        "originshift",
                        "hexparallel",
                        "hexblobby",
                        "penrose",
                    ])
                    .default_value("parallel"),
            )
            .arg(Arg::new("variant").short('v').default_value_ifs([
                ("algorithm", "parallel", Some("6")),
                ("algorithm", "aldousbroder", Some("slow")),
                ("algorithm", "wilson", Some("fast")),
                ("algorithm", "growingtree", Some("middle")),
                ("algorithm", "bintree", Some("random:NorthWest")),
                ("algorithm", "sidewinder", Some("hard")),
                ("algorithm", "originshift", Some("1")),
                ("algorithm", "hexparallel", Some("3")),
                ("algorithm", "penrose", Some("king")),
                // ("algorithm", None, Some("unused")),
            ]))
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
            _ => matches
                .get_one::<String>("variant")
                .unwrap_or(&"unused".to_string())
                .to_owned(),
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
