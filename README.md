# Impex (short for Implicit/Explicit)

implix lets you annotate struct fields as implicit or explicit using Rust macros.
Each field is automatically wrapped in a newtype that carries additional metadata and implements Deref for seamless access.
It solves the usecase to load partially defined data from a json (taking the rest from Default::default), using it and then saving it back without persisting default values.


## Status
This repos is in prototype state. 

Thank you for all the inputs during the impl day at EuroRust in Paris. Feel welcome to participate.


