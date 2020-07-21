#[macro_export]
macro_rules! create_host_param {
    ({ $($field:ident -> $e:expr)*  }) => {
        $crate::host::CreateHostParam {
            $($field: $e,)*
            ..Default::default()
        }
    };
}
