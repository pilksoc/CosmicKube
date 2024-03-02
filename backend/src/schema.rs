// @generated automatically by Diesel CLI.

diesel::table! {
    kube_recipe_lines (recipe_id) {
        recipe_id -> Uuid,
        input_id -> Uuid,
    }
}

diesel::table! {
    kube_recipes (id) {
        id -> Uuid,
        output_id -> Uuid,
    }
}

diesel::table! {
    kubes (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(kube_recipe_lines -> kube_recipes (recipe_id));
diesel::joinable!(kube_recipe_lines -> kubes (input_id));
diesel::joinable!(kube_recipes -> kubes (output_id));

diesel::allow_tables_to_appear_in_same_query!(
    kube_recipe_lines,
    kube_recipes,
    kubes,
);
