create table post (
    id serial primary key,
    category varchar not null,
    user_name varchar not null,
    user_id varchar not null,
    title varchar not null,
    body varchar not null
)