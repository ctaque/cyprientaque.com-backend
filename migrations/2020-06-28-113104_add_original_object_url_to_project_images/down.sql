-- This file should undo anything in `up.sql`
alter table project_images drop column if exists original_object_url;
