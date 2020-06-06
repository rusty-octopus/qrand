# Readme

## Learning goals

* no_std
* Compile time / constant evaluation
* Debugging
* Compile options (sobol, rd, etc.)

## Design

```plantuml
package "qrand_core crate" as qrand_core_crate {
    interface Sequential
    interface Parallel
    component qrand_core
    component sobol
    component rd
}
note left of qrand_core_crate: no_std

package "qrand crate" {
    interface Iterator
    interface ParallelIterator
    component qrand
}

component distribution_converter
note right of distribution_converter: no_std

package "qrand_int crate" as qrand_int_crate {
    component qrand_int
    interface SequentialInteger
    interface ParallelInteger
}
note top of qrand_int_crate: no_std

component quasi_monte_carlo_engine

component quasi_monte_carlo_engine_embedded
note top of quasi_monte_carlo_engine_embedded: no_std

Sequential -- qrand_core
Parallel -- qrand_core

qrand --> Sequential
qrand --> Parallel

qrand_core -- rd
qrand_core -- sobol

Iterator -- qrand
ParallelIterator -- qrand

qrand_int --> Sequential
qrand_int --> Parallel

SequentialInteger -- qrand_int
ParallelInteger -- qrand_int

qrand --> SequentialInteger
qrand --> ParallelInteger

quasi_monte_carlo_engine --> ParallelIterator
quasi_monte_carlo_engine --> Iterator
quasi_monte_carlo_engine --> distribution_converter

quasi_monte_carlo_engine_embedded --> Parallel
quasi_monte_carlo_engine_embedded --> Sequential
quasi_monte_carlo_engine_embedded --> distribution_converter

```

## Design ideas

* qrand_core is `no_std` & embedded
    * Required memory (this means slice for all values in the dimension) needs to be allocated from the caller
