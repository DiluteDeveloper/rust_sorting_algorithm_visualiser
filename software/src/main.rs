pub const VERSION_STRING : &str = env!("CARGO_PKG_VERSION");

mod static_sorts;

use crate::static_sorts::bubble_sort;

fn main() {
    println!("Initializing rust_sorting_algorithm_visualiser {}", VERSION_STRING);

    bubble_sort([3,2,1]);
}