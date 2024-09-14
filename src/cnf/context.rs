/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused)]

use super::Settings;
use std::sync::Arc;

pub struct Context {
    settings: Arc<Settings>,
}
