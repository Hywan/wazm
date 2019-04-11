//! WebAssembly code consists of sequences of instructions. Its
//! computational model is based on a stack machine in that
//! instructions manipulate values on an implicit operand stack,
//! consuming (popping) argument values and producing or returning
//! (pushing) result values.

/// Numeric instructions.
#[allow(missing_docs)]
pub enum Numeric {
    // Constants.
    I32Const(i32),
    I64Const(i64),
    F32Const(u32),
    F64Const(u64),

    // Unary operators.
    I32Clz,
    I64Clz,

    I32Ctz,
    I64Ctz,

    I32Popcnt,
    I64Popcnt,

    F32Abs,
    F64Abs,

    F32Neg,
    F64Neg,

    F32Sqrt,
    F64Sqrt,

    F32Ceil,
    F64Ceil,

    F32Floor,
    F64Floor,

    F32Trunc,
    F64Trunc,

    F32Nearest,
    F64Nearest,

    // Binary operators.
    I32Add,
    I64Add,
    F32Add,
    F64Add,

    I32Sub,
    I64Sub,
    F32Sub,
    F64Sub,

    I32Mul,
    I64Mul,
    F32Mul,
    F64Mul,

    I32DivU,
    I64DivU,

    I32DivS,
    I64DivS,

    F32Div,
    F64Div,

    I32RemU,
    I64RemU,

    I32RemS,
    I64RemS,

    I32And,
    I64And,

    I32Or,
    I64Or,

    I32Xor,
    I64Xor,

    I32Shl,
    I64Shl,

    I32ShrU,
    I64ShrU,

    I32ShrS,
    I64ShrS,

    I32Rotl,
    I64Rotl,

    I32Rotr,
    I64Rotr,

    F32Min,
    F64Min,

    F32Max,
    F64Max,

    F32CopySign,
    F64CopySign,

    // Test operators.
    I32Eqz,
    I64Eqz,

    // Comparison operators.
    I32Eq,
    I64Eq,

    I32Ne,
    I64Ne,

    I32LtU,
    I32LtS,
    I64LtU,
    I64LtS,

    I32GtU,
    I32GtS,
    I64GtU,
    I64GtS,

    I32LeU,
    I32LeS,
    I64LeU,
    I64LeS,

    I32GeU,
    I32GeS,
    I64GeU,
    I64GeS,

    F32Eq,
    F64Eq,

    F32Ne,
    F64Ne,

    F32Lt,
    F64Lt,

    F32Gt,
    F64Gt,

    F32Le,
    F64Le,

    F32Ge,
    F64Ge,

    // Conversion operators.
    I32WrapI64,
    I64WrapI32U,
    I64WrapI32S,

    I32TruncF32U,
    I32TruncF32S,
    I32TruncF64U,
    I32TruncF64S,
    I64TruncF32U,
    I64TruncF32S,
    I64TruncF64U,
    I64TruncF64S,

    F32DemoteF64,
    F64PromoteF32,

    F32ConvertI32U,
    F32ConvertI32S,
    F32ConvertI64U,
    F32ConvertI64S,
    F64ConvertI32U,
    F64ConvertI32S,
    F64ConvertI64U,
    F64ConvertI64S,

    I32ReinterpretF32,
    I32ReinterpretF64,
    I64ReinterpretF32,
    I64ReinterpretF64,

    F32ReinterpretI32,
    F32ReinterpretI64,
    F64ReinterpretI32,
    F64ReinterpretI64,
}

/// All instructions, grouped by types.
pub enum Instructions {
    /// Numeric instructions.
    Numeric(Numeric),
}
