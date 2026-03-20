use crate::atcoder::AtCoderClient;
use crate::browser;
use crate::config;

pub async fn login() -> anyhow::Result<()> {
    // Open AtCoder login page in browser
    let login_url = "https://atcoder.jp/login";
    browser::open(login_url);

    println!("Opening AtCoder login page in your browser...");
    println!();
    println!("After logging in, please copy the REVEL_SESSION cookie value:");
    println!("  1. Open DevTools (F12)");
    println!("  2. Go to Application tab > Cookies > https://atcoder.jp");
    println!("  3. Find REVEL_SESSION and copy its value");
    println!();

    // Read REVEL_SESSION from stdin
    print!("REVEL_SESSION: ");
    std::io::Write::flush(&mut std::io::stdout())?;
    let mut revel_session = String::new();
    std::io::stdin().read_line(&mut revel_session)?;
    let revel_session = revel_session.trim().to_string();

    if revel_session.is_empty() {
        anyhow::bail!("REVEL_SESSION cannot be empty.");
    }

    // Validate session
    println!("Validating session...");
    let client = AtCoderClient::with_session(&revel_session)?;
    match client.check_session().await? {
        Some(username) => {
            config::session::save(&config::session::SessionConfig { revel_session })?;
            println!("Logged in as {}.", username);
        }
        None => {
            anyhow::bail!(
                "Invalid or expired session. Please make sure you are logged in to AtCoder and copied the correct REVEL_SESSION value."
            );
        }
    }
    Ok(())
}

pub fn logout() -> anyhow::Result<()> {
    config::session::delete()?;
    println!("Logged out.");
    Ok(())
}

pub async fn check() -> anyhow::Result<()> {
    let session = config::session::load()?;
    let client = AtCoderClient::with_session(&session.revel_session)?;
    match client.check_session().await? {
        Some(username) => println!("Logged in as {}.", username),
        None => println!("Session expired. Run `acr login` again."),
    }
    Ok(())
}
