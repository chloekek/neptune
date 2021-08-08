use crate::Matrix;
use crate::Vector;
use crate::path::Opcode;
use crate::path::Word;

use std::ops;

/// Transient representation of a path drawing instruction.
///
/// Instructions tell a drawing system how to draw a path.
/// The drawing system follows the instructions in order,
/// constructing a path from the Bézier curves that make it up.
/// Instructions do not store the starting point of each Bézier curve,
/// as the Bézier curves are usually connected end-on-end.
/// The [`Instruction::Move`] variant may be used to disconnect Bézier curves.
///
/// To store a sequence of instructions, use [`PathBuf`].
/// [`PathBuf`] stores the instructions in an efficient packed encoding.
///
/// [`PathBuf`]: `crate::PathBuf`
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instruction
{
    /// Set \\( p_0 \\) of the next Bézier curve to the given point.
    Move(Vector),

    /// Draw a first-order Bézier curve
    /// with the given point \\( p_1 \\).
    Linear(Vector),

    /// Draw a second-order Bézier curve
    /// with the given points \\( p_1 \\) and \\( p_2 \\).
    Quadratic(Vector, Vector),

    /// Draw a third-order Bézier curve
    /// with the given points \\( p_1 \\),
    /// \\( p_2 \\), and \\( p_3 \\).
    Cubic(Vector, Vector, Vector),
}

impl Instruction
{
    /// Read an instruction from its packed encoding.
    ///
    /// The sequence is resliced when this function returns.
    /// It will begin at the word that immediately follows
    /// the final word of the first instruction.
    ///
    /// # Safety
    ///
    /// The given sequence must begin with
    /// a valid packed instruction.
    /// See the documentation for [`Word`] for
    /// more information about the packed encoding.
    pub unsafe fn from_words_unchecked(words: &mut &[Word]) -> Self
    {
        unsafe fn read_opcode(words: &mut &[Word]) -> Opcode
        {
            let opcode = words.get_unchecked(0).opcode;
            *words = words.get_unchecked(1 ..);
            opcode
        }

        unsafe fn read_operand(words: &mut &[Word]) -> f64
        {
            let operand = words.get_unchecked(0).operand;
            *words = words.get_unchecked(1 ..);
            operand
        }

        let opcode = read_opcode(words);

        match opcode {
            Opcode::Move => {
                let x = read_operand(words);
                let y = read_operand(words);
                Self::Move(Vector{x, y})
            },
            Opcode::Linear => {
                let p1_x = read_operand(words);
                let p1_y = read_operand(words);
                let p1 = Vector{x: p1_x, y: p1_y};
                Self::Linear(p1)
            },
            Opcode::Quadratic => {
                let p1_x = read_operand(words);
                let p1_y = read_operand(words);
                let p2_x = read_operand(words);
                let p2_y = read_operand(words);
                let p1 = Vector{x: p1_x, y: p1_y};
                let p2 = Vector{x: p2_x, y: p2_y};
                Self::Quadratic(p1, p2)
            },
            Opcode::Cubic => {
                let p1_x = read_operand(words);
                let p1_y = read_operand(words);
                let p2_x = read_operand(words);
                let p2_y = read_operand(words);
                let p3_x = read_operand(words);
                let p3_y = read_operand(words);
                let p1 = Vector{x: p1_x, y: p1_y};
                let p2 = Vector{x: p2_x, y: p2_y};
                let p3 = Vector{x: p3_x, y: p3_y};
                Self::Cubic(p1, p2, p3)
            },
        }
    }

    /// Write the packed encoding for the instruction to a buffer.
    ///
    /// The written sequence of words will
    /// always form a valid packed instruction.
    /// See the documentation for [`Word`] for
    /// more information about the packed encoding.
    pub fn to_words<T>(&self, buf: &mut T)
        where T: Extend<Word>
    {
        match *self {
            Self::Move(to) => {
                let words = [
                    Word{opcode: Opcode::Move},
                    Word{operand: to.x},
                    Word{operand: to.y},
                ];
                buf.extend(words.iter().copied());
            },
            Self::Linear(p1) => {
                let words = [
                    Word{opcode: Opcode::Linear},
                    Word{operand: p1.x},
                    Word{operand: p1.y},
                ];
                buf.extend(words.iter().copied());
            },
            Self::Quadratic(p1, p2) => {
                let words = [
                    Word{opcode: Opcode::Quadratic},
                    Word{operand: p1.x},
                    Word{operand: p1.y},
                    Word{operand: p2.x},
                    Word{operand: p2.y},
                ];
                buf.extend(words.iter().copied());
            },
            Self::Cubic(p1, p2, p3) => {
                let words = [
                    Word{opcode: Opcode::Cubic},
                    Word{operand: p1.x},
                    Word{operand: p1.y},
                    Word{operand: p2.x},
                    Word{operand: p2.y},
                    Word{operand: p3.x},
                    Word{operand: p3.y},
                ];
                buf.extend(words.iter().copied());
            },
        }
    }
}

impl ops::Mul<Instruction> for Matrix
{
    type Output = Instruction;

    fn mul(self, rhs: Instruction) -> Self::Output
    {
        match rhs {
            Instruction::Move(to) =>
                Instruction::Move(self * to),
            Instruction::Linear(p1) =>
                Instruction::Linear(self * p1),
            Instruction::Quadratic(p1, p2) =>
                Instruction::Quadratic(self * p1, self * p2),
            Instruction::Cubic(p1, p2, p3) =>
                Instruction::Cubic(self * p1, self * p2, self * p3),
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_instruction_words_roundtrip()
    {
        let p1 = Vector{x:  50.0, y: 100.0};
        let p2 = Vector{x: 150.0, y: 200.0};
        let p3 = Vector{x: 250.0, y: 300.0};

        let instructions = [
            Instruction::Move(p1),
            Instruction::Linear(p1),
            Instruction::Quadratic(p1, p2),
            Instruction::Cubic(p1, p2, p3),
        ];

        for &instruction in &instructions {

            let mut words = Vec::new();
            instruction.to_words(&mut words);
            words.push(Word{operand: 9999.0});

            let mut stream = &words[..];
            let roundtripped = unsafe {
                Instruction::from_words_unchecked(&mut stream)
            };
            assert_eq!(instruction, roundtripped);
            assert_eq!(stream.len(), 1);

        }
    }
}
