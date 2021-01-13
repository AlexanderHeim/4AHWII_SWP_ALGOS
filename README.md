# 4AHWII_SWP_ALGOS
My repository for algorithm related programming stuff.
I decided to use the Rust-Programming language, as it is one of my favourite languages and is quite nice for programming this kind of stuff.

## Quick overview
The relevant code is located in "/rust/algos/src/".

### main.rs
The main function in main.rs acts as the entry point on execution of the compiled binary.
The main function is most likely used as a temporary testing-tool and not that important.
The remaining code in main.rs is just basic Rust-environment related code, to make the module system aware of it's different modules.

### recursion.rs
Located in this module are very basic implementations of recursive algorithms, such as (and currently only) an algorithm that calculates the fibonacci sequence.

### search.rs
This module contains implementations of various search-algorithms. That's it.

### sort.rs
This module contains implementations of various sorting-algorithms.

### test.rs
"test.rs" contains unit tests, which test the functionality of my implementations. They ensure that my code is working as intended, which is **really** important.
