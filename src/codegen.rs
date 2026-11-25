use crate::knitout::{Carrier, Direction, Needle, NeedleKind, Op, PresserMode, Stitch};

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
    ops: Vec<Op>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeftRight {
    Left,
    Right,
}

impl Builder {
    pub fn shima_waste_yarn_cast_on(&mut self, carrier: Carrier, width: i32) {
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
        self.emit(Op::Knit(
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

    pub fn tube(
        &mut self,
        carrier: Carrier,
        width: i32,
        rows: i32,
        start_edge: LeftRight,
        clockwise: bool,
    ) {
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
