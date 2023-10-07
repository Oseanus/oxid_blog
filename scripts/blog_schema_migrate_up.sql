CREATE TABLE posts (
    post_id SERIAL PRIMARY KEY,
    post_date DATE NOT NULL DEFAULT CURRENT_DATE,
    post_title TEXT,
    post_body TEXT)	
);