//! Metadata from source code coverage analysis and instrumentation.

use rustc_index::IndexVec;
use rustc_macros::HashStable;
use rustc_span::{Span, Symbol};

use std::fmt::{self, Debug, Formatter};

rustc_index::newtype_index! {
    /// Used by [`CoverageKind::BlockMarker`] to mark blocks during THIR-to-MIR
    /// lowering, so that those blocks can be identified later.
    #[derive(HashStable)]
    #[encodable]
    #[debug_format = "BlockMarkerId({})"]
    pub struct BlockMarkerId {}
}

rustc_index::newtype_index! {
    /// ID of a coverage counter. Values ascend from 0.
    ///
    /// Before MIR inlining, counter IDs are local to their enclosing function.
    /// After MIR inlining, coverage statements may have been inlined into
    /// another function, so use the statement's source-scope to find which
    /// function/instance its IDs are meaningful for.
    ///
    /// Note that LLVM handles counter IDs as `uint32_t`, so there is no need
    /// to use a larger representation on the Rust side.
    #[derive(HashStable)]
    #[encodable]
    #[orderable]
    #[max = 0xFFFF_FFFF]
    #[debug_format = "CounterId({})"]
    pub struct CounterId {}
}

rustc_index::newtype_index! {
    /// ID of a coverage-counter expression. Values ascend from 0.
    ///
    /// Before MIR inlining, expression IDs are local to their enclosing function.
    /// After MIR inlining, coverage statements may have been inlined into
    /// another function, so use the statement's source-scope to find which
    /// function/instance its IDs are meaningful for.
    ///
    /// Note that LLVM handles expression IDs as `uint32_t`, so there is no need
    /// to use a larger representation on the Rust side.
    #[derive(HashStable)]
    #[encodable]
    #[orderable]
    #[max = 0xFFFF_FFFF]
    #[debug_format = "ExpressionId({})"]
    pub struct ExpressionId {}
}

rustc_index::newtype_index! {
    /// ID of a mcdc condition. Used by llvm to check mcdc coverage.
    ///
    /// Note for future: the max limit of 0xFFFF is probably too loose. Actually llvm does not
    /// support decisions with too many conditions (7 and more at LLVM 18 while may be hundreds at 19)
    /// and represents it with `int16_t`. This max value may be changed once we could
    /// figure out an accurate limit.
    #[derive(HashStable)]
    #[encodable]
    #[orderable]
    #[max = 0xFFFF]
    #[debug_format = "ConditionId({})"]
    pub struct ConditionId {}
}

impl ConditionId {
    pub const NONE: Self = Self::from_u32(0);
}

/// Enum that can hold a constant zero value, the ID of an physical coverage
/// counter, or the ID of a coverage-counter expression.
///
/// This was originally only used for expression operands (and named `Operand`),
/// but the zero/counter/expression distinction is also useful for representing
/// the value of code/gap mappings, and the true/false arms of branch mappings.
#[derive(Copy, Clone, PartialEq, Eq)]
#[derive(TyEncodable, TyDecodable, Hash, HashStable, TypeFoldable, TypeVisitable)]
pub enum CovTerm {
    Zero,
    Counter(CounterId),
    Expression(ExpressionId),
}

impl Debug for CovTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "Zero"),
            Self::Counter(id) => f.debug_tuple("Counter").field(&id.as_u32()).finish(),
            Self::Expression(id) => f.debug_tuple("Expression").field(&id.as_u32()).finish(),
        }
    }
}

#[derive(Clone, PartialEq, TyEncodable, TyDecodable, Hash, HashStable, TypeFoldable, TypeVisitable)]
pub enum CoverageKind {
    /// Marks a span that might otherwise not be represented in MIR, so that
    /// coverage instrumentation can associate it with its enclosing block/BCB.
    ///
    /// Should be erased before codegen (at some point after `InstrumentCoverage`).
    SpanMarker,

    /// Marks its enclosing basic block with an ID that can be referred to by
    /// side data in [`BranchInfo`].
    ///
    /// Should be erased before codegen (at some point after `InstrumentCoverage`).
    BlockMarker { id: BlockMarkerId },

    /// Marks the point in MIR control flow represented by a coverage counter.
    ///
    /// This is eventually lowered to `llvm.instrprof.increment` in LLVM IR.
    ///
    /// If this statement does not survive MIR optimizations, any mappings that
    /// refer to this counter can have those references simplified to zero.
    CounterIncrement { id: CounterId },

