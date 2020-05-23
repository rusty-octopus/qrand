# Readme

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
* qrand enables `std` interface for iterators and parallel iterators
    * Compile option of qrand_core to dynamically create sequences
    * e.g. "constructor" in two options: `::new(dim:usize)` & `::new(init_data:&[??])`
* Sequential interface "uses" the succeeding value to calculate the next
* Parallel interface does work independently, i.e. one can access each sequence element indepent of each other
* There are infinite and finite sequences, which can be configured
    * Infinite sequence may panic with `collect`
* Sobol and Rd may be built options
    * This means qrand_core could be as two crates
* Use as base for a quasi monte carlo engine (embedded and not)
* Use as base for dithering and other low discrepancy sequence use cases
    * Simple Monte Carlo Integration
    * Dithering
    * Option pricing
    * Use cases in simple git hub repos
* Write an OpenCL lib for both (use Rust as base for compilation and tests, examples)

## Todo

1. Define first simple interfaces & write unit tests
2. Start with Sobol or Rd 2-Dimensional (enables visualization in plots)
3. Find out how to write "pure virtual" interfaces / static factoy pattern in Rust
4. Then initialisation
    * Sobol: polynomials & direction things
    * Rd: alphas, i.e. golden ratios
    * Create as constants into the source code => program code vs. Stack!
    * Consider max dimension although for Rd, e.g. output s.th. during compile time

## Links

### Alternative crates

* [quasirandom](https://crates.io/crates/quasirandom)
* [quasi-rd](https://crates.io/crates/quasi-rd)
* [sobol](https://crates.io/crates/sobol)
* [blue-noise-sampler](https://crates.io/crates/blue-noise-sampler)

### References

* [Roberts - The Unreasonable Effectiveness of Quasirandom Sequences](http://extremelearning.com.au/unreasonable-effectiveness-of-quasirandom-sequences/)
* [Heitz - A Low-Discrepancy Sampler that Distributes Monte Carlo Errors as a Blue Noise in Screen Space](https://eheitzresearch.wordpress.com/762-2/)
* [Monte Carlo and Quasi-Monte Carlo Wiki](http://roth.cs.kuleuven.be/wiki/Main_Page)
* [Wikipedia - Low Discrepancy Sequences](https://en.wikipedia.org/wiki/Low-discrepancy_sequence)
* [Wikipedia - Sobol sequence](https://en.wikipedia.org/wiki/Sobol_sequence)
* [A practical guide to quasi monte carlo methods](https://people.cs.kuleuven.be/~dirk.nuyens/taiwan/)
* [Wikipedia - Plastic number](https://en.wikipedia.org/wiki/Plastic_number)
