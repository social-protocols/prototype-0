create table post (
    id integer primary key -- row id
    , content text not null
    , parent integer references post (id) on delete cascade on update cascade -- nullable
    , question_id integer post (id) on delete cascade on update cascade -- nullable
        -- if a question is a top-level question -> no parent and question_id = id
        -- if a question is a reply -> parent is the id of the post it replies to
        -- -- if it reuses a question -> that question is its question_id
        -- -- if it doesn't reuse a question -> question_id = id
);

create table vote_history (
    post_id not null references post (id) on delete cascade on update cascade
    , user_id not null references user (id) on delete cascade on update cascade
    , timestamp timestamp without timezone
    , direction integer not null
);

create table user (
    id primary key
);

