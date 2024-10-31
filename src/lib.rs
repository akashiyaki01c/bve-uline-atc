#![allow(non_snake_case)]

mod atc;
mod ato;
mod tims;

use ::bveats_rs::*;

ats_main!(crate::atc::uline_atc::ULineATC);

