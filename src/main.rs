use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use yaml_rust::YamlLoader;

fn get_services_profiles(path: PathBuf) -> (Vec<String>, Vec<String>) {
    let mut all_profiles: HashSet<String> = HashSet::new();
    let mut all_services: HashSet<String> = HashSet::new();

    let contents = fs::read_to_string(path.clone())
        .expect(format!("Something went wrong reading the file {}", path.to_str().unwrap()).as_str());

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];
    let services = doc["services"].as_hash().unwrap();

    for (k, v) in services.iter() {
        all_services.insert(k.as_str().unwrap().to_string());
        if let Some(profiles) = v["profiles"].as_vec() {
            for p in profiles.iter() {
                all_profiles.insert(p.as_str().unwrap().to_string());
            }
        }
    }

    let mut services: Vec<String> = Vec::new();
    for s in all_services.iter() {
        services.push(s.to_string());
    }


    let mut profiles: Vec<String> = Vec::new();
    for p in all_profiles.iter() {
        profiles.push(p.to_string());
    }

    (services, profiles)
}

fn select(profiles: Vec<String>) -> Vec<String> {
    let all_profiles: Vec<&str> = profiles.iter().map(|s| s as &str).collect();
    let multiselect = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select profiles (space to select, enter to confirm)")
        .items(&all_profiles[..])
        .interact();

    return match multiselect {
        Ok(selections) => {
            if selections.is_empty() {
                Vec::new()
            } else {
                let mut selected: Vec<String> = Vec::new();
                for selection in selections {
                    selected.push(String::from(all_profiles[selection]));
                }

                selected
            }
        },
        Err(_) => {
            println!("WARNING: No profiles found *");
            Vec::new()
        }
    }


}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_path = String::from(".");
    let path_prefix = &args.get(1).unwrap_or(&default_path);
    println!("Path prefix: {}", path_prefix);

    let mut all_services: Vec<String> = Vec::new();
    let mut all_profiles: Vec<String> = Vec::new();
    let (mut services, mut profiles) = get_services_profiles(Path::new(path_prefix).join("docker-compose.yml"));
    all_services.append( &mut services);
    all_profiles.append( &mut profiles);

    let (mut services, mut profiles) = get_services_profiles(Path::new(path_prefix).join("docker-compose.override.yml"));
    all_services.append( &mut services);
    all_profiles.append( &mut profiles);

    all_profiles.sort_by(|a, b| a.cmp(b));
    let selected_profiles = select(all_profiles);
    println!("{}", selected_profiles.join(","));
}