use crate::config;

pub fn execute(key: Option<String>, value: Option<String>) -> anyhow::Result<()> {
    match (key, value) {
        (None, None) => {
            let cfg = config::global::load()?;
            println!("editor = {}", cfg.editor);
            println!("browser = {}", cfg.browser);
            println!("language_id = {}", cfg.language_id);
            Ok(())
        }
        (Some(key), None) => {
            let cfg = config::global::load()?;
            match key.as_str() {
                "editor" => println!("{}", cfg.editor),
                "browser" => println!("{}", cfg.browser),
                "language_id" => println!("{}", cfg.language_id),
                _ => eprintln!("Unknown config key: {}", key),
            }
            Ok(())
        }
        (Some(key), Some(value)) => {
            let mut cfg = config::global::load()?;
            match key.as_str() {
                "editor" => cfg.editor = value,
                "browser" => cfg.browser = value,
                "language_id" => cfg.language_id = value,
                _ => anyhow::bail!("Unknown config key: {}", key),
            }
            config::global::save(&cfg)?;
            println!("Updated {}.", key);
            Ok(())
        }
        (None, Some(_)) => unreachable!(),
    }
}
