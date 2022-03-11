table! {
    weather (id) {
        id -> Int4,
        temperature_c -> Float8,
        temperature_f -> Float8,
        humidity -> Float8,
        pressure -> Float8,
        date -> Timestamptz,
    }
}
