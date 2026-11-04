//! Types for Knitout operations.

// TODO:
// - Determine allowed increments of stitch and racking values for whether f64
//   is appropriate.
// - Determine grammar of machine needles and how machine-specific.
// - Intern carrier name strings.

use core::fmt;

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
    Stitch(Stitch),
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
/// further requirements. Examples: `A`, `B`, and `2`. SWG supports `1`, `2`,
/// `3`, `4`, `5`, `6`, `7`, `8`, `9`, and `10`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Carrier {
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
}

/// A `stitch L T` operation giving two machine units to retract a needle.
/// Before forming a loop, pull needle by `L` machine units. After forming a
/// loop, pull needle by `T` machine units.
#[expect(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Stitch {
    /// `stitch -10 10`
    Ln10_S10,
    /// `stitch 0 10`
    L0_S10,
    /// `stitch 10 10`
    L10_S10,
    /// `stitch 5 15`
    L5_S15,
    /// `stitch 10 15`
    L10_S15,
    /// `stitch 15 15`
    L15_S15,
    /// `stitch 10 20`
    L10_S20,
    /// `stitch 15 20`
    L15_S20,
    /// `stitch 20 20`
    L20_S20,
    /// `stitch 15 25`
    L15_S25,
    /// `stitch 20 25`
    L20_S25,
    /// `stitch 25 25`
    L25_S25,
    /// `stitch 25 30`
    L25_S30,
    /// `stitch 25 35`
    L25_S35,
    /// `stitch 25 40`
    L25_S40,
    /// `stitch 25 45`
    L25_S45,
    /// `stitch 25 50`
    L25_S50,
    /// `stitch 25 55`
    L25_S55,
    /// `stitch 25 60`
    L25_S60,
    /// `stitch 25 65`
    L25_S65,
}

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

impl fmt::Debug for Carrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Carrier::C1 => "1",
            Carrier::C2 => "2",
            Carrier::C3 => "3",
            Carrier::C4 => "4",
            Carrier::C5 => "5",
            Carrier::C6 => "6",
            Carrier::C7 => "7",
            Carrier::C8 => "8",
            Carrier::C9 => "9",
            Carrier::C10 => "10",
        })
    }
}

impl fmt::Debug for Stitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Stitch::Ln10_S10 => "stitch -10 10",
            Stitch::L0_S10 => "stitch 0 10",
            Stitch::L10_S10 => "stitch 10 10",
            Stitch::L5_S15 => "stitch 5 15",
            Stitch::L10_S15 => "stitch 10 15",
            Stitch::L15_S15 => "stitch 15 15",
            Stitch::L10_S20 => "stitch 10 20",
            Stitch::L15_S20 => "stitch 15 20",
            Stitch::L20_S20 => "stitch 20 20",
            Stitch::L15_S25 => "stitch 15 25",
            Stitch::L20_S25 => "stitch 20 25",
            Stitch::L25_S25 => "stitch 25 25",
            Stitch::L25_S30 => "stitch 25 30",
            Stitch::L25_S35 => "stitch 25 35",
            Stitch::L25_S40 => "stitch 25 40",
            Stitch::L25_S45 => "stitch 25 45",
            Stitch::L25_S50 => "stitch 25 50",
            Stitch::L25_S55 => "stitch 25 55",
            Stitch::L25_S60 => "stitch 25 60",
            Stitch::L25_S65 => "stitch 25 65",
        })
    }
}
