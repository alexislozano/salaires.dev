# salaires.dev

Accessible sur [salaires.dev](https://salaires.dev).

Partagez votre salaire de développeur / développeuse avec la communauté.

Les PRs sont les bienvenues !

### Lancer les tests

```
cargo test
```

### Lancer en local

```
cargo run
```

### Créer un build

```
cargo build
```

## Supabase

Par défaut, salaires.dev se branche sur une db en saas : supabase.

Pour créer les tables et vues dont salaires.dev a besoin, vous pouvez lancer les commandes suivantes dans l'éditeur SQL de supabase:

### Créer la table des salaires

```sql
create table salaries (
    id uuid primary key not null default uuid_generate_v4(),
    email varchar not null,
    location varchar not null,
    company varchar not null,
    title varchar,
    date date not null default now(),
    compensation numeric not null,
    level varchar,
    company_xp int8,
    total_xp int8,
    status varchar not null,
);

alter table salaries enable row level security;

create policy "Enable read access for all users" on "public"."salaries"
as permissive for select
to public
using (true);

create policy "Enable insert access for all users" on "public"."salaries"
as permissive for insert
to public
with check (true);

create policy "Enable update access for all users" ON "public"."salaries"
as permissive for update
to public
using (true)
with check (true);
```

### Créer la table des tokens

```sql
create table tokens (
    salary_id uuid primary key not null,
    token varchar not null unique,
    created_at timestamp not null default now(),
    foreign key (salary_id) references salaries(id) on delete cascade
);

alter table tokens enable row level security;

create policy "Enable read access for all users" on "public"."tokens"
as permissive for select
to public
using (true);

create policy "Enable insert access for all users" on "public"."tokens"
as permissive for insert
to public
with check (true);

create policy "Enable delete access for all users" on "public"."tokens"
as permissive for delete
to public
using (true);
```

### Créer la vue des entreprises

```sql
create view companies as (
    select distinct company from salaries where status='PUBLISHED'
);
```

### Créer la vue des localisations

```sql
create view locations as (
    select distinct location from salaries where status='PUBLISHED'
);
```

### Créer la vue des intitulés de poste

```sql
create view titles as (
    select distinct title from salaries where title is not null and status='PUBLISHED'
);
```
