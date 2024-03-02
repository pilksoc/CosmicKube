-- Your SQL goes here

CREATE TABLE kubes (
  id uuid primary key,
  name varchar(255) not null
);

CREATE INDEX k_i on kubes(name);

CREATE TABLE kube_recipes (
  id uuid primary key,
  output_id uuid not null references kubes(id)
);

CREATE TABLE kube_recipe_lines (
  recipe_id uuid primary key references kube_recipes(id),
  input_id uuid not null references kubes(id)
);

CREATE INDEX krl_i ON kube_recipe_lines(recipe_id, input_id);
