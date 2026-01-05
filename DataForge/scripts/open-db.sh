#!/bin/bash
# Open DataForge database in DB Browser for SQLite

DB_PATH="$HOME/Library/Application Support/com.dataforge.app/dataforge.db"
DB_DIR="$HOME/Library/Application Support/com.dataforge.app"

# Check if database exists
if [ ! -f "$DB_PATH" ]; then
    echo "⚠️  Database not found at: $DB_PATH"
    echo ""
    echo "Creating database with schema..."
    
    # Try to create database using cargo
    if command -v cargo &> /dev/null; then
        cd "$(dirname "$0")/.." || exit 1
        cargo run --manifest-path crates/dataforge-core/Cargo.toml --bin create-db
        if [ $? -eq 0 ]; then
            echo ""
            echo "✅ Database created! Opening now..."
        else
            echo ""
            echo "❌ Failed to create database. Please launch the DataForge app once to create it."
            exit 1
        fi
    else
        echo "Cargo not found. Please launch the DataForge app once to create the database."
        exit 1
    fi
fi

# Try to open with DB Browser for SQLite
if [ -d "/Applications/DB Browser for SQLite.app" ]; then
    echo "📂 Opening database in DB Browser for SQLite..."
    open -a "DB Browser for SQLite" "$DB_PATH"
    echo "✅ Database opened!"
else
    echo "❌ DB Browser for SQLite not found in Applications folder"
    echo ""
    echo "Please install DB Browser for SQLite from:"
    echo "  https://sqlitebrowser.org/"
    echo ""
    echo "Or open manually:"
    echo "  open \"$DB_PATH\""
    exit 1
fi

