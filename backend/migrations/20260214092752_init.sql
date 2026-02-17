-- Add migration script here
CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT
);

CREATE TYPE WORKSTATUS AS ENUM ('NotStarted', 'InProgress', 'Completed', 'Cancelled');

CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    project_id INT NOT NULL,
    name TEXT NOT NULL,
    status WORKSTATUS DEFAULT 'NotStarted',
    CONSTRAINT fk_project
        FOREIGN KEY (project_id)
        REFERENCES projects(id)
        ON DELETE CASCADE
);

CREATE TYPE DIFFICULTY AS ENUM ('Easy', 'Medium', 'Hard');

CREATE TABLE jobs (
    id SERIAL PRIMARY KEY,
    task_id INT NOT NULL,
    name TEXT NOT NULL,
    status WORKSTATUS DEFAULT 'NotStarted',
    difficulty DIFFICULTY DEFAULT 'Easy',

    CONSTRAINT fk_task 
        FOREIGN KEY(task_id)
        REFERENCES tasks(id)
        ON DELETE CASCADE
);