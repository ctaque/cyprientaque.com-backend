-- Your SQL goes here
alter table project_categories add column parent_category_id integer references project_categories(id) on delete restrict;
