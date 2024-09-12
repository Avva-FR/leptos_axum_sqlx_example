CREATE TABLE IF NOT EXISTS blog_entries (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    date TEXT,
    content TEXT NOT NULL,
    img_path BYTEA
);
