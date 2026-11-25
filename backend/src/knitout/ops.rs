//! Types for [Knitout](https://textiles-lab.github.io/knitout/) operations,
//! specialized for the Shima Seiki WholeGarment SWG 091 N<sub>2</sub>.

use std::fmt;

use enumset::{EnumSet, EnumSetType};

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
    /// `stitch L T`: Before forming a loop, pull needle by `L` machine units
    /// (leading). After forming a loop, pull needle by `T` machine units
    /// (stitch).
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
    /// `pause`: Pause the machine until the operator enters a command to
    /// continue. (Not generally needed, but can be useful when, e.g., having
    /// the operator change yarns.)
    Pause,
    /// `x-speed-number N`: Set the speed number (an abstract index into a
    /// machine table of speed values). Sets speed for both knitting and
    /// transfer passes. Valid speeds are 0 (default) or 1-15.
    Speed(u8),
    /// `x-sub-roller-number N`: Set the sub roller table number (an index into
    /// a machine table of sub roller settings). Sets speed for only knitting
    /// passes. Valid speeds are 0 (default) or 1-31.
    SubRoller(u8),
    /// `x-presser-mode M`: Set the fabric presser mode to `off` (no fabric
    /// presser), `auto` (fabric presser on passes with only front or only back
    /// stitches), or `on` (break passes so that fabric presser can always be
    /// used). The default mode is `off`.
    PresserMode(PresserMode),
    /// `x-loop-start L`: Start loop number `L` (11-41 are valid loop numbers).
    /// Loops may be nested up to two levels.
    LoopStart(u8),
    /// `x-loop-end L`: End loop number `L` (11-41 are valid loop numbers).
    /// Loops may be nested up to two levels.
    LoopEnd(u8),
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Needle {
    pub kind: NeedleKind,
    pub index: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NeedleKind {
    FrontHook,
    BackHook,
    FrontSlider,
    BackSlider,
}

/// List of zero or more carrier names. Examples: `A B`, `2`, and ``.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CarrierSet(EnumSet<Carrier>);

/// A carrier name. Carrier names are strings that do not contain the ` `
/// (space), `,` (comma), or `;` (semicolon) character. Backends may impose
/// further requirements. Examples: `A`, `B`, and `2`. SWG supports `1`, `2`,
/// `3`, `4`, `5`, `6`, `7`, `8`, `9`, and `10`.
#[derive(Debug, EnumSetType)]
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
/// Before forming a loop, pull needle by `L` machine units (leading). After
/// forming a loop, pull needle by `T` machine units (stitch). On SWGN2
/// machines, leading and stitch numbers are stored in an on-machine table, and
/// accessed by an index.
#[expect(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stitch {
    /// `stitch -10 10` (SWGN2 stitch number 81).
    Ln10_S10,
    /// `stitch 0 10` (SWGN2 stitch number 82).
    L0_S10,
    /// `stitch 10 10` (SWGN2 stitch number 83).
    L10_S10,
    /// `stitch 5 15` (SWGN2 stitch number 84).
    L5_S15,
    /// `stitch 10 15` (SWGN2 stitch number 85).
    L10_S15,
    /// `stitch 15 15` (SWGN2 stitch number 86).
    L15_S15,
    /// `stitch 10 20` (SWGN2 stitch number 87).
    L10_S20,
    /// `stitch 15 20` (SWGN2 stitch number 88).
    L15_S20,
    /// `stitch 20 20` (SWGN2 stitch number 89).
    L20_S20,
    /// `stitch 15 25` (SWGN2 stitch number 90).
    L15_S25,
    /// `stitch 20 25` (SWGN2 stitch number 91).
    L20_S25,
    /// `stitch 25 25` (SWGN2 stitch number 92).
    L25_S25,
    /// `stitch 25 30` (SWGN2 stitch number 93).
    L25_S30,
    /// `stitch 25 35` (SWGN2 stitch number 94).
    L25_S35,
    /// `stitch 25 40` (SWGN2 stitch number 95).
    L25_S40,
    /// `stitch 25 45` (SWGN2 stitch number 96).
    L25_S45,
    /// `stitch 25 50` (SWGN2 stitch number 97).
    L25_S50,
    /// `stitch 25 55` (SWGN2 stitch number 98).
    L25_S55,
    /// `stitch 25 60` (SWGN2 stitch number 99).
    L25_S60,
    /// `stitch 25 65` (SWGN2 stitch number 100).
    L25_S65,
    /// `x-stitch-number N`: Sets the stitch number (an abstract index into a
    /// machine table of stitch and leading values). Valid stitch numbers are
    /// 1-120.
    Number(u8),
}

