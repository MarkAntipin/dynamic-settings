CREATE table settings (
    id serial primary key,
    key text not null,
    value text not null,
    value_type text not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);
CREATE unique index settings_key_unique on settings (key);
