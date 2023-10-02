#![allow(non_snake_case)]
#![feature(once_cell)]

mod api;
mod ats;
mod voice;
use crate::api::*;

ats_main!(ats::KobeCitySubwayATS);

