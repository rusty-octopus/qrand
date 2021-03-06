# Todos

0. Cargo [env] usage?
    * [Add support for [env] section in .cargo/config.toml](https://github.com/rust-lang/cargo/pull/9175)
1. Review code
    * [https://rust-unofficial.github.io/patterns/intro.html]
    * NewType?
    * COnstructor?
2. Refactor/Restructure: More crates (not all of the hosted on crates.io, just the main ones?)
3. Create solution with RD + Build script + embedded feature
    * Emit size in bytes during execution
    * Rename `std_interface` feature to `std` only?
    * Extract helper functions in new crate qrand_build_utils & test
    * Extract rd creation in new crate & test
    * Emit compile error if rd and sobol is defined
        * See [Advanced Cargo Features](https://blog.turbo.fish/cargo-features/)
4. Update #[cfg] usage to have real either Rd or Sobol with following pattern
    * #[cfg(all(feature = "feat1", not(feature = "feat2")))]
    * See [Advanced Cargo Features](https://blog.turbo.fish/cargo-features/)
5. qrand_std
    * Maybe use [Renaming dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#renaming-dependencies-in-cargotoml) to include Rd and Sobol and maybe a new interface that enables the creation of all sequences (guarded by a feature?)
6. qrand_core examples?
7. freestanding (#[no_main]) real embedded example, possibly in a new crate (since we need some output in the form of defmt or similar)
    * [freestanding rust binary](https://os.phil-opp.com/freestanding-rust-binary/)
8. Extract and heavily test own `fract` function
    * Still necessary?
9. Then initialisation
    * Sobol: polynomials & direction things
    * Rd: alphas, i.e. golden ratios
    * Create as constants into the source code => program code vs. Stack!
    * Consider max dimension although for Rd, e.g. output s.th. during compile time
10. Rework Readme.md

## Library & executable to create direction numbers

* Library that extracts the values and creates the required direction numbers
* Exports these as u32 (u64?) array
* Write the array as byte array to a file
    * Consider Endianness
* can be used with `include_bytes!` macro

## Benchmarks

* Rd
    * f64 vs. u128 speed & performance
    * also test that the result is closely the "same"
* Sobol calculation vs. Rd calculation
    * I.e. f64 multiplication vs. 32 XORss
