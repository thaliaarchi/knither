use std::fmt;

use enumset::EnumSet;

use crate::knitout::{Carrier, CarrierSet, Direction, Needle, NeedleKind, Op, PresserMode, Stitch};

// Notes:
// - Needle numbering:
//   - Needles are numbered relative to an initial origin that cannot be changed
//     mid-design.
//   - Racking shifts the back bed to change which needles are aligned, but does
//     not renumber them. Thus the Knitout producer must keep track of the
//     current racking value.

// TODO:
// - There's a lot of repetitive codegen. I should make an abstraction for
//   loops.

pub struct Builder {
    /// Map from carrier number to yarn header string.
    yarns: Vec<(Carrier, String)>,
    carriers: EnumSet<Carrier>,
    ops: Vec<Op>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeftRight {
    Left,
    Right,
}

impl Builder {
    pub fn new(yarns: Vec<(Carrier, String)>) -> Self {
        let mut carriers = EnumSet::new();
        for (carrier, _) in &yarns {
            carriers.insert(*carrier);
        }
        assert_eq!(carriers.len(), yarns.len());
        Builder {
            yarns,
            carriers,
            ops: Vec::new(),
        }
    }

    pub fn cast_on(&mut self, carrier: Carrier, width: i32) {
        assert!(self.carriers.contains(carrier));
        self.shima_waste_yarn_cast_on(carrier, width);
        self.waste_yarn_tube(carrier, width);
        self.draw_thread(Carrier::C1, width);
        self.french_welt_cast_on(carrier, width);
        // self.emit(Op::OutHook(1));
        self.emit(Op::PresserMode(PresserMode::Auto));
    }

    pub fn shima_waste_yarn_cast_on(&mut self, carrier: Carrier, width: i32) {
        assert!(self.carriers.contains(carrier));
        assert!(width > 2 && width % 2 == 0);

        // Shima waste yarn cast on:
        self.emit(Op::Stitch(Stitch::Number(50)));
        self.emit(Op::InHook(carrier.into()));
        // Yarn insertion needs to start with front knit.
        self.emit(Op::Knit(
            Direction::Minus,
            Needle::new(NeedleKind::FrontHook, width - 2),
            carrier.into(),
        ));
        self.emit(Op::Stitch(Stitch::Number(53)));
        for n in (0..width).step_by(2).rev() {
            self.emit(Op::Knit(
                Direction::Minus,
                Needle::new(NeedleKind::BackHook, n + 1),
                carrier.into(),
            ));
            self.emit(Op::Knit(
                Direction::Minus,
                Needle::new(NeedleKind::FrontHook, n),
                carrier.into(),
            ));
        }
        self.emit(Op::Stitch(Stitch::Number(54)));
        self.emit(Op::PresserMode(PresserMode::On));
        for n in (0..width).step_by(2) {
            self.emit(Op::Miss(
                Direction::Plus,
                Needle::new(NeedleKind::BackHook, n),
                carrier.into(),
            ));
            self.emit(Op::Knit(
                Direction::Plus,
                Needle::new(NeedleKind::BackHook, n + 1),
                carrier.into(),
            ));
        }
        self.emit(Op::Stitch(Stitch::Number(55)));
        for n in (0..width).step_by(2).rev() {
            self.emit(Op::Knit(
                Direction::Minus,
                Needle::new(NeedleKind::BackHook, n + 1),
                carrier.into(),
            ));
            self.emit(Op::Miss(
                Direction::Minus,
                Needle::new(NeedleKind::FrontHook, n),
                carrier.into(),
            ));
        }
        self.emit(Op::Stitch(Stitch::Number(56)));
        self.emit(Op::PresserMode(PresserMode::Off));

        for n in (0..width).step_by(2) {
            self.emit(Op::Knit(
                Direction::Plus,
                Needle::new(NeedleKind::FrontHook, n),
                carrier.into(),
            ));
        }
        self.emit(Op::Miss(
            Direction::Plus,
            Needle::new(NeedleKind::FrontHook, width - 1),
            carrier.into(),
        ));

        self.emit(Op::ReleaseHook(carrier.into()));
        self.emit(Op::Stitch(Stitch::Number(22)));
        self.emit(Op::PresserMode(PresserMode::On));
        for n in (0..width).step_by(2).rev() {
            self.emit(Op::Knit(
                Direction::Minus,
                Needle::new(NeedleKind::BackHook, n + 1),
                carrier.into(),
            ));
            self.emit(Op::Miss(
                Direction::Minus,
                Needle::new(NeedleKind::FrontHook, n),
                carrier.into(),
            ));
        }
        self.emit(Op::PresserMode(PresserMode::Off));
        for n in (0..width).step_by(2) {
            self.emit(Op::Knit(
                Direction::Plus,
                Needle::new(NeedleKind::FrontHook, n),
                carrier.into(),
            ));
            self.emit(Op::Miss(
                Direction::Plus,
                Needle::new(NeedleKind::BackHook, n + 1),
                carrier.into(),
            ));
        }
    }

