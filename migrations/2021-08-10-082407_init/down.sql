-- This file should undo anything in `up.sql`
DROP TABLE users CASCADE;
DROP TABLE namespaces CASCADE;
DROP TABLE namespaces_users CASCADE;
DROP TABLE threads CASCADE;
DROP TABLE threads_users CASCADE;
DROP TABLE comments CASCADE;