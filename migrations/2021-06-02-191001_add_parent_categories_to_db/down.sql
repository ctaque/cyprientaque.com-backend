-- This file should undo anything in `up.sql`
delete from project_categories where slug = 'wood&composites';
delete from project_categories where slug = 'development';