/// Number indicating the offset of the front bed relative to the back bed. That
/// is, at racking R, back needle index B is aligned to front needle index B+R.
/// Needles are considered aligned if they can transfer. That is, at racking 2,
/// it is possible to transfer from f3 to b1. May be fractional on some
/// machines. E.g., on Shima machines, 0.25/-0.75 are used for all-needle
/// knitting. Examples: `4`, `5.0`, `2.25`, and `-1.75`. SWGN2 supports racking
/// values in [-8, 8] that are integers or offset by 0.25 (quarter-pitch), e.g.,
/// `-7.75`, `-0.75`, `0.25`, and `7.25`.
pub type RackingValue = Float;

#[derive(Clone, Copy, Debug)]
pub struct Float(f64);

/// Fabric presser mode. The default mode is `off`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresserMode {
    /// No fabric presser.
    Off,
    /// Fabric presser on passes with only front or only back stitches.
    Auto,
    /// Break passes so that fabric presser can always be
    /// used.
    On,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::In(cs) => write!(f, "in {cs}"),
            Op::InHook(cs) => write!(f, "inhook {cs}"),
            Op::ReleaseHook(cs) => write!(f, "releasehook {cs}"),
            Op::Out(cs) => write!(f, "out {cs}"),
            Op::OutHook(cs) => write!(f, "outhook {cs}"),
            Op::Stitch(stitch) => stitch.fmt(f),
            Op::Rack(r) => write!(f, "rack {r}"),
            Op::Knit(d, n, cs) => write!(f, "knit {d} {n} {cs}"),
            Op::Tuck(d, n, cs) => write!(f, "tuck {d} {n} {cs}"),
            Op::Split(d, n, n2, cs) => write!(f, "split {d} {n} {n2} {cs}"),
            Op::Drop(n) => write!(f, "drop {n}"),
            Op::Amiss(n) => write!(f, "amiss {n}"),
            Op::Xfer(n, n2) => write!(f, "xfer {n} {n2}"),
            Op::Miss(d, n, cs) => write!(f, "miss {d} {n} {cs}"),
            Op::Pause => write!(f, "pause"),
            Op::Speed(n) => write!(f, "x-speed-number {n}"),
            Op::SubRoller(n) => write!(f, "x-sub-roller-number {n}"),
            Op::PresserMode(m) => write!(f, "x-presser-mode {m}"),
            Op::LoopStart(l) => write!(f, "x-loop-start {l}"),
            Op::LoopEnd(l) => write!(f, "x-loop-end {l}"),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Direction::Plus => "+",
            Direction::Minus => "-",
        })
    }
}

impl fmt::Display for Needle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.kind, self.index)
    }
}

impl fmt::Display for NeedleKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            NeedleKind::FrontHook => "f",
            NeedleKind::BackHook => "b",
            NeedleKind::FrontSlider => "fs",
            NeedleKind::BackSlider => "bs",
        })
    }
}

impl fmt::Display for CarrierSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cs = self.0.iter();
        let Some(first) = cs.next() else {
            return f.write_str("empty");
        };
        write!(f, "{first}")?;
        for c in cs {
            write!(f, " {c}")?;
        }
        Ok(())
    }
}

impl fmt::Display for Carrier {
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

impl fmt::Display for Stitch {
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
            Stitch::Number(n) => return write!(f, "x-stitch-number {n}"),
        })
    }
}

impl fmt::Display for PresserMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PresserMode::Off => "off",
            PresserMode::Auto => "auto",
            PresserMode::On => "on",
        })
    }
}

impl<T: Into<EnumSet<Carrier>>> From<T> for CarrierSet {
    fn from(cs: T) -> Self {
        CarrierSet(cs.into())
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Float {}
impl fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Float(value)
    }
}

impl Needle {
    pub fn new(kind: NeedleKind, index: i32) -> Self {
        Needle { kind, index }
    }
}
