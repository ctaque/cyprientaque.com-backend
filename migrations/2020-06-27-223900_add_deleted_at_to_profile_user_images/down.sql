-- This file should undo anything in `up.sql`
alter table profile_user_images drop column deleted_at;
