-- Your SQL goes here
alter table projects add column user_id int not null references users(id);
