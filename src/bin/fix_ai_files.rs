//! Standalone binary to fix AI-generated markdown files with JSON encoding issues

use anyhow::Result;
use std::process;

// We need to include the fix_ai_files module
// Since this is a standalone binary, we'll include the logic directly

use serde_json::Value;
use std::fs;
use std::path::Path;

/// Fix AI-generated markdown files that contain JSON-encoded content
fn fix_ai_generated_files() -> Result<()> {
    let files_to_check = ["ai_generated_plan.md", "ai_generated_roadmap.md"];

    for file_path in &files_to_check {
        if Path::new(file_path).exists() {
            match fix_single_file(file_path) {
                Ok(fixed) => {
                    if fixed {
                        println!("✅ Fixed {}", file_path);
                    } else {
                        println!("ℹ️  {} appears to be correctly formatted", file_path);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to fix {}: {}", file_path, e);
                }
            }
        } else {
            println!("📄 {} not found, skipping", file_path);
        }
    }

    Ok(())
}

/// Fix a single markdown file if it contains JSON-encoded content
fn fix_single_file(file_path: &str) -> Result<bool> {
    let content = fs::read_to_string(file_path)?;

    // Check if the content looks like a JSON-encoded string
    if is_json_encoded_markdown(&content) {
        let fixed_content = decode_json_markdown(&content)?;

        // Create backup before fixing
        let backup_path = format!("{}.backup", file_path);
        fs::write(&backup_path, &content)?;

        // Write the fixed content
        fs::write(file_path, fixed_content)?;

        println!("📦 Created backup: {}", backup_path);
        return Ok(true);
    }

    Ok(false)
}

/// Check if content appears to be JSON-encoded markdown
fn is_json_encoded_markdown(content: &str) -> bool {
    let trimmed = content.trim();

    // Check if it starts and ends with quotes and contains literal \n
    (trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.contains("\\n"))
    ||
    // Check if it looks like markdown wrapped in a JSON string
    (trimmed.starts_with("markdown\\n") || trimmed.starts_with("\"markdown\\n"))
    ||
    // Check if it contains literal \n sequences that should be newlines
    (trimmed.contains("\\n#") || trimmed.contains("\\n\\n"))
}

/// Decode JSON-encoded markdown content
fn decode_json_markdown(content: &str) -> Result<String> {
    let trimmed = content.trim();

    // Try parsing as a JSON string first
    if let Ok(Value::String(decoded)) = serde_json::from_str(trimmed) {
        return Ok(decoded);
    }

    // If that fails, try manual decoding
    let mut result = trimmed.to_string();

    // Remove wrapping quotes if present
    if result.starts_with('"') && result.ends_with('"') {
        result = result[1..result.len() - 1].to_string();
    }

    // Remove markdown prefix if present
    if result.starts_with("markdown\\n") {
        result = result[9..].to_string();
    }

    // Replace literal \n with actual newlines
    result = result.replace("\\n", "\n");

    // Replace literal \t with actual tabs
    result = result.replace("\\t", "\t");

    // Replace escaped quotes
    result = result.replace("\\\"", "\"");

    // Replace other common escape sequences
    result = result.replace("\\r", "\r");
    result = result.replace("\\\\", "\\");

    // Clean up any remaining artifacts
    result = result.trim().to_string();

    // Ensure the content ends with a single newline
    if !result.ends_with('\n') {
        result.push('\n');
    }

    Ok(result)
}

fn main() {
    println!("🔧 Fixing AI-generated markdown files...\n");

    if let Err(e) = fix_ai_generated_files() {
        eprintln!("❌ Error: {}", e);
        process::exit(1);
    }

    println!("\n✨ Done! Check the files to ensure they render correctly now.");
    println!("💡 Tip: Use 'cat filename.md' or open in your editor to verify the formatting.");
}
