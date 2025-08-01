use {
    crate::files,
    std::{
        collections::HashSet,
        fs::File,
        io::{BufRead, BufReader},
        process::Command,
    },
};

pub fn get_amass_subdomains(
    target: &str,
    external_subdomains_dir: &str,
    quiet_flag: bool,
) -> Option<HashSet<String>> {
    if !quiet_flag {
        println!("Getting amass subdomains for {target}");
    }
    let output_filename = &format!("{external_subdomains_dir}/amass_subdomains_{target}.txt");
    let mut subdomains = HashSet::new();
    if !(File::create(output_filename).is_ok()
        && Command::new("amass")
            .args(vec![
                "enum",
                "-passive",
                "-d",
                target,
                "-o",
                output_filename,
            ])
            .output()
            .is_ok()
        && files::check_no_empty(output_filename))
    {
        eprintln!("Error getting amass subdomains for {target}\n");
    }
    match File::open(output_filename) {
        Ok(file) => {
            BufReader::new(file)
                .lines()
                .map_while(Result::ok)
                .for_each(|target| {
                    subdomains.insert(target);
                });
            Some(subdomains)
        }
        Err(e) => {
            eprintln!("Can not open file {output_filename}. Error: {e}\n");
            None
        }
    }
}

pub fn get_subfinder_subdomains(
    target: &str,
    external_subdomains_dir: &str,
    quiet_flag: bool,
) -> Option<HashSet<String>> {
    if !quiet_flag {
        println!("Getting subfinder subdomains for {target}");
    }
    let output_filename = &format!("{external_subdomains_dir}/subfinder_subdomains_{target}.txt");
    let mut subdomains = HashSet::new();
    if !(File::create(output_filename).is_ok()
        && Command::new("subfinder")
            .args(vec!["-silent", "-all", "-d", target, "-o", output_filename])
            .output()
            .is_ok()
        && files::check_no_empty(output_filename))
    {
        eprintln!("Error getting subfinder subdomains for {target}\n");
    }
    match File::open(output_filename) {
        Ok(file) => {
            BufReader::new(file)
                .lines()
                .map_while(Result::ok)
                .for_each(|target| {
                    subdomains.insert(target);
                });
            Some(subdomains)
        }
        Err(e) => {
            eprintln!("Can not open file {output_filename}. Error: {e}\n");
            None
        }
    }
}
