// @generated automatically by Diesel CLI.

diesel::table! {
    people (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    quotes (id) {
        id -> Integer,
        person_id -> Integer,
        content -> Text,
    }
}

diesel::joinable!(quotes -> people (person_id));

diesel::allow_tables_to_appear_in_same_query!(
    people,
    quotes,
);
