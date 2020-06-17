-- This file should undo anything in `up.sql`
alter table project_categories drop column created_at;
alter table project_categories drop column updated_at;
alter table project_categories drop column deleted_at;
