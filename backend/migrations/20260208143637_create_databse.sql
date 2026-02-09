-- Add migration script here
CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name TEXT,
    tags TEXT[]
);


CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    is_completed BOOLEAN NOT NULL DEFAULT FALSE,
    CONSTRAINT fk_project 
        FOREIGN KEY (project_id)
        REFERENCES projects(id)
        ON DELETE CASCADE
);