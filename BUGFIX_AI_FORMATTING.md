# AI Formatting Bug Fix

## Problem Description

AI-generated markdown files were being incorrectly formatted with JSON encoding, making them unreadable in terminals and markdown viewers. The files contained literal `\n` characters instead of actual line breaks and were wrapped in JSON string quotes.

### Symptoms
- Files like `ai_generated_plan.md` and `ai_generated_roadmap.md` contained content like:
  ```
  "# Project Title\n\nContent with \\n instead of newlines"
  ```
- Terminal rendering showed raw escape sequences instead of formatted markdown
- Files were unreadable in markdown viewers

## Root Cause

The issue was in `src/commands/ai.rs` in the `handle_ai_roadmap` function. When an output path was specified, the code was incorrectly JSON-serializing the AI response string:

```rust
// Problematic code:
let json_output = serde_json::to_string_pretty(&roadmap)
    .map_err(|e| format!("Failed to serialize roadmap: {}", e))?;
fs::write(output_path, json_output)
```

The `roadmap` variable in this context was already a formatted markdown string returned by `generate_project_roadmap()`, but the code was treating it as if it were a structured object that needed JSON serialization.

## Solution

### 1. Fixed the AI Command (Immediate Fix)

Updated the `handle_ai_roadmap` function to write the markdown content directly:

```rust
// Fixed code:
fs::write(output_path, &roadmap)
    .map_err(|e| format!("Failed to write to file: {}", e))?;
```

### 2. Created Recovery Utility

Built a standalone utility (`src/bin/fix_ai_files.rs`) that:
- Detects JSON-encoded markdown files
- Creates backups of original files
- Decodes the content back to proper markdown format
- Handles various escape sequences (`\n`, `\t`, `\"`, etc.)

### 3. Usage

To fix existing broken files:
```bash
cargo run --bin fix_ai_files
```

## Prevention

The fix ensures that:
1. Future AI-generated roadmaps are written as plain markdown text
2. No JSON serialization occurs on markdown content
3. Files are immediately readable in terminals and editors

## Files Changed

- `src/commands/ai.rs` - Fixed the JSON serialization issue
- `src/bin/fix_ai_files.rs` - Created recovery utility
- `fix_ai_files.rs` - Standalone module (can be removed after integration)

## Testing

After applying the fix:
1. AI-generated files render correctly in terminal (`cat filename.md`)
2. Markdown viewers display proper formatting
3. No literal escape sequences appear in the content
4. Line breaks and formatting are preserved

## Technical Details

The `generate_project_roadmap()` method in `AiService` returns `Result<String>` containing properly formatted markdown. The bug occurred because the calling code assumed this string needed further serialization, when it should be written directly to disk.

This type of bug could occur in other AI command handlers that save output to files. Always verify that:
- String responses are written directly as text
- Only structured data (like analysis results) should be JSON-serialized
- File extensions match the actual content format (.md for markdown, .json for JSON)