//! Describes all the possible types of a valid WebAssembly program.

/// Value types classify the individual values that WebAssembly code
/// can compute with and the values that a variable accepts.
///
///     valtype ::= i32 | i64 | f32 | f64
///
/// The types i32 and i64 classify 32 and 64 bit integers,
/// respectively. Integers are not inherently signed or unsigned,
/// their interpretation is determined by individual operations.
///
/// The types f32 and f64 classify 32 and 64 bit floating-point data,
/// respectively. They correspond to the respective binary
/// floating-point representations, also known as single and double
/// precision, as defined by the IEEE 754-2008 standard.
pub enum Value {
    /// 32 bit integers.
    I32,

    /// 64 bit integers.
    I64,

    /// 32 bit floating-point numbers.
    F32,

    /// 64 bit floating-point numbers.
    F64,
}

/// Result type classifies the result of executing instructions or
/// blocks.
///
/// Note: In the current version of WebAssembly, at most one value is
/// allowed as a result. However, this may be generalized to sequences
/// of values in future versions.
pub type Result = [Value; 1];

/// Function type classifies the signature of functions, mapping a
/// vector of parameters to a vector of results.
///
/// Note: In the current version of WebAssembly, the length of the
/// result type vector of a valid function type maybe at most 1. This
/// restriction may be removed in future versions.
pub struct Function<'inputs> {
    /// Inputs, i.e. parameters.
    pub inputs: &'inputs [Value],

    /// Outputs, i.e. results.
    pub outputs: [Value; 1],
}

/// Limits classify the size range of resizeable storage associated
/// with memory type and table type.
///
/// The limits are given in units of page size.
pub struct Limits {
    /// The minimum size.
    pub minimum: u32,

    /// The maximum size. None represents a storage that can grow to
    /// any size.
    pub maximum: Option<u32>,
}

/// Memory type classifies linear memories and their size range.
pub struct Memory {
    /// The size of the memory.
    pub limits: Limits,
}

/// Element types of a table type.
pub enum TableElement {
    /// The element type funcref is the infinite union of all function
    /// types. A table of that type thus contains references to
    /// functions of heterogeneous type.
    FunctionReference,
}

/// Table type classifies tables over elements of element types within
/// a size range.
pub struct Table {
    /// The size of the table.
    pub limits: Limits,

    /// The type of the element inside the table.
    pub elements: TableElement,
}

/// Mutability of a value.
pub enum Mutability {
    /// The value is mutable.
    Mutable,

    /// The value is immutable.
    Immutable,
}

/// Global type classifies global variables, which holds a value and can
/// either be mutable or immutable.
pub struct Global {
    /// Mutability of the value.
    pub mutability: Mutability,

    /// Type of the value.
    pub value: Value,
}

/// External types classify imports and external values with their
/// respective types.
pub enum External<'function_inputs> {
    /// External function.
    Function(Function<'function_inputs>),

    /// External table.
    Table(Table),

    /// External memory.
    Memory(Memory),

    /// External global variable.
    Global(Global),
}
