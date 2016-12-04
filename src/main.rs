extern crate toml;
extern crate pbr;
extern crate glob;
extern crate casclib;

use std::env;
use std::fs::{File, create_dir_all};
use std::io::{Read};
use pbr::ProgressBar;
use std::path::{Path};

mod config;

fn main() {
    let mut args = env::args();

    if args.len() > 2 {
        println!("Usage: casc-extract [config_file_name]");
        return;
    }

    let filename = {
        let mut name = args.nth(1).unwrap_or("config.toml".to_string());
        if name.ends_with(".toml") {
            name
        } else {
            name.push_str(".toml");
            name
        }
    };
    let mut config_file = File::open(&filename).expect(&format!("open '{}'", filename));
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content).expect("read config file content");
    let conf = config::Config::parse(&config_content).expect("parse config");

    println!("creating output dir: {}", conf.extract.out_dir);
    create_dir_all(&conf.extract.out_dir).unwrap();
    let base_path = Path::new(&conf.extract.out_dir);    

    println!("opening storage: {}", conf.storage.path);
    let storage = casclib::open(conf.storage.path).expect("open storage");

    println!("extracting files matching globs:");
    let mut patterns: Vec<glob::Pattern> = vec![];
    for g in &conf.extract.globs {
        println!("- {}", g);
        patterns.push(glob::Pattern::new(&g).unwrap());
    }

    let count = storage.get_file_count() as u64;
    let mut pb = ProgressBar::new(count);
    let mut matched_count: u64 = 0;
    let mut skipped: Vec<(String, String)> = vec![];
    pb.format("[=>-]");
    for result in storage.files() {
        let entry = result.unwrap();
        let matches = {
            let name = entry.get_name();
            patterns.iter().any(|p| p.matches(&name))
        };

        if matches {
            let out_file_path = {
                let name = &entry.get_name();
                let path = Path::new(name);
                if let Some(ref parent) = path.parent() {
                    let path = base_path.join(parent);
                    create_dir_all(path).unwrap();
                }
                base_path.join(path)
            };
            let file = entry.open().expect("open file in archive");
            let mut out_file = File::create(out_file_path).unwrap();
            file.extract(&mut out_file).unwrap_or_else(|e| {
                skipped.push((file.get_name().to_string(), format!("{}", e)));
                0
            });
            matched_count = matched_count + 1;            
        }
        pb.inc();
    }
    pb.finish_print(&format!("done. {} files scanned, {} extracted, {} skipped.", storage.get_file_count(), matched_count, skipped.len()));
    if skipped.len() > 0 {
        println!("skipped files:");
        for (path, reason) in skipped {
            println!("- {}\n    error: {}", path, reason);
        }
    }
}
