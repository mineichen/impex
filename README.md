# Impex (short for Implicit/Explicit)

implix lets you annotate struct fields as implicit or explicit using simple Rust macros.
Each field is automatically wrapped in a newtype that carries additional metadata (like origin, visibility, or computation hints) and implements Deref for seamless access.
It solves the usecase to load partially defined data from a json (taking the rest from Default::default), using it and then saving it back without persisting default values.


## Status
This repos is in prototype state. It doesn't use any macros yet. The code in autogenerate section of serde.rs is expected to be in a state where it is trivial to put into a macro later.

Thank you for all the inputs during the impl day at EuroRust in Paris. Feel welcome to participate.


## Pending features (Possible contributions)
- Allow replacing the Wrapper for primitives, so it can contain more Metadata per field, or make the Value reactive (e.g. Leptos)
- Add support for all Structures (PlainEnums, TupleEnums, UnitEnums, PlainStruct, TupleStruct, UnitStruct)
- Write macros
