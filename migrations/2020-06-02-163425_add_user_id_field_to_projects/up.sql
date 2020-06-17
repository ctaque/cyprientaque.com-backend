-- Your SQL goes here
alter table projects add column if not exists user_id int not null references users(id);
