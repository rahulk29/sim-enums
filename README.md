# Simulator Enums

Integrated circuit simulators often allow users to run different types
of analyses (DC analysis, AC analysis, transient analysis, etc.).

Rust APIs for these simulators often have an interface similar to this:

```rust
pub enum AnalysisInput {
    Dc(DcInput),
    Ac(AcInput),
    Tran(TranInput),
}

pub enum AnalysisOutput {
    Dc(DcOutput),
    Ac(AcOutput),
    Tran(TranOutput),
}

impl MySimulator {
    pub fn simulate(&self, inputs: Vec<AnalysisInput>) -> Vec<AnalysisOutput> {
        // ...
    }
}
```

This API presents a few unchecked "promises":
- The output vector should be the same length as the input.
- Element `i` of the output should be the output corresponding to input `i`.

This example crate shows how a more strongly typed API can be layered upon this
basic API. Using the API in this crate, you can do things like this:

```rs
let simulator_output: (AcOutput, TranOutput) = simulator.simulate((AcInput, TranInput));
```

There is no need for users of this API to check correspondence between inputs and outputs;
this checking is done once after getting raw data back from the simulator. If this check
completes successfully, the resulting information is encoded in the Rust type system.
Users do not need to repeatedly assert that their output is of, for example, the `Tran`
variant of the `AnalysisOutput` enum.
