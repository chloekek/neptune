/// Component of a packed path drawing instruction.
///
/// The packed instruction encoding consists of a sequence of words.
/// A word is either an opcode or an operand,
/// depending on where it appears in the sequence.
/// Each instruction begins with an opcode followed by a number of operands.
/// How many operands the instruction takes depends on the opcode.
/// The [`Instruction`] type has methods for
/// encoding and decoding packed instructions.
///
/// [`Instruction`]: `crate::path::Instruction`
#[derive(Clone, Copy)]
pub union Word
{
    /// What kind of instruction is encoded.
    pub opcode: Opcode,

    /// An operand to an instruction.
    pub operand: f64,
}

/// What kind of path drawing instruction
/// is encoded in a packed encoding.
#[derive(Clone, Copy)]
#[repr(u64)]
pub enum Opcode
{
    /// The opcode begins a [`Move`] instruction.
    ///
    /// Followed by two operands \\( x \\) and \\( y \\).
    ///
    /// [`Move`]: `crate::path::Instruction::Move`
    Move,

    /// The opcode begins a [`Linear`] instruction.
    ///
    /// Followed by two operands \\( p_{0_x} \\) and \\( p_{0_y} \\).
    ///
    /// [`Linear`]: `crate::path::Instruction::Linear`
    Linear,

    /// The opcode begins a [`Quadratic`] instruction.
    ///
    /// Followed by four operands
    /// \\( p_{0_x} \\), \\( p_{0_y} \\),
    /// \\( p_{1_x} \\), and \\( p_{1_y} \\).
    ///
    /// [`Quadratic`]: `crate::path::Instruction::Quadratic`
    Quadratic,

    /// The opcode begins a [`Cubic`] instruction.
    ///
    /// Followed by six operands
    /// \\( p_{0_x} \\), \\( p_{0_y} \\),
    /// \\( p_{1_x} \\), \\( p_{1_y} \\),
    /// \\( p_{2_x} \\), and \\( p_{2_y} \\).
    ///
    /// [`Cubic`]: `crate::path::Instruction::Cubic`
    Cubic,
}
