macro_rules! query_params {
    { $( $field:ident = $value:expr ),* $(,)? } => {{
        &[
            $( (stringify!($field), &$value) ),*
        ]
    }};
}
pub(crate) use query_params;

macro_rules! request_body {
    [] => {
        None::<()>
    };
    ( $value:expr $(,)? ) => {
        Some($value)
    };
    { $( $field:ident: $type:ty = $value:expr ),+ $(,)? } => {{
        #[allow(non_snake_case)]
        #[derive(::serde_derive::Serialize)]
        struct Request { $( $field: $type ),* }
        #[allow(clippy::redundant_field_names)]
        Some(Request { $( $field: $value ),* })
    }};
}
pub(crate) use request_body;

macro_rules! response_body {
    () => {
        |_: ::serde_json::Value| ()
    };
    (..) => {
        |response| response
    };
    { $( $field:ident: $type:ty ),+ $(,)? } => {{
        #[allow(non_snake_case)]
        #[derive(::serde_derive::Deserialize)]
        struct Response { $( $field: $type ),* }
        |response: Response| ($( response.$field ),*)
    }};
}
pub(crate) use response_body;
