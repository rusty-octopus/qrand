# Readme

Workspace for qrand core crates.

## Crates

* Todo

## Design

```plantuml

package "qrand_core crate" as qrand_core {
    interface LowDiscrepancySequence
    interface GetSequence
    interface CreateSequence
    rectangle "impl LowDiscrepancySequence" as impl_lds {
        component rd
        component sobol
        note as feature_note #White
            either feature = rd
            or feature = sobol
        end note
        collections alphas
        collections direction_numbers
    }
    component "build.rs" as build
}

package "qrand_rd_alphas crate" as qrand_rd_alphas {
    interface Create as create_alphas
    component "qrand_rd_alphas" as qrand_rd_alphas_mod
}

package "qrand_sobol_direction_numbers crate" as qrand_sobol_direction_numbers {
    interface Create as create_direction_numbers
    component "qrand_sobol_direction_numbers" as qrand_sobol_direction_numbers_mod
}

package "qrand_std crate" as qrand_std {
    interface "LowDiscrepancySequence" as LowDiscrepancySequence_reexport
    interface NewSequence
    component "qrand_std" as qrand_std_mod
}

actor "no_std User" as no_std_user
actor "std User" as std_user

std_user -down-> NewSequence
no_std_user ---down-> GetSequence

build --down-> create_alphas: feature = rd &\nfeature = no_std
build --down-> create_direction_numbers: feature = sobol &\n feature = no_std


qrand_rd_alphas_mod .up-> create_alphas
qrand_sobol_direction_numbers_mod .up-> create_direction_numbers

qrand_std_mod .up-|> NewSequence
qrand_std_mod .up-|> LowDiscrepancySequence_reexport

impl_lds .up-|> LowDiscrepancySequence
impl_lds .up-|> GetSequence: feature = no_std
impl_lds .up-|> CreateSequence: feature != no_std

rd -down-> alphas : feature = no_std
sobol -down-> direction_numbers : feature = no_std

feature_note .left- rd
feature_note .right- sobol

qrand_std_mod -down-> CreateSequence: NewSequence realized via CreateSequence

qrand_std_mod -down-> create_alphas
qrand_std_mod -down-> create_direction_numbers

build -up-> alphas
build -up-> direction_numbers
```

## Links

* [So you want to write object oriented Rust](https://blog.darrien.dev/posts/so-you-want-to-object/)