    pub fn waste_yarn_tube(&mut self, carrier: Carrier, width: i32) {
        assert!(self.carriers.contains(carrier));
        self.tube(carrier, width, 20, LeftRight::Right, true);
        self.emit(Op::Rack(0.0.into()));
        for n in 0..width {
            self.emit(Op::Xfer(
                Needle::new(NeedleKind::BackHook, n),
                Needle::new(NeedleKind::FrontHook, n),
            ));
        }
        for n in (0..width).rev() {
            self.emit(Op::Knit(
                Direction::Minus,
                Needle::new(NeedleKind::FrontHook, n),
                carrier.into(),
            ));
        }
        for n in 0..width {
            self.emit(Op::Knit(
                Direction::Plus,
                Needle::new(NeedleKind::FrontHook, n),
                carrier.into(),
            ));
        }
        self.emit(Op::Miss(
            Direction::Plus,
            Needle::new(NeedleKind::FrontHook, width),
            carrier.into(),
        ));
        self.emit(Op::Rack(0.25.into()));
    }

    /// Insert the nylon draw thread separating the waste yarn from the piece.
    pub fn draw_thread(&mut self, carrier: Carrier, width: i32) {
        assert!(self.carriers.contains(carrier));
        self.emit(Op::InHook(carrier.into()));
        self.emit(Op::Knit(
            Direction::Minus,
            Needle::new(NeedleKind::FrontHook, width),
            carrier.into(),
        ));
        self.emit(Op::Knit(
            Direction::Plus,
            Needle::new(NeedleKind::FrontHook, width - 1),
            carrier.into(),
        ));
        for n in (0..width - 1).rev() {
            self.emit(Op::Knit(
                Direction::Minus,
                Needle::new(NeedleKind::FrontHook, n),
                carrier.into(),
            ));
        }
        for n in 0..width {
            self.emit(Op::Knit(
                Direction::Plus,
                Needle::new(NeedleKind::FrontHook, n),
                carrier.into(),
            ));
        }
        self.emit(Op::Miss(
            Direction::Plus,
            Needle::new(NeedleKind::FrontHook, width),
            carrier.into(),
        ));
        self.emit(Op::ReleaseHook(carrier.into()));
        self.emit(Op::Drop(Needle::new(NeedleKind::FrontHook, width)));
    }

    pub fn french_welt_cast_on(&mut self, carrier: Carrier, width: i32) {
        assert!(self.carriers.contains(carrier));
        self.emit(Op::Stitch(Stitch::Number(101)));
        self.emit(Op::Rack(0.25.into()));
        for n in (0..width).rev() {
            self.emit(Op::Knit(
                Direction::Minus,
                Needle::new(NeedleKind::BackHook, n),
                carrier.into(),
            ));
            self.emit(Op::Knit(
                Direction::Minus,
                Needle::new(NeedleKind::FrontHook, n),
                carrier.into(),
            ));
        }
        self.emit(Op::Stitch(Stitch::Number(103)));
        self.emit(Op::Miss(
            Direction::Plus,
            Needle::new(NeedleKind::FrontHook, 0),
            carrier.into(),
        ));
        for n in 1..width {
            self.emit(Op::Knit(
                Direction::Plus,
                Needle::new(NeedleKind::FrontHook, n),
                carrier.into(),
            ))
        }
        for n in (0..width).rev() {
            self.emit(Op::Knit(
                Direction::Minus,
                Needle::new(NeedleKind::BackHook, n),
                carrier.into(),
            ))
        }
        for n in 0..width {
            self.emit(Op::Knit(
                Direction::Plus,
                Needle::new(NeedleKind::FrontHook, n),
                carrier.into(),
            ))
        }
        self.emit(Op::Miss(
            Direction::Plus,
            Needle::new(NeedleKind::FrontHook, width),
            carrier.into(),
        ));
    }

    pub fn tube(
        &mut self,
        carrier: Carrier,
        width: i32,
        rows: i32,
        start_edge: LeftRight,
        clockwise: bool,
    ) {
        assert!(self.carriers.contains(carrier));
        let left_going = if clockwise {
            NeedleKind::FrontHook
        } else {
            NeedleKind::BackHook
        };
        let right_going = if clockwise {
            NeedleKind::BackHook
        } else {
            NeedleKind::FrontHook
        };
        for _ in 0..rows {
            if start_edge == LeftRight::Right {
                for n in (0..width).rev() {
                    self.emit(Op::Knit(
                        Direction::Minus,
                        Needle::new(left_going, n),
                        carrier.into(),
                    ));
                }
            }
            for n in 0..width {
                self.emit(Op::Knit(
                    Direction::Plus,
                    Needle::new(right_going, n),
                    carrier.into(),
                ));
            }
            if start_edge == LeftRight::Left {
                for n in (0..width).rev() {
                    self.emit(Op::Knit(
                        Direction::Minus,
                        Needle::new(left_going, n),
                        carrier.into(),
                    ));
                }
            }
        }
    }

    fn emit(&mut self, op: Op) {
        self.ops.push(op);
    }
}

impl fmt::Display for Builder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, ";!knitout-2")?;
        writeln!(f, ";;Carriers: {}", CarrierSet::from(EnumSet::all()))?;
        for (carrier, yarn) in &self.yarns {
            writeln!(f, ";;Yarn-{carrier}: {yarn}")?;
        }
        for op in &self.ops {
            writeln!(f, "{op}")?;
        }
        Ok(())
    }
}
