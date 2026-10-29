//! Types for Knitout operations.

// TODO:
// - Determine allowed increments of stitch and racking values for whether f64
//   is appropriate.
// - Determine grammar of machine needles and how machine-specific.
// - Intern carrier name strings.

/// A Knitout operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Op {
    /// `in CS`: Indicate that the given carrier set should be brought into
    /// action from the yarn grippers when next used.
    In(CarrierSet),
    /// `inhook CS`: Indicate that the given carrier set should be brought into
    /// action using the yarn inserting hook when next used. The inserting hook
    /// will be parked just before the first stitch made with the carriers.
    InHook(CarrierSet),
    /// `releasehook CS`: Release the yarns currently held in the yarn inserting
    /// hook. Must be proceeded by a call to `inhook` with the same carrier set
    /// and at least one knitting operation.
    ReleaseHook(CarrierSet),
    /// `out CS`: Bring a set of carriers out of action by directly moving into
    /// the grippers.
    Out(CarrierSet),
    /// `outhook CS`: Bring a set of carriers out of action, using a yarn
    /// inserting hook to bring their yarns to the grippers.
    OutHook(CarrierSet),
    /// `stitch L T`: Before forming a loop, pull needle by `L` machine units.
    /// After forming a loop, pull needle by `T` machine units.
    Stitch(StitchValue, StitchValue),
    /// `rack R`: Set the offset of the back bed relative to the front bed.
    Rack(RackingValue),
    /// `knit D N CS`: Pull a loop formed in direction `D` by the yarns in
    /// carriers CS through the loops on needle `N`, dropping them in the
    /// process. Knitting with an empty carrier set will drop.
    Knit(Direction, Needle, CarrierSet),
    /// `tuck D N CS`: Add a loop formed in direction `D` by the yarns held by
    /// carriers in `CS` to those already on needle `N`. Tucking with an empty
    /// carrier set will pull on the stitches without doing anything else (an
    /// "a-miss").
    Tuck(Direction, Needle, CarrierSet),
    /// `split D N N2 CS`: Pull a loop formed in direction `D` by the yarns in
    /// carriers `CS` through the loops on needle `N`, transferring the old
    /// loops to opposite-bed needle `N2` in the process. Splitting with an
    /// empty carrier set will transfer.
    Split(Direction, Needle, Needle, CarrierSet),
    /// `drop N`: Synonym for `knit + N`.
    Drop(Needle),
    /// `amiss N`: Synonym for `tuck + N`.
    Amiss(Needle),
    /// `xfer N N2`: Synonym for `split + N N2`.
    Xfer(Needle, Needle),
    /// `miss D N CS`: Move the specified carriers as if they had just formed a
    /// loop in direction `D` at location `N`. (Not generally needed, used when
    /// performing explicit kickbacks or purposeful yarn capture.)
    Miss(Direction, Needle, CarrierSet),
    /// `pause `: Pause the machine until the operator enters a command to
    /// continue. (Not generally needed, but can be useful when, e.g., having
    /// the operator change yarns.)
    Pause,
    /// `x-* *`: Extension opcode.
    Ext(String, String),
}

/// A direction on the machine bed. For example, `+`, `-`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    /// `+` indicates increasing needle number (typically, rightward movement).
    Plus,
    /// `-` indicates decreasing needle number (typically, leftward movement).
    Minus,
}

/// A machine needle. Consists of a location prefix followed by a number. On a
/// V-bed machine, needles are prefixed `f` and `b` for front and back. A V-bed
/// with slide needles adds `fs` and `bs` for front and back sliders. Examples:
/// `f1`, `b-2`, and `fs3`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Needle(String);

/// List of zero or more carrier names. Examples: `A B`, `2`, and ``.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CarrierSet(Vec<Carrier>);

/// A carrier name. Carrier names are strings that do not contain the ` `
/// (space), `,` (comma), or `;` (semicolon) character. Backends may impose
/// further requirements. Examples: `A`, `B`, and `2`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Carrier(String);

/// A floating point number giving the number of machine units to retract a
/// needle. Used with the `stitch` instruction. The meaning of a stitch value
/// may vary between machines and manufacturers; be aware that negative stitch
/// values are supported by at least one manufacturer. Examples: `2`, `-7.5`,
/// and `30`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StitchValue(Float);

/// Number indicating the offset of the front bed relative to the back bed. That
/// is, at racking R, back needle index B is aligned to front needle index B+R.
/// Needles are considered aligned if they can transfer. That is, at racking 2,
/// it is possible to transfer from f3 to b1. May be fractional on some
/// machines. E.g., on Shima machines, 0.25/-0.75 are used for all-needle
/// knitting. Examples: `4`, `5.0`, `2.25`, and `-1.75`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RackingValue(Float);

#[derive(Clone, Copy, Debug)]
pub struct Float(f64);

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Float {}
