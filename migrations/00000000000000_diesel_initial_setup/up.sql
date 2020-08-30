-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.




-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE TABLE person (
    id UUID NOT NULL,
    name VARCHAR NOT NULL,
    create_at TIMESTAMP NOT NULL,
    update_at TIMESTAMP NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE post (
    id UUID NOT NULL,
    person_id UUID NOT NULL,
    text VARCHAR NOT NULL,
    create_at TIMESTAMP NOT NULL,
    update_at TIMESTAMP NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (person_id)
    REFERENCES person (id)
);
