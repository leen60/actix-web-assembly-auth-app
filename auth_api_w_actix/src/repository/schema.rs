// @generated automatically by Diesel CLI.

diesel::table! {
    usr (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
