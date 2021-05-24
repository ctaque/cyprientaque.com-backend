-- This file should undo anything in `up.sql`
alter table project_categories drop column parent_category_id;
