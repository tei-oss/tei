```sql
create table users  
(  
    id          serial  
        constraint users_pk primary key,  
    alias       text not null  
        constraint users_alias_unique unique,  
    external_id text not null  
        constraint users_external_id_unique unique  
);  
  
create table groups  
(  
    id         serial  
        constraint groups_pk primary key,  
    title      text not null,  
    created_by integer not null,  
    created_at timestamptz not null,  
    updated_by integer,  
    updated_at timestamptz,  
    version integer,  
    constraint groups_users_created_by_fk  
        foreign key (created_by) references users (id),  
    constraint groups_users_updated_by_fk  
        foreign key (updated_by) references users (id)  
);  
  
create table tags  
(  
    group_id    integer not null,  
    id          bigserial,  
    label       text    not null,  
    description text,  
    color       text,  
    icon        text,  
    created_by integer not null,  
    created_at timestamptz not null,  
    updated_by integer,  
    updated_at timestamptz,  
    version integer,  
    constraint tags_pk  
        primary key (group_id, id),  
    constraint tags_users_created_by_fk  
        foreign key (created_by) references users (id),  
    constraint tags_users_updated_by_fk  
        foreign key (updated_by) references users (id),
    constraint tags_groups_id_fk  
        foreign key (group_id) references groups (id)  
);
```

