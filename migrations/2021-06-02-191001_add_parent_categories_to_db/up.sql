-- Your SQL goes here
insert into project_categories (name, slug, picture_url, parent_category_id, color_hex) values ('Développement', 'development', (SELECT picture_url from project_categories where slug = 'logiciel') , null, '#000');

insert into project_categories (name, slug, picture_url, parent_category_id, color_hex) values ('Bois et matériaux composites', 'wood&composites', (SELECT picture_url from project_categories where slug = 'mobilier') , null, '#000');
