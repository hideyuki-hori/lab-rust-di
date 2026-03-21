macro_rules! money_type {
    ($name:ident) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            serde::Serialize,
            serde::Deserialize,
            sqlx::Type,
        )]
        #[serde(transparent)]
        #[sqlx(transparent)]
        pub struct $name(pub i64);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl $name {
            pub fn new(value: i64) -> Result<Self, String> {
                if value < 0 {
                    return Err(format!("{} must be non-negative", stringify!($name)));
                }
                Ok(Self(value))
            }
        }
    };
}

pub(crate) use money_type;
