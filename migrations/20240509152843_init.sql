CREATE TYPE policies AS ENUM (
    'ForbidExtern',
    'WarnExtern',
    'AllowRedirects',
    'AllowTrees',
    'ForbidCustomUrl',
    'ForbidUserCreation',
    'EnforceUniqueness',
    'EnforceLogin'
);

CREATE TABLE scopes (
    id serial not null,
    url_prefix varchar(5) not null,
    policies policies[] default('{}'),
    CONSTRAINT scope_key PRIMARY KEY(id),
    CONSTRAINT scope_pre UNIQUE(url_prefix)
);

CREATE TABLE link (
    id serial not null,
    scope_id int not null,
    url varchar(32) not null,
    target text default(null),
    tree_id int default(null),
    warn_extern boolean not null default(false),
    enforce_login boolean not null default(false),
    CONSTRAINT link_key PRIMARY KEY(id),
    FOREIGN KEY (scope_id) REFERENCES scopes (id),
    CONSTRAINT unique_link UNIQUE (id, scope_id)
);


CREATE TABLE entry (
    id serial not null,
    name varchar(128) not null,
    target text not null,
    CONSTRAINT entry_key PRIMARY KEY(id)
);

CREATE TABLE tree (
    id serial not null,
    entry_id int not null,
    CONSTRAINT tree_key PRIMARY KEY(id, entry_id),
    FOREIGN KEY (entry_id) REFERENCES entry(id)
);