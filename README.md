# radamsa-rs

This project is wrapper for Rust around `libradamsa` exposed by the [radamsa](https://gitlab.com/akihe/radamsa/) project. 

## API

The API is notably different from the `radamsa` C API in a few ways:
- It does not require an `init` function to be called.
- It renames the `radamsa` function to `generate`.
- It renames the `radamsa_inplace` function to `mutate`.
- It makes seeds optional by establishing a global atomic counter that increments.

Other than that go take a look at the [docs](https://docs.rs/radamsa).

I used this project as a means to learn some rust packaging. I am very open to PRs and Issues.