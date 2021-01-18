use crate::util::Args;
use clap::Arg;
use quicksilver::{log, Result};

pub struct Desktop {
    args: String,
    variant: String,
}

impl Desktop {
    pub fn new() -> Self {
        let matches = app_from_crate!("\n")
            .arg(
                Arg::with_name("algorithm")
                    .short('a')
                    .about("Which algorithm to run")
                    .long_about("Specify an algorithm to run.")
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
                    ])
                    .default_value("parallel"),
            )
            .arg(Arg::with_name("variant").default_value_ifs(&[
                ("algorithm", Some("parallel"), "6"),
                ("algorithm", Some("aldousbroder"), "slow"),
                ("algorithm", Some("wilson"), "fast"),
                ("algorithm", Some("growingtree"), "middle"),
                ("algorithm", Some("bintree"), "random:NorthWest"),
                ("algorithm", Some("sidewinder"), "hard"),
                ("algorithm", Some("hexparallel"), "3"),
                ("algorithm", None, "unused"),
            ]))
            .get_matches();
        log::info!("DT matches {:?}", matches);
        let args = matches.value_of("algorithm").unwrap().to_owned();
        let variant = match args.as_str() {
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
        Self { args, variant }
    }
}

impl Args for Desktop {
    fn get_args(&self) -> Result<String> {
        Ok(self.args.clone())
    }

    fn get_variant(&self) -> String {
        self.variant.clone()
    }
}
