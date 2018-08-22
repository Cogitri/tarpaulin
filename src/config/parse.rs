use std::env;
use std::path::{PathBuf};
use std::time::{Duration};

use clap::{ArgMatches};
use coveralls_api::{CiService};
use regex::{Regex};

use super::types::*;


pub(super) fn get_list(args: &ArgMatches, key: &str) -> Vec<String> {
    args.values_of_lossy(key).unwrap_or_else(Vec::new)
}


pub(super) fn get_line_cov(args: &ArgMatches) -> bool {
    let cover_lines = args.is_present("line");
    let cover_branches = args.is_present("branch");

    cover_lines || !(cover_lines || cover_branches)
}


pub(super) fn get_branch_cov(args: &ArgMatches) -> bool {
    let cover_lines = args.is_present("line");
    let cover_branches = args.is_present("branch");

    cover_branches || !(cover_lines || cover_branches)
}


pub(super) fn get_manifest(args: &ArgMatches) -> PathBuf {
    let mut manifest = env::current_dir().unwrap();

    if let Some(path) = args.value_of("root") {
        manifest.push(path);
    }

    manifest.push("Cargo.toml");
    manifest.canonicalize().unwrap_or(manifest)
}


pub(super) fn get_ci(args: &ArgMatches) -> Option<CiService> {
    value_t!(args, "ciserver", Ci).map(|x| x.0).ok()
}


pub(super) fn get_coveralls(args: &ArgMatches) -> Option<String> {
    args.value_of("coveralls").map(ToString::to_string)
}


pub(super) fn get_report_uri(args: &ArgMatches) -> Option<String> {
    args.value_of("report-uri").map(ToString::to_string)
}


pub(super) fn get_outputs(args: &ArgMatches) -> Vec<Format> {
    values_t!(args.values_of("format"), Format).unwrap_or(vec![Format::Stdout])
}


pub(super) fn get_excluded(args: &ArgMatches) -> Vec<Regex> {
    let mut files = vec![];

    for temp_str in &get_list(args, "exclude-files") {
        let s = &temp_str.replace(".", r"\.").replace("*", ".*");

        if let Ok(re) = Regex::new(s) {
            files.push(re);
        }
        else {
            eprintln!("Invalid regex: {}", temp_str);
        }
    }

    files
}


pub(super) fn get_timeout(args: &ArgMatches) -> Duration {
    if args.is_present("timeout") {
        let duration = value_t!(args.value_of("timeout"), u64).unwrap_or(60);
        Duration::from_secs(duration)
    }
    else {
        Duration::from_secs(60)
    }
}

