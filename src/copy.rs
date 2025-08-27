use copypasta::{ClipboardContext, ClipboardProvider};
use std::fs;

pub fn add_file_to_clipboard(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the file contents
    let contents = fs::read_to_string(file_path)?;

    let mut ctx = ClipboardContext::new().unwrap();

    ctx.set_contents(contents).unwrap();

    println!(
        "Copied contents of {} to clipboard. Paste it in your browser's console (f12) console.",
        file_path
    );
    Ok(())
}
