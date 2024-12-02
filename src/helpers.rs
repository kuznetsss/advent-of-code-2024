use std::{fs, path::PathBuf, str::FromStr, sync::Arc, time::Duration};

use reqwest::{blocking::Client, cookie::Jar, Url};

pub fn get_input(day: i16) -> Result<String, String> {
    let input = Input { day };
    if let Some(content) = get_from_cache(&input) {
        return Ok(content);
    }
    let content = fetch(&input)?;
    save_to_cache(&input, &content)?;
    Ok(content)
}

const CACHE_DIR: &str = ".cache/aoc2025";

struct Input {
    day: i16,
}

impl Input {
    fn cache_path(&self) -> PathBuf {
        PathBuf::from_str(CACHE_DIR)
            .unwrap()
            .join(format!("{}.txt", self.day))
    }

    fn url(&self) -> String {
        format!("https://adventofcode.com/2024/day/{}/input", self.day)
    }
}

fn get_from_cache(input: &Input) -> Option<String> {
    let cache_path = input.cache_path();
    if fs::exists(&cache_path).unwrap_or(false) {
        return Some(
            fs::read_to_string(&cache_path).expect(&format!("Couldn't read {:?}", cache_path)),
        );
    }
    None
}

fn save_to_cache(input: &Input, content: &str) -> Result<(), String> {
    let cache_dir = PathBuf::from_str(CACHE_DIR).unwrap();
    if !fs::exists(&cache_dir).unwrap_or(false) {
        fs::create_dir_all(&cache_dir)
            .map_err(|e| format!("Couldn't create {:?}: {e}", cache_dir))?;
    }
    let file_path = cache_dir.join(format!("{}.txt", input.day));
    fs::write(file_path, content).map_err(|e| format!("Error writing file: {e}"))
}

fn fetch(input: &Input) -> Result<String, String> {
    print!("Downloading input for day {} ... ", input.day);
    let cookie = fs::read_to_string(".cookie")
        .map_err(|e| format!("Error reading cookie file: {e}"))
        .unwrap();
    let url = "https://adventofcode.com".parse::<Url>().expect("Bad URL");
    let jar = Arc::new(Jar::default());
    jar.add_cookie_str(&cookie, &url);

    let client = Client::builder()
        .cookie_provider(jar)
        .timeout(Duration::from_secs(1))
        .build()
        .expect("Couldn't build client");
    let response = client
        .get(input.url())
        .send()
        .map_err(|e| format!("Request error: {e}"))?;
    println!("OK");
    response
        .text()
        .map_err(|e| format!("Error getting body from response: {e}"))
}
