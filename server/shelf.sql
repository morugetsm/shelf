create or replace table auth_user
(
    id        int auto_increment
        primary key,
    username  varchar(16)                            not null,
    password  varchar(64)                            not null,
    name      varchar(32)                            not null,
    admin_yn  tinyint(1) default 0                   not null,
    reg_date  datetime   default current_timestamp() not null,
    mod_date  datetime                               null,
    delete_yn tinyint(1) default 0                   not null
);

create or replace table proj_project
(
    id          int auto_increment
        primary key,
    manager_id  int                                    not null,
    title       varchar(64)                            not null,
    start_date  datetime                               null,
    end_date    datetime                               null,
    description text       default ''                  not null,
    reg_date    datetime   default current_timestamp() not null,
    mod_date    datetime                               null,
    delete_yn   tinyint(1) default 0                   not null,
    constraint proj_project_auth_user_id_fk
        foreign key (manager_id) references auth_user (id)
);

create or replace table proj_post
(
    id         int auto_increment
        primary key,
    project_id int                                    not null,
    subject    varchar(64)                            not null,
    author_id  int                                    not null,
    manager_id int                                    null,
    status     tinyint    default 0                   not null,
    start_date datetime                               not null,
    end_date   datetime                               not null,
    content    text       default ''                  not null,
    share_yn   tinyint(1) default 0                   not null,
    notice_yn  tinyint(1) default 0                   not null,
    reg_date   datetime   default current_timestamp() not null,
    mod_date   datetime                               null,
    delete_yn  tinyint(1) default 0                   not null,
    constraint proj_post_author_auth_user_id_fk
        foreign key (author_id) references auth_user (id),
    constraint proj_post_manager_auth_user_id_fk
        foreign key (manager_id) references auth_user (id),
    constraint proj_post_proj_project_id_fk
        foreign key (project_id) references proj_project (id)
);

create or replace table proj_comment
(
    id        int auto_increment
        primary key,
    post_id   int                                    not null,
    author_id int                                    not null,
    content   text                                   not null,
    reg_date  datetime   default current_timestamp() not null,
    mod_date  datetime                               null,
    delete_yn tinyint(1) default 0                   not null,
    constraint proj_comment_auth_user_id_fk
        foreign key (author_id) references auth_user (id),
    constraint proj_comment_proj_post_id_fk
        foreign key (post_id) references proj_post (id)
);

