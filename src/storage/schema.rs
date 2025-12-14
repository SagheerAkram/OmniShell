pub const CREATE_TABLES_SQL: &str = r#"
-- Contacts table
CREATE TABLE IF NOT EXISTS contacts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    public_key BLOB NOT NULL,
    fingerprint TEXT NOT NULL,
    trust_level TEXT DEFAULT 'medium',
    last_seen INTEGER,
    created_at INTEGER NOT NULL,
    notes TEXT
);

-- Messages table
CREATE TABLE IF NOT EXISTS messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    contact_id INTEGER NOT NULL,
    direction TEXT NOT NULL, -- 'sent' or 'received'
    content_encrypted BLOB NOT NULL,
    timestamp INTEGER NOT NULL,
    protocol TEXT NOT NULL,
    status TEXT NOT NULL, -- 'pending', 'delivered', 'read', 'failed'
    message_id TEXT UNIQUE NOT NULL,
    reply_to TEXT,
    edited_at INTEGER,
    deleted_at INTEGER,
    FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE CASCADE
);

-- Groups table
CREATE TABLE IF NOT EXISTS groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    created_at INTEGER NOT NULL,
    encryption_key BLOB NOT NULL,
    settings TEXT -- JSON blob for group settings
);

-- Group members table
CREATE TABLE IF NOT EXISTS group_members (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    group_id INTEGER NOT NULL,
    contact_id INTEGER NOT NULL,
    role TEXT NOT NULL DEFAULT 'member', -- 'admin' or 'member'
    joined_at INTEGER NOT NULL,
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
    FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE CASCADE,
    UNIQUE(group_id, contact_id)
);

-- Message queue table
CREATE TABLE IF NOT EXISTS queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    message_id TEXT NOT NULL,
    recipient TEXT NOT NULL,
    encrypted_content BLOB NOT NULL,
    protocol TEXT,
    priority TEXT NOT NULL DEFAULT 'normal',
    retry_count INTEGER DEFAULT 0,
    next_retry INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    last_error TEXT
);

-- Configuration table
CREATE TABLE IF NOT EXISTS config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Message reactions table
CREATE TABLE IF NOT EXISTS reactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    message_id TEXT NOT NULL,
    contact_id INTEGER NOT NULL,
    emoji TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE CASCADE,
    UNIQUE(message_id, contact_id)
);

-- Starred messages table
CREATE TABLE IF NOT EXISTS starred_messages (
    message_id TEXT PRIMARY KEY,
    starred_at INTEGER NOT NULL
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_messages_contact_id ON messages(contact_id);
CREATE INDEX IF NOT EXISTS idx_messages_timestamp ON messages(timestamp);
CREATE INDEX IF NOT EXISTS idx_messages_status ON messages(status);
CREATE INDEX IF NOT EXISTS idx_queue_next_retry ON queue(next_retry);
CREATE INDEX IF NOT EXISTS idx_reactions_message_id ON reactions(message_id);
"#;