    /// Marks the point in MIR control-flow represented by a coverage expression.
    ///
    /// If this statement does not survive MIR optimizations, any mappings that
    /// refer to this expression can have those references simplified to zero.
    ///
    /// (This is only inserted for expression IDs that are directly used by
    /// mappings. Intermediate expressions with no direct mappings are
    /// retained/zeroed based on whether they are transitively used.)
    ExpressionUsed { id: ExpressionId },

    /// Marks the point in MIR control flow represented by a evaluated condition.
    ///
    /// This is eventually lowered to `llvm.instrprof.mcdc.condbitmap.update` in LLVM IR.
    ///
    /// If this statement does not survive MIR optimizations, the condition would never be
    /// taken as evaluated.
    CondBitmapUpdate { id: ConditionId, value: bool, decision_depth: u16 },

    /// Marks the point in MIR control flow represented by a evaluated decision.
    ///
    /// This is eventually lowered to `llvm.instrprof.mcdc.tvbitmap.update` in LLVM IR.
    ///
    /// If this statement does not survive MIR optimizations, the decision would never be
    /// taken as evaluated.
    TestVectorBitmapUpdate { bitmap_idx: u32, decision_depth: u16 },
}

impl Debug for CoverageKind {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        use CoverageKind::*;
        match self {
            SpanMarker => write!(fmt, "SpanMarker"),
            BlockMarker { id } => write!(fmt, "BlockMarker({:?})", id.index()),
            CounterIncrement { id } => write!(fmt, "CounterIncrement({:?})", id.index()),
            ExpressionUsed { id } => write!(fmt, "ExpressionUsed({:?})", id.index()),
            CondBitmapUpdate { id, value, decision_depth } => {
                write!(
                    fmt,
                    "CondBitmapUpdate({:?}, {:?}, depth={:?})",
                    id.index(),
                    value,
                    decision_depth
                )
            }
            TestVectorBitmapUpdate { bitmap_idx, decision_depth } => {
                write!(fmt, "TestVectorUpdate({:?}, depth={:?})", bitmap_idx, decision_depth)
            }
        }
    }
}

#[derive(Clone, TyEncodable, TyDecodable, Hash, HashStable, PartialEq, Eq, PartialOrd, Ord)]
#[derive(TypeFoldable, TypeVisitable)]
pub struct CodeRegion {
    pub file_name: Symbol,
    pub start_line: u32,
    pub start_col: u32,
    pub end_line: u32,
    pub end_col: u32,
}

impl Debug for CodeRegion {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{}:{}:{} - {}:{}",
            self.file_name, self.start_line, self.start_col, self.end_line, self.end_col
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, TyEncodable, TyDecodable, Hash, HashStable)]
#[derive(TypeFoldable, TypeVisitable)]
pub enum Op {
    Subtract,
    Add,
}

impl Op {
    pub fn is_add(&self) -> bool {
        matches!(self, Self::Add)
    }

    pub fn is_subtract(&self) -> bool {
        matches!(self, Self::Subtract)
    }
}

#[derive(Clone, Debug)]
#[derive(TyEncodable, TyDecodable, Hash, HashStable, TypeFoldable, TypeVisitable)]
pub struct Expression {
    pub lhs: CovTerm,
    pub op: Op,
    pub rhs: CovTerm,
}

#[derive(Clone, Debug)]
#[derive(TyEncodable, TyDecodable, Hash, HashStable, TypeFoldable, TypeVisitable)]
pub enum MappingKind {
    /// Associates a normal region of code with a counter/expression/zero.
    Code(CovTerm),
    /// Associates a branch region with separate counters for true and false.
    Branch { true_term: CovTerm, false_term: CovTerm },
    /// Associates a branch region with separate counters for true and false.
    MCDCBranch { true_term: CovTerm, false_term: CovTerm, mcdc_params: ConditionInfo },
    /// Associates a decision region with a bitmap and number of conditions.
    MCDCDecision(DecisionInfo),
}

impl MappingKind {
    /// Iterator over all coverage terms in this mapping kind.
    pub fn terms(&self) -> impl Iterator<Item = CovTerm> {
        let zero = || None.into_iter().chain(None);
        let one = |a| Some(a).into_iter().chain(None);
        let two = |a, b| Some(a).into_iter().chain(Some(b));
        match *self {
            Self::Code(term) => one(term),
            Self::Branch { true_term, false_term } => two(true_term, false_term),
            Self::MCDCBranch { true_term, false_term, .. } => two(true_term, false_term),
            Self::MCDCDecision(_) => zero(),
        }
    }

