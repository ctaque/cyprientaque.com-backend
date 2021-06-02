-- This file should undo anything in `up.sql`
alter table project_categories alter column picture_url drop not null;
