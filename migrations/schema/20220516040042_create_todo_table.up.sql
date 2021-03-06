-- Add up migration script here
create table todo (
    id bigint auto_increment primary key,
    title varchar(255),
    description text,
    created_at datetime default current_timestamp,
    updated_at datetime default current_timestamp on update current_timestamp,
    deleted_at datetime default null
) character set utf8mb4 collate utf8mb4_bin;