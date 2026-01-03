use rusqlite::Connection;
use std::path::PathBuf;
use directories::ProjectDirs;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> anyhow::Result<Self> {
        let db_path = Self::get_db_path()?;
        
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let conn = Connection::open(&db_path)?;
        
        // Initialize schema
        Self::init_schema(&conn)?;
        
        Ok(Database { conn })
    }
    
    fn get_db_path() -> anyhow::Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "amritesh", "eznote")
            .ok_or_else(|| anyhow::anyhow!("Could not determine project directory"))?;
        
        let data_dir = proj_dirs.data_dir();
        Ok(data_dir.join("notes.db"))
    }
    
    fn init_schema(conn: &Connection) -> anyhow::Result<()> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL,
                priority TEXT DEFAULT 'medium' CHECK(priority IN ('low', 'medium', 'high', 'urgent')),
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                is_archived INTEGER DEFAULT 0
            );
            
            CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL
            );
            
            CREATE TABLE IF NOT EXISTS note_tags (
                note_id INTEGER NOT NULL,
                tag_id INTEGER NOT NULL,
                PRIMARY KEY (note_id, tag_id),
                FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
            );
            
            CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(
                content,
                content='notes',
                content_rowid='id'
            );
            
            CREATE TRIGGER IF NOT EXISTS notes_ai AFTER INSERT ON notes BEGIN
                INSERT INTO notes_fts(rowid, content) VALUES (new.id, new.content);
            END;
            
            CREATE TRIGGER IF NOT EXISTS notes_ad AFTER DELETE ON notes BEGIN
                DELETE FROM notes_fts WHERE rowid = old.id;
            END;
            
            CREATE TRIGGER IF NOT EXISTS notes_au AFTER UPDATE ON notes BEGIN
                UPDATE notes_fts SET content = new.content WHERE rowid = old.id;
            END;
            
            CREATE INDEX IF NOT EXISTS idx_notes_created_at ON notes(created_at DESC);
            CREATE INDEX IF NOT EXISTS idx_notes_priority ON notes(priority);
            "#
        )?;
        
        Ok(())
    }
    
    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}
