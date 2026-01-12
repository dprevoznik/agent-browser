use std::env;

pub struct Flags {
    pub json: bool,
    pub full: bool,
    pub headed: bool,
    pub debug: bool,
    pub session: String,
    pub cdp_url: Option<String>,
}

pub fn parse_flags(args: &[String]) -> Flags {
    let mut flags = Flags {
        json: false,
        full: false,
        headed: false,
        debug: false,
        session: env::var("AGENT_BROWSER_SESSION").unwrap_or_else(|_| "default".to_string()),
        cdp_url: None,
    };

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => flags.json = true,
            "--full" | "-f" => flags.full = true,
            "--headed" => flags.headed = true,
            "--debug" => flags.debug = true,
            "--session" => {
                if let Some(s) = args.get(i + 1) {
                    flags.session = s.clone();
                    i += 1;
                }
            }
            "--cdp-url" => {
                if let Some(url) = args.get(i + 1) {
                    flags.cdp_url = Some(url.clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    flags
}

pub fn clean_args(args: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    let mut skip_next = false;

    for arg in args.iter() {
        if skip_next {
            skip_next = false;
            continue;
        }
        if arg == "--session" || arg == "--cdp-url" {
            skip_next = true;
            continue;
        }
        if !arg.starts_with("--") && arg != "-f" {
            result.push(arg.clone());
        }
    }
    result
}
