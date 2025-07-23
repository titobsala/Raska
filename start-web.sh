#!/bin/bash
# Simple script to start the Rask web server

echo "🚀 Starting Rask web server..."
echo "📁 Current directory: $(pwd)"
echo "📂 Checking for .rask directory..."

if [ ! -d ".rask" ]; then
    echo "❌ No .rask directory found!"
    echo "💡 Run 'rask init <roadmap.md>' first"
    exit 1
fi

echo "✅ Found .rask directory"
echo "📝 Starting web server on port 3000..."

# Kill any existing servers
pkill -f "rask web" 2>/dev/null || true

# Build and start server
cargo run --bin rask -- web --port 3000