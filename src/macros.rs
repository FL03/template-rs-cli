/*
    Appellation: macros <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused)]

macro_rules! impl_display_json {
    (@debug $name:ident) => {
        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str(serde_json::to_string_pretty(self).unwrap().as_str())
            }
        }
    };
    (@display $name:ident) => {
        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str(serde_json::to_string(self).unwrap().as_str())
            }
        }
    };
    (dbg($($name:ident),* $(,)?)) => {
        $(impl_display_json!(@debug $name);)*
    };
    ($($name:ident),* $(,)?) => {
        $(impl_display_json!(@display $name);)*
    };
}
