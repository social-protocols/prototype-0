create table post (
    id integer primary key -- row id
    , content text not null
    , parent integer references post (id) on delete cascade on update cascade -- nullable
);

