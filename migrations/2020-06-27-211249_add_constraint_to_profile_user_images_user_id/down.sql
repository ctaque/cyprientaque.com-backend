-- This file should undo anything in `up.sql`
alter table profile_user_images drop constraint profile_user_images_user_id_foreign;
