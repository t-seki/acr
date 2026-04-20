use crate::config;

pub fn execute() -> anyhow::Result<()> {
    let config_dir = config::config_dir()?;
    std::fs::create_dir_all(&config_dir)?;

    // config.toml — always (re-)prompt so `acr init` can also update an
    // existing configuration. Existing values are offered as the prompt
    // default, so pressing Enter keeps them.
    let config_path = config_dir.join("config.toml");
    let existed = config_path.exists();
    let existing = if existed {
        config::global::load()?
    } else {
        config::global::GlobalConfig::default()
    };
    let editor = prompt_with_default("Editor", &existing.editor)?;
    let browser = prompt_with_default("Browser", &existing.browser)?;
    let cfg = config::global::GlobalConfig { editor, browser };
    config::global::save(&cfg)?;
    println!(
        "{} config.toml",
        if existed { "Updated" } else { "Created" }
    );

    // template.rs
    let template_path = config::global::template_path()?;
    if template_path.exists() {
        println!("template.rs already exists, skipping.");
    } else {
        std::fs::write(&template_path, config::global::default_template())?;
        println!("Created template.rs");
    }

    // .cargo/config.toml (shared target directory)
    let cargo_config_dir = std::env::current_dir()?.join(".cargo");
    let cargo_config_path = cargo_config_dir.join("config.toml");
    if cargo_config_path.exists() {
        println!(".cargo/config.toml already exists, skipping.");
    } else {
        std::fs::create_dir_all(&cargo_config_dir)?;
        std::fs::write(&cargo_config_path, "[build]\ntarget-dir = \"target\"\n")?;
        println!("Created .cargo/config.toml");
    }

    // .gitignore
    let gitignore_path = std::env::current_dir()?.join(".gitignore");
    if gitignore_path.exists() {
        println!(".gitignore already exists, skipping.");
    } else {
        std::fs::write(&gitignore_path, "/target\n")?;
        println!("Created .gitignore");
    }

    println!("Initialization complete!");
    Ok(())
}

fn prompt_with_default(label: &str, default: &str) -> anyhow::Result<String> {
    print!("{} [{}]: ", label, default);
    std::io::Write::flush(&mut std::io::stdout())?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();
    Ok(if trimmed.is_empty() {
        default.to_string()
    } else {
        trimmed.to_string()
    })
}
