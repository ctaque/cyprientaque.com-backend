-- Your SQL goes here
alter table project_categories add column created_at timestamp not null default CURRENT_TIMESTAMP;
alter table project_categories add column updated_at timestamp not null default CURRENT_TIMESTAMP;
alter table project_categories add column deleted_at timestamp;
