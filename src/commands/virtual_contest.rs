use chrono::Timelike;

use crate::atcoder;
use crate::browser;
use crate::workspace;
use crate::workspace::CurrentContext;

pub async fn execute(
    contest_id: String,
    problems: Vec<String>,
    at: Option<String>,
) -> anyhow::Result<()> {
    let current = workspace::detect_current_context();
    match current {
        CurrentContext::ProblemDir(_) | CurrentContext::ContestDir(_) => {
            anyhow::bail!(
                "Cannot create a new contest inside a problem or contest directory."
            );
        }
        CurrentContext::Outside => {}
    }

    // Determine start time
    let target = match &at {
        Some(time_str) => super::new::resolve_target_time(time_str)?,
        None => auto_calculate_start_time(),
    };

    // Format as YYYY-MM-DD HH:MM:SS for pasting into AtCoder form
    let formatted_time = target.format("%Y-%m-%d %H:%M:%S").to_string();

    // Copy to clipboard
    let copied = arboard::Clipboard::new()
        .and_then(|mut cb| cb.set_text(&formatted_time))
        .is_ok();

    // Open virtual registration page in browser
    let virtual_url = format!("{}/contests/{}/virtual", atcoder::BASE_URL, contest_id);
    browser::open(&virtual_url);

    if copied {
        println!(
            "Start time copied to clipboard: {}",
            formatted_time
        );
    } else {
        println!(
            "Start time: {} (clipboard copy failed)",
            formatted_time
        );
    }
    println!("Opened virtual registration page. Paste the time and click register.");

    // Wait until start time
    super::new::wait_until(target).await?;

    // Create workspace (same as `acr new --at`)
    super::new::setup_contest_workspace(&contest_id, &problems, true).await
}

/// Auto-calculate start time: next full minute + 1 minute buffer.
/// E.g., if now is 07:56:25, returns 07:58:00.
fn auto_calculate_start_time() -> chrono::DateTime<chrono::Local> {
    let now = chrono::Local::now();
    let next_minute = now + chrono::Duration::minutes(1);
    let rounded = next_minute
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap();
    rounded + chrono::Duration::minutes(1)
}
