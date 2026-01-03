use crate::db::Database;
use crate::models::{Note, Priority, Stats};
use chrono::{Utc, Duration};
use rusqlite::params;

pub struct NoteService {
    db: Database,
}

impl NoteService {
    pub fn new(db: Database) -> Self {
        NoteService { db }
    }
    
    pub fn add_note(&self, content: &str, tags: Vec<String>, priority: Priority) -> anyhow::Result<Note> {
        let now = Utc::now();
        let conn = self.db.connection();
        
        // Insert note
        conn.execute(
            "INSERT INTO notes (content, priority, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![content, priority.to_string(), now.to_rfc3339(), now.to_rfc3339()],
        )?;
        
        let note_id = conn.last_insert_rowid();
        
        // Add tags
        for tag in &tags {
            self.add_tag_to_note(note_id, tag)?;
        }
        
        // Return the created note
        self.get_note(note_id)
    }
    
    pub fn get_note(&self, id: i64) -> anyhow::Result<Note> {
        let conn = self.db.connection();
        
        let mut stmt = conn.prepare(
            "SELECT id, content, priority, created_at, updated_at, is_archived FROM notes WHERE id = ?1"
        )?;
        
        let note = stmt.query_row(params![id], |row| {
            Ok(Note {
                id: Some(row.get(0)?),
                content: row.get(1)?,
                priority: Priority::from_str(&row.get::<_, String>(2)?).unwrap(),
                created_at: row.get::<_, String>(3)?.parse().unwrap(),
                updated_at: row.get::<_, String>(4)?.parse().unwrap(),
                is_archived: row.get::<_, i32>(5)? != 0,
                tags: Vec::new(),
            })
        })?;
        
        // Load tags
        let tags = self.get_tags_for_note(id)?;
        
        Ok(Note { tags, ..note })
    }
    
    pub fn list_recent(&self, limit: usize) -> anyhow::Result<Vec<Note>> {
        let conn = self.db.connection();
        
        let mut stmt = conn.prepare(
            "SELECT id, content, priority, created_at, updated_at, is_archived 
             FROM notes 
             WHERE is_archived = 0 
             ORDER BY created_at DESC 
             LIMIT ?1"
        )?;
        
        let notes = stmt.query_map(params![limit], |row| {
            Ok(Note {
                id: Some(row.get(0)?),
                content: row.get(1)?,
                priority: Priority::from_str(&row.get::<_, String>(2)?).unwrap(),
                created_at: row.get::<_, String>(3)?.parse().unwrap(),
                updated_at: row.get::<_, String>(4)?.parse().unwrap(),
                is_archived: row.get::<_, i32>(5)? != 0,
                tags: Vec::new(),
            })
        })?;
        
        let mut result = Vec::new();
        for note in notes {
            let mut note = note?;
            note.tags = self.get_tags_for_note(note.id.unwrap())?;
            result.push(note);
        }
        
        Ok(result)
    }
    
    pub fn list_today(&self, limit: usize) -> anyhow::Result<Vec<Note>> {
        let today_start = Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap();
        let today_start = chrono::DateTime::<Utc>::from_naive_utc_and_offset(today_start, Utc);
        
        let conn = self.db.connection();
        
        let mut stmt = conn.prepare(
            "SELECT id, content, priority, created_at, updated_at, is_archived 
             FROM notes 
             WHERE is_archived = 0 AND created_at >= ?1
             ORDER BY created_at DESC 
             LIMIT ?2"
        )?;
        
        let notes = stmt.query_map(params![today_start.to_rfc3339(), limit], |row| {
            Ok(Note {
                id: Some(row.get(0)?),
                content: row.get(1)?,
                priority: Priority::from_str(&row.get::<_, String>(2)?).unwrap(),
                created_at: row.get::<_, String>(3)?.parse().unwrap(),
                updated_at: row.get::<_, String>(4)?.parse().unwrap(),
                is_archived: row.get::<_, i32>(5)? != 0,
                tags: Vec::new(),
            })
        })?;
        
        let mut result = Vec::new();
        for note in notes {
            let mut note = note?;
            note.tags = self.get_tags_for_note(note.id.unwrap())?;
            result.push(note);
        }
        
        Ok(result)
    }
    
    pub fn list_by_tag(&self, tag: &str, limit: usize) -> anyhow::Result<Vec<Note>> {
        let conn = self.db.connection();
        
        let mut stmt = conn.prepare(
            "SELECT n.id, n.content, n.priority, n.created_at, n.updated_at, n.is_archived 
             FROM notes n
             JOIN note_tags nt ON n.id = nt.note_id
             JOIN tags t ON nt.tag_id = t.id
             WHERE n.is_archived = 0 AND t.name = ?1
             ORDER BY n.created_at DESC 
             LIMIT ?2"
        )?;
        
        let notes = stmt.query_map(params![tag, limit], |row| {
            Ok(Note {
                id: Some(row.get(0)?),
                content: row.get(1)?,
                priority: Priority::from_str(&row.get::<_, String>(2)?).unwrap(),
                created_at: row.get::<_, String>(3)?.parse().unwrap(),
                updated_at: row.get::<_, String>(4)?.parse().unwrap(),
                is_archived: row.get::<_, i32>(5)? != 0,
                tags: Vec::new(),
            })
        })?;
        
        let mut result = Vec::new();
        for note in notes {
            let mut note = note?;
            note.tags = self.get_tags_for_note(note.id.unwrap())?;
            result.push(note);
        }
        
        Ok(result)
    }
    
