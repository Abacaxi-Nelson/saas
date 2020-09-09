table! {
    in_teams (id) {
        id -> Nullable<Integer>,
        id_user -> Nullable<Integer>,
        id_team -> Nullable<Integer>,
    }
}

table! {
    teams (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        firstname -> Text,
        lastname -> Text,
        email -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    in_teams,
    teams,
    users,
);
