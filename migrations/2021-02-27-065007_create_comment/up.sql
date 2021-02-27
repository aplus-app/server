create table comment (
    id serial primary key not null,
    post_id varchar not null,
    body varchar not null,
    hearts int not null default 0
)