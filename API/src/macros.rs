// macros.rs
use colored::Colorize;
use serde::Deserialize;
use serde::{de, Deserializer};
use std::{fmt, str::FromStr};

#[macro_export]
macro_rules! define_query {
    ($(#[$meta:meta])* $vis:vis struct $struct_name:ident { $($field:ident: $type:ty),* $(,)? }) => {
        $(#[$meta])*
        #[derive(::serde::Deserialize)]
        $vis struct $struct_name {
            $(

                pub $field: $type,
            )*
        }
    };
    ($(#[$meta:meta])* $vis:vis struct $struct_name:ident { $($field:ident: Option<$type:ty>),* $(,)? }) => {
        $(#[$meta])*
        #[derive(::serde::Deserialize)]
        $vis struct $struct_name {
            $(
                #[serde(default, deserialize_with = "crate::macros::ignore_empty")]
                pub $field: Option<$type>,
            )*
        }
    };
}

pub fn ignore_empty<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

pub trait ResultExt<T, E> {
    fn unwrap_or_exit(self, msg: &str) -> T;
}

impl<T, E: std::fmt::Display> ResultExt<T, E> for Result<T, E> {
    fn unwrap_or_exit(self, msg: &str) -> T {
        self.map_err(|e| {
            println!("{}", e.to_string().yellow());
            println!("{}", msg.red());
            std::process::exit(1);
        })
        .unwrap()
    }
}