    pub fn search(&self, query: &str) -> anyhow::Result<Vec<Note>> {
        let conn = self.db.connection();
        
        let mut stmt = conn.prepare(
            "SELECT n.id, n.content, n.priority, n.created_at, n.updated_at, n.is_archived 
             FROM notes n
             JOIN notes_fts ON n.id = notes_fts.rowid
             WHERE notes_fts MATCH ?1 AND n.is_archived = 0
             ORDER BY n.created_at DESC"
        )?;
        
        let notes = stmt.query_map(params![query], |row| {
            Ok(Note {
                id: Some(row.get(0)?),
                content: row.get(1)?,
                priority: Priority::from_str(&row.get::<_, String>(2)?).unwrap(),
                created_at: row.get::<_, String>(3)?.parse().unwrap(),
                updated_at: row.get::<_, String>(4)?.parse().unwrap(),
                is_archived: row.get::<_, i32>(5)? != 0,
                tags: Vec::new(),
            })
        })?;
        
        let mut result = Vec::new();
        for note in notes {
            let mut note = note?;
            note.tags = self.get_tags_for_note(note.id.unwrap())?;
            result.push(note);
        }
        
        Ok(result)
    }
    
    pub fn delete_note(&self, id: i64) -> anyhow::Result<()> {
        let conn = self.db.connection();
        conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
        Ok(())
    }
    
    pub fn get_stats(&self) -> anyhow::Result<Stats> {
        let conn = self.db.connection();
        
        let total: usize = conn.query_row(
            "SELECT COUNT(*) FROM notes WHERE is_archived = 0",
            [],
            |row| row.get(0),
        )?;
        
        let today_start = Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap();
        let today_start = chrono::DateTime::<Utc>::from_naive_utc_and_offset(today_start, Utc);
        
        let today: usize = conn.query_row(
            "SELECT COUNT(*) FROM notes WHERE is_archived = 0 AND created_at >= ?1",
            params![today_start.to_rfc3339()],
            |row| row.get(0),
        )?;
        
        let week_start = today_start - Duration::days(7);
        let week: usize = conn.query_row(
            "SELECT COUNT(*) FROM notes WHERE is_archived = 0 AND created_at >= ?1",
            params![week_start.to_rfc3339()],
            |row| row.get(0),
        )?;
        
        let month_start = today_start - Duration::days(30);
        let month: usize = conn.query_row(
            "SELECT COUNT(*) FROM notes WHERE is_archived = 0 AND created_at >= ?1",
            params![month_start.to_rfc3339()],
            |row| row.get(0),
        )?;
        
        let urgent: usize = conn.query_row(
            "SELECT COUNT(*) FROM notes WHERE is_archived = 0 AND priority = 'urgent'",
            [],
            |row| row.get(0),
        )?;
        
        let high: usize = conn.query_row(
            "SELECT COUNT(*) FROM notes WHERE is_archived = 0 AND priority = 'high'",
            [],
            |row| row.get(0),
        )?;
        
        let medium: usize = conn.query_row(
            "SELECT COUNT(*) FROM notes WHERE is_archived = 0 AND priority = 'medium'",
            [],
            |row| row.get(0),
        )?;
        
        let low: usize = conn.query_row(
            "SELECT COUNT(*) FROM notes WHERE is_archived = 0 AND priority = 'low'",
            [],
            |row| row.get(0),
        )?;
        
        Ok(Stats {
            total,
            today,
            week,
            month,
            urgent,
            high,
            medium,
            low,
        })
    }
    
    fn add_tag_to_note(&self, note_id: i64, tag: &str) -> anyhow::Result<()> {
        let conn = self.db.connection();
        
        // Get or create tag
        conn.execute(
            "INSERT OR IGNORE INTO tags (name) VALUES (?1)",
            params![tag],
        )?;
        
        let tag_id: i64 = conn.query_row(
            "SELECT id FROM tags WHERE name = ?1",
            params![tag],
            |row| row.get(0),
        )?;
        
        // Link tag to note
        conn.execute(
            "INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
            params![note_id, tag_id],
        )?;
        
        Ok(())
    }
    
    fn get_tags_for_note(&self, note_id: i64) -> anyhow::Result<Vec<String>> {
        let conn = self.db.connection();
        
        let mut stmt = conn.prepare(
            "SELECT t.name FROM tags t
             JOIN note_tags nt ON t.id = nt.tag_id
             WHERE nt.note_id = ?1"
        )?;
        
        let tags = stmt.query_map(params![note_id], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;
        
        Ok(tags)
    }
}