* Initialisation of Sobol sequence and Rd sequence via macros and during compile time
    * Init is necessary since especially alpha's for for Rd need to be approximated
    * [Rust - constants](https://doc.rust-lang.org/rust-by-example/custom_types/constants.html)
    * [Rust - Static items](https://doc.rust-lang.org/reference/items/static-items.html)
    * [Rust - Constant items](https://doc.rust-lang.org/reference/items/constant-items.html)
    * [global data in Rust](https://github.com/paulkernfeld/global-data-in-rust)
* qrand enables `std` interface for iterators and parallel iterators
    * Compile option of qrand_core to dynamically create sequences
    * e.g. "constructor" in two options: `::new(dim:usize)` & `::new(init_data:&[??])`
* Sequential interface "uses" the succeeding value to calculate the next
* Parallel interface does work independently, i.e. one can access each sequence element indepent of each other
* There are infinite and finite sequences, which can be configured
    * Infinite sequence may panic with `collect`
* Sobol and Rd may be built options
    * This means qrand_core could be used as two crates
* Sobol initialisation
    * Create a small tool that parses the Joe-Kuo direction numbers and creates a blob with already created polynomials
        * or at least packs the data in binary
    * This file can then easier be used during constant evaluation
        * should be packed into a `'static`, immutable array with exact the size as chosen during compilation
* Use as base for a quasi monte carlo engine (embedded and not)
* Use as base for dithering and other low discrepancy sequence use cases
    * Simple Monte Carlo Integration
    * Dithering
    * Option pricing
    * Use cases in simple git hub repos
* Use cargo fuzzing
* Write an OpenCL lib for both (use Rust as base for compilation and tests, examples)

## Open Items

* Find a way for the Sequential Iterator to work
    * Either add it to qrand crate
    * Or Use two traits that extend Iterator + Sized
    * Or expose two Structs that implement the required iterators
    * For a blanket implementation, a get_dimension API is needed
    * `fn into_iter(len:usize) -> LowDiscrepancyIterator<DimensionIterator<f64>>`
    * Check how `into_iter` resp. `iter` is implemented
* SIMD optimization for Sobol:
    * Execute the XORs with x_n on 4 Direction Numbers in parallel
    * Or create several dimensions in one step parallel
    * Could be a compile time option
* What is faster for Rd?
    * Use f64 for the calculation?
    * Or use u128 and the convert to f64?
    * Is usage of u128 necessary?
* How to calculate sequence with u64
    * It is not clear at the moment how to get the alpha values from the inverse of the golden ratios
    * Document this when you've found out
    * One problem is that we need the most siginificant digits "after" the radix, i.e. conversion to i128 may be necessary
* Can Sobol Seqeunce be completely calculated with u64?
    * Yes they can, according to [sobol.cc](https://web.maths.unsw.edu.au/~fkuo/sobol/sobol.cc)
    * You just need 64 direction nummber per dimension
* Can I create functions that are only used during constant evaluation and not added to the final binary
    * Can I unit test these functions?
* Is it necessary that the output is the one side open interval [0,1)? Or is it possible to have [0,1]?
* Parallelisation: Skip & Leap like in Matlab?

## Todo

1. Define first simple interfaces & write unit tests
    * Rename get_j_th_of_n_th to get_elem_of_dim
2. Find out how to write "pure virtual" interfaces / static factoy pattern in Rust
3. Extract and heavily test own `fract` function
4. Then initialisation
    * Sobol: polynomials & direction things
    * Rd: alphas, i.e. golden ratios
    * Create as constants into the source code => program code vs. Stack!
    * Consider max dimension although for Rd, e.g. output s.th. during compile time
5. Focus on a spike

### Library & executable to create direction numbers

* Library that extracts the values and creates the required direction numbers
* Exports these as u32 (u64?) array
* Write the array as byte array to a file
    * Consider Endianness
* can be used with `include_bytes!` macro

### Benchmarks

* Rd
    * f64 vs. u128 speed & performance
    * also test that the result is closely the "same"
* Sobol calculation vs. Rd calculation
    * I.e. f64 multiplication vs. 32 XORss

### Sobol details

* Direction numbers are required to calculate the points in the sequence
* Number of dimensions equals number of direction numbers
* Direction numbers are derived from primitive polynomials & initialisation values
* Program Memory usage: 32 direction numbers per dimension!

### Postpone

* Rd sequence especially in u64 including generating alphas in integer arithmetic

## Links

### Alternative crates

* [quasirandom](https://crates.io/crates/quasirandom)
* [quasi-rd](https://crates.io/crates/quasi-rd)
* [sobol](https://crates.io/crates/sobol)
* [blue-noise-sampler](https://crates.io/crates/blue-noise-sampler)
* [rand - issue](https://github.com/rust-random/rand/issues/182)

### References

* [Roberts - The Unreasonable Effectiveness of Quasirandom Sequences](http://extremelearning.com.au/unreasonable-effectiveness-of-quasirandom-sequences/)
* [Heitz - A Low-Discrepancy Sampler that Distributes Monte Carlo Errors as a Blue Noise in Screen Space](https://eheitzresearch.wordpress.com/762-2/)
* [Monte Carlo and Quasi-Monte Carlo Wiki](http://roth.cs.kuleuven.be/wiki/Main_Page)
* [Wikipedia - Low Discrepancy Sequences](https://en.wikipedia.org/wiki/Low-discrepancy_sequence)
* [Wikipedia - Sobol sequence](https://en.wikipedia.org/wiki/Sobol_sequence)
* [A practical guide to quasi monte carlo methods](https://people.cs.kuleuven.be/~dirk.nuyens/taiwan/)
* [Wikipedia - Plastic number](https://en.wikipedia.org/wiki/Plastic_number)
* [Savine - Sobol sequence explained](https://medium.com/@antoine_savine/sobol-sequence-explained-188f422b246b)
* [Joe & Kuo - Sobol sequence generator](https://web.maths.unsw.edu.au/~fkuo/sobol/)
* [Joe & Kuo - Notes](https://web.maths.unsw.edu.au/~fkuo/sobol/joe-kuo-notes.pdf)
    > With the implementation of Gray Code, we simply obtain the points in a different order, which still preserve their uniformity properties. It is because every block of 2m points for m=0,1,⋯ is the same as the original implementation.
* [Sobol seqeunce explained](http://deltaquants.com/sobol-sequence-simplified)
* [sobol-test](https://people.sc.fsu.edu/~jburkardt/cpp_src/sobol_test/sobol_test.html)
* [sobol](https://people.sc.fsu.edu/~jburkardt/cpp_src/sobol/sobol.html)
* [sobolseq](https://github.com/stevengj/nlopt/blob/master/src/util/sobolseq.c)
* [Sobol sequence without direction numbers?](https://xilinx.github.io/Vitis_Libraries/quantitative_finance/guide_L1/SobolRsg/sobolrsg.html)
* [Matlab - Sobol](https://de.mathworks.com/help/stats/sobolset.html)
* [Matlab - Quasirandom](https://de.mathworks.com/help/stats/generating-quasi-random-numbers.html)
