-- This file should undo anything in `up.sql`
alter table projects drop column if exists user_id;
