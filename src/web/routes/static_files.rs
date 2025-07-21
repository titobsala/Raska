//! Static file serving for the React frontend

use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use include_dir::{include_dir, Dir};

// Embed the React build directory at compile time
static REACT_BUILD: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/web-ui/dist");

/// Static file routes
pub fn routes() -> Router {
    Router::new()
        .route("/", get(serve_index))
        .route("/index.html", get(serve_index))
        .route("/assets/*file", get(serve_static_file))
}

/// Serve the main index.html file
async fn serve_index() -> Html<&'static str> {
    // Try to serve the built React app, fall back to default
    if let Some(index_file) = REACT_BUILD.get_file("index.html") {
        if let Ok(content) = std::str::from_utf8(index_file.contents()) {
            return Html(content);
        }
    }

    // Fallback to default HTML
    Html(DEFAULT_INDEX)
}

/// Default index.html content when React build is not available
const DEFAULT_INDEX: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rask Web Interface</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 2rem;
            background: #f5f5f5;
        }
        .container {
            max-width: 800px;
            margin: 0 auto;
            background: white;
            padding: 2rem;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 Rask Web Interface</h1>
        <p>React frontend coming soon...</p>
    </div>
</body>
</html>
"#;

/// Serve static files from the embedded React build
async fn serve_static_file(Path(file_path): Path<String>) -> impl IntoResponse {
    // The file_path is already the file name without "assets/" prefix
    let full_path = format!("assets/{}", file_path);

    if let Some(file) = REACT_BUILD.get_file(&full_path) {
        let content_type = get_content_type(&full_path);

        let response = Response::builder()
            .status(StatusCode::OK)
            .header("content-type", content_type)
            .body(Body::from(file.contents().to_vec()))
            .unwrap();

        response
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()
    }
}

/// Get the appropriate MIME type for a file based on its extension
fn get_content_type(path: &str) -> &'static str {
    if path.ends_with(".js") {
        "application/javascript"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".html") {
        "text/html"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        "image/jpeg"
    } else if path.ends_with(".ico") {
        "image/x-icon"
    } else {
        "application/octet-stream"
    }
}
