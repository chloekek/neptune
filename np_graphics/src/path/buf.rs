use crate::Vector;
use crate::path::Instruction;
use crate::path::Word;

use std::fmt;
use std::iter::FromIterator;

/// Owned sequence of path drawing instructions.
///
/// The [`Instruction`] type is an enum with variants of different sizes.
/// The smaller variants therefore waste space on padding.
/// [`PathBuf`] stores the instructions as a vector of [`Word`]s,
/// making this type more compact than `Vec<Instruction>`.
pub struct PathBuf
{
    words: Vec<Word>,
}

impl PathBuf
{
    /// Empty path with no path drawing instructions.
    ///
    /// ```
    /// # use np_graphics::PathBuf;
    /// let path = PathBuf::new();
    /// assert!(path.instructions().next().is_none());
    /// ```
    pub fn new() -> Self
    {
        Self{words: Vec::new()}
    }

    /// Append an instruction to the path.
    ///
    /// ```
    /// # use np_graphics::PathBuf;
    /// # use np_graphics::Vector;
    /// # use np_graphics::path::Instruction;
    /// #
    /// let mut path = PathBuf::new();
    ///
    /// let instruction1 = Instruction::Move(Vector{x: 20.0, y: 40.0});
    /// let instruction2 = Instruction::Linear(Vector{x: 40.0, y: 60.0});
    /// path.push(instruction1);
    /// path.push(instruction2);
    ///
    /// let mut instructions = path.instructions();
    /// assert_eq!(instructions.next(), Some(instruction1));
    /// assert_eq!(instructions.next(), Some(instruction2));
    /// assert_eq!(instructions.next(), None);
    /// ```
    pub fn push(&mut self, instruction: Instruction)
    {
        instruction.to_words(&mut self.words);
    }

    /// Append a [`Move`] instruction to the path.
    ///
    /// [`Move`]: `Instruction::Move`
    pub fn push_move(&mut self, to: Vector)
    {
        self.push(Instruction::Move(to));
    }

    /// Append a [`Linear`] instruction to the path.
    ///
    /// [`Linear`]: `Instruction::Linear`
    pub fn push_linear(&mut self, p1: Vector)
    {
        self.push(Instruction::Linear(p1));
    }

    /// Append a [`Quadratic`] instruction to the path.
    ///
    /// [`Quadratic`]: `Instruction::Quadratic`
    pub fn push_quadratic(&mut self, p1: Vector, p2: Vector)
    {
        self.push(Instruction::Quadratic(p1, p2));
    }

    /// Append a [`Cubic`] instruction to the path.
    ///
    /// [`Cubic`]: `Instruction::Cubic`
    pub fn push_cubic(&mut self, p1: Vector, p2: Vector, p3: Vector)
    {
        self.push(Instruction::Cubic(p1, p2, p3));
    }

    /// Iterator over the instructions in the path.
    pub fn instructions(&self) -> Instructions
    {
        // SAFETY: It is an invariant of PathBuf
        // SAFETY: that the sequence of words is valid.
        unsafe { Instructions::from_words_unchecked(&self.words) }
    }

    /// Create a path from a sequence of packed instructions.
    ///
    /// # Safety
    ///
    /// The given sequence must be composed
    /// entirely of valid packed instructions.
    /// See the documentation for [`Word`] for
    /// more information about the packed encoding.
    pub unsafe fn from_words_unchecked(words: Vec<Word>) -> Self
    {
        Self{words}
    }

    /// The packed encoding for the sequence of instructions.
    ///
    /// See the documentation for [`Word`] for
    /// more information about the packed encoding.
    pub fn words(&self) -> &[Word]
    {
        &self.words
    }

    /// The packed encoding for the sequence of instructions.
    ///
    /// See the documentation for [`Word`] for
    /// more information about the packed encoding.
    pub fn into_words(self) -> Vec<Word>
    {
        self.words
    }
}

impl Extend<Instruction> for PathBuf
{
    fn extend<T>(&mut self, iter: T)
        where T: IntoIterator<Item=Instruction>
    {
        for instruction in iter {
            self.push(instruction);
        }
    }
}

impl FromIterator<Instruction> for PathBuf
{
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item=Instruction>
    {
        let mut this = Self::new();
        this.extend(iter);
        this
    }
}

impl fmt::Debug for PathBuf
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        f.debug_list()
            .entries(self.instructions())
            .finish()
    }
}

/// Iterator over path drawing instructions.
pub struct Instructions<'a>
{
    words: &'a [Word],
}

impl<'a> Instructions<'a>
{
    /// Create an iterator from a sequence of packed instructions.
    ///
    /// # Safety
    ///
    /// The given sequence must be composed
    /// entirely of valid packed instructions.
    /// See the documentation for [`Word`] for
    /// more information about the packed encoding.
    pub unsafe fn from_words_unchecked(words: &'a [Word]) -> Self
    {
        Self{words}
    }
}

impl<'a> Iterator for Instructions<'a>
{
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.words.is_empty() {
            None
        } else {
            // SAFETY: It is an invariant of Instructions
            // SAFETY: that the sequence of words is valid.
            Some(unsafe { Instruction::from_words_unchecked(&mut self.words) })
        }
    }
}
