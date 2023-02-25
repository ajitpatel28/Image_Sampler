use crate::get_non_zero_u32;
use fast_image_resize as fr;
use std::num::{NonZeroU16, NonZeroU32, ParseIntError};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum Size {
    Pixels(NonZeroU32),
    Percent(NonZeroU16),
}

impl Size {
    pub fn calculate_size(&self, src_size: NonZeroU32) -> NonZeroU32 {
        match *self {
            Self::Pixels(size) => size,
            Self::Percent(percent) => get_non_zero_u32(
                (src_size.get() as f32 * percent.get() as f32 / 100.).round() as u32,
            ),
        }
    }
}

impl FromStr for Size {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(percent_str) = s.strip_suffix('%') {
            NonZeroU16::from_str(percent_str).map(Self::Percent)
        } else {
            NonZeroU32::from_str(s).map(Self::Pixels)
        }
    }
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum Algorithm {
    Nearest,
    Convolution,
    SuperSampling,
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum FilterType {



    Box,


    Bilinear,





    Hamming,



    CatmullRom,



    Mitchell,



    Lanczos3,
}

impl From<FilterType> for fr::FilterType {
    fn from(filter_type: FilterType) -> Self {
        match filter_type {
            FilterType::Box => fr::FilterType::Box,
            FilterType::Bilinear => fr::FilterType::Bilinear,
            FilterType::Hamming => fr::FilterType::Hamming,
            FilterType::CatmullRom => fr::FilterType::CatmullRom,
            FilterType::Mitchell => fr::FilterType::Mitchell,
            FilterType::Lanczos3 => fr::FilterType::Lanczos3,
        }
    }
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum ColorSpace {
    Linear,

    NonLinear,
}