    /// Returns a copy of this mapping kind, in which all coverage terms have
    /// been replaced with ones returned by the given function.
    pub fn map_terms(&self, map_fn: impl Fn(CovTerm) -> CovTerm) -> Self {
        match *self {
            Self::Code(term) => Self::Code(map_fn(term)),
            Self::Branch { true_term, false_term } => {
                Self::Branch { true_term: map_fn(true_term), false_term: map_fn(false_term) }
            }
            Self::MCDCBranch { true_term, false_term, mcdc_params } => Self::MCDCBranch {
                true_term: map_fn(true_term),
                false_term: map_fn(false_term),
                mcdc_params,
            },
            Self::MCDCDecision(param) => Self::MCDCDecision(param),
        }
    }
}

#[derive(Clone, Debug)]
#[derive(TyEncodable, TyDecodable, Hash, HashStable, TypeFoldable, TypeVisitable)]
pub struct Mapping {
    pub kind: MappingKind,
    pub code_region: CodeRegion,
}

/// Stores per-function coverage information attached to a `mir::Body`,
/// to be used in conjunction with the individual coverage statements injected
/// into the function's basic blocks.
#[derive(Clone, Debug)]
#[derive(TyEncodable, TyDecodable, Hash, HashStable, TypeFoldable, TypeVisitable)]
pub struct FunctionCoverageInfo {
    pub function_source_hash: u64,
    pub num_counters: usize,
    pub mcdc_bitmap_bytes: u32,
    pub expressions: IndexVec<ExpressionId, Expression>,
    pub mappings: Vec<Mapping>,
    /// The depth of the deepest decision is used to know how many
    /// temp condbitmaps should be allocated for the function.
    pub mcdc_max_decision_depth: u16,
}

/// Branch information recorded during THIR-to-MIR lowering, and stored in MIR.
#[derive(Clone, Debug)]
#[derive(TyEncodable, TyDecodable, Hash, HashStable, TypeFoldable, TypeVisitable)]
pub struct BranchInfo {
    /// 1 more than the highest-numbered [`CoverageKind::BlockMarker`] that was
    /// injected into the MIR body. This makes it possible to allocate per-ID
    /// data structures without having to scan the entire body first.
    pub num_block_markers: usize,
    pub branch_spans: Vec<BranchSpan>,
    pub mcdc_branch_spans: Vec<MCDCBranchSpan>,
    pub mcdc_decision_spans: Vec<MCDCDecisionSpan>,
}

#[derive(Clone, Debug)]
#[derive(TyEncodable, TyDecodable, Hash, HashStable, TypeFoldable, TypeVisitable)]
pub struct BranchSpan {
    pub span: Span,
    pub true_marker: BlockMarkerId,
    pub false_marker: BlockMarkerId,
}

#[derive(Copy, Clone, Debug)]
#[derive(TyEncodable, TyDecodable, Hash, HashStable, TypeFoldable, TypeVisitable)]
pub struct ConditionInfo {
    pub condition_id: ConditionId,
    pub true_next_id: ConditionId,
    pub false_next_id: ConditionId,
}

impl Default for ConditionInfo {
    fn default() -> Self {
        Self {
            condition_id: ConditionId::NONE,
            true_next_id: ConditionId::NONE,
            false_next_id: ConditionId::NONE,
        }
    }
}

#[derive(Clone, Debug)]
#[derive(TyEncodable, TyDecodable, Hash, HashStable, TypeFoldable, TypeVisitable)]
pub struct MCDCBranchSpan {
    pub span: Span,
    /// If `None`, this actually represents a normal branch span inserted for
    /// code that was too complex for MC/DC.
    pub condition_info: Option<ConditionInfo>,
    pub true_marker: BlockMarkerId,
    pub false_marker: BlockMarkerId,
    pub decision_depth: u16,
}

#[derive(Copy, Clone, Debug)]
#[derive(TyEncodable, TyDecodable, Hash, HashStable, TypeFoldable, TypeVisitable)]
pub struct DecisionInfo {
    pub bitmap_idx: u32,
    pub conditions_num: u16,
}

#[derive(Clone, Debug)]
#[derive(TyEncodable, TyDecodable, Hash, HashStable, TypeFoldable, TypeVisitable)]
pub struct MCDCDecisionSpan {
    pub span: Span,
    pub conditions_num: usize,
    pub end_markers: Vec<BlockMarkerId>,
    pub decision_depth: u16,
}
