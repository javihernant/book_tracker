// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        mail -> Text,
        api_token -> Nullable<Text>,
    }
}
