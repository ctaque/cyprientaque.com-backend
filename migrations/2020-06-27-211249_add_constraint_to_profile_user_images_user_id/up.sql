-- Your SQL goes here
alter table profile_user_images add constraint profile_user_images_user_id_foreign foreign key (user_id) references users(id);
