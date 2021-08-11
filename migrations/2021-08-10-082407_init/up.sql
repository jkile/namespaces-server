-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR ( 50 ) NOT NULL UNIQUE,
    password VARCHAR ( 50 ) NOT NULL,
    first_name VARCHAR ( 50 ),
    last_name VARCHAR ( 50 ),
    email VARCHAR ( 255 ) UNIQUE NOT NULL,
    created_on TIMESTAMP NOT NULL,
    last_login TIMESTAMP
);

CREATE TABLE namespaces (
    name VARCHAR ( 50 ) PRIMARY KEY,
    owner_id SERIAL NOT NULL,
    FOREIGN KEY ( owner_id ) REFERENCES users(id)
);

CREATE TABLE namespaces_users (
    user_id SERIAL NOT NULL,
    namespace_name VARCHAR( 50 ) NOT NULL,
    PRIMARY KEY (user_id, namespace_name),
    FOREIGN KEY ( user_id ) REFERENCES users(id),
    FOREIGN KEY ( namespace_name ) REFERENCES namespaces(name)
);

CREATE TABLE threads (
    id SERIAL PRIMARY KEY,
    title VARCHAR ( 140 ) NOT NULL,
    last_posted TIMESTAMP NOT NULL,
    namespace_name VARCHAR ( 50 ) NOT NULL,
    FOREIGN KEY (namespace_name) REFERENCES namespaces(name)
);

CREATE TABLE threads_users (
    author_id SERIAL NOT NULL,
    thread_id SERIAL NOT NULL,
    PRIMARY KEY (author_id, thread_id),
    FOREIGN KEY (author_id) REFERENCES users(id),
    FOREIGN KEY (thread_id) REFERENCES threads(id)
);

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    author_id SERIAL NOT NULL,
    FOREIGN KEY (author_id) REFERENCES users(id),
    timestamp TIMESTAMP NOT NULL,
    contents VARCHAR ( 3000 ) NOT NULL,
    thread_position INT NOT NULL,
    thread_id SERIAL NOT NULL,
    FOREIGN KEY (thread_id) REFERENCES threads(id)
);