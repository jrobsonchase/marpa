macro_rules! result_from {
    ( $t1:ident, $t2:ident ) => {
        impl From<$t2> for Result<$t1> {
            fn from(other: $t2) -> Self {
                $t1::new(other)
            }
        }
    };
}
