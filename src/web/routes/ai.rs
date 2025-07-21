//! AI-related API routes

use axum::{
    extract::{Json as ExtractJson, Path},
    http::StatusCode,
    response::{Json, Sse, sse::Event},
    routing::{get, post},
    Router,
};
use futures_util::stream::{self, Stream};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::StreamExt as _;

/// AI routes
pub fn routes() -> Router {
    Router::new()
        .route("/chat", post(chat))
        .route("/chat/stream", post(chat_stream))
        .route("/analyze", post(analyze))
        .route("/breakdown", post(breakdown))
        .route("/suggest", post(suggest))
        .route("/insights", get(insights))
}

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
    project_context: Option<String>,
    session_id: Option<String>,
}

#[derive(Serialize)]
struct ChatResponse {
    response: String,
    session_id: String,
}

/// AI chat endpoint (non-streaming)
async fn chat(ExtractJson(payload): ExtractJson<ChatRequest>) -> Result<Json<ChatResponse>, StatusCode> {
    // TODO: Integrate with existing AI service
    let session_id = payload.session_id.unwrap_or_else(|| {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        format!("session_{}", timestamp)
    });

    // Simulate AI processing
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    let response = format!(
        "Hello! You asked: '{}'. This is a simulated AI response. Full AI integration coming soon!",
        payload.message
    );

    Ok(Json(ChatResponse {
        response,
        session_id,
    }))
}

/// AI chat endpoint with streaming response
async fn chat_stream(
    ExtractJson(payload): ExtractJson<ChatRequest>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let message = payload.message.clone();
    
    // Create a stream that simulates AI response chunks
    let stream = stream::iter(simulate_ai_response_chunks(&message))
        .then(|chunk| async move {
            // Simulate processing delay for each chunk
            tokio::time::sleep(Duration::from_millis(50)).await;
            
            Ok(Event::default()
                .json_data(json!({
                    "type": "chunk",
                    "content": chunk,
                    "done": false
                }))
                .unwrap())
        })
        .chain(stream::once(async {
            // Send final message indicating completion
            Ok(Event::default()
                .json_data(json!({
                    "type": "done",
                    "content": "",
                    "done": true
                }))
                .unwrap())
        }));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

/// Simulate AI response chunks for streaming
fn simulate_ai_response_chunks(message: &str) -> Vec<String> {
    let full_response = format!(
        "Thank you for your message: '{}'. This is a streaming AI response that demonstrates how the chat interface will work once fully integrated with the Gemini AI service. The response is being delivered in real-time chunks to provide a better user experience.",
        message
    );
    
    // Split response into word chunks for streaming effect
    let words: Vec<&str> = full_response.split_whitespace().collect();
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    
    for (i, word) in words.iter().enumerate() {
        current_chunk.push_str(word);
        
        // Send chunks of 2-4 words
        if i % 3 == 2 || i == words.len() - 1 {
            chunks.push(current_chunk.clone());
            current_chunk = " ".to_string();
        } else {
            current_chunk.push(' ');
        }
    }
    
    chunks
}

/// AI task analysis
async fn analyze() -> Result<Json<Value>, StatusCode> {
    // TODO: Use existing AI analysis functionality
    Ok(Json(json!({
        "analysis": {},
        "message": "AI analysis not yet implemented"
    })))
}

/// AI task breakdown
async fn breakdown() -> Result<Json<Value>, StatusCode> {
    // TODO: Use existing AI breakdown functionality
    Ok(Json(json!({
        "tasks": [],
        "message": "AI breakdown not yet implemented"
    })))
}

/// AI suggestions
async fn suggest() -> Result<Json<Value>, StatusCode> {
    // TODO: Use existing AI suggestion functionality
    Ok(Json(json!({
        "suggestions": [],
        "message": "AI suggestions not yet implemented"
    })))
}

/// AI insights
async fn insights() -> Result<Json<Value>, StatusCode> {
    // TODO: Use existing AI insights functionality
    Ok(Json(json!({
        "insights": {},
        "message": "AI insights not yet implemented"
    })))
}