// Portable SIMD API is enabled by "simd" feature.
#![cfg_attr(feature = "simd", allow(unstable_features), feature(portable_simd))]

macro_rules! library {
    ($year:tt $description:literal $($day:tt),*) => {
        #[doc = concat!("# ", $description)]
        pub mod $year {
            $(pub mod $day;)*
        }
    }
}

library!(util "Utility modules to handle common recurring Advent of Code patterns."
    ansi, integer, parse
);

library!(year2025 "Finish the North Pole decorations in time for Christmas."
    day01, day02
);
