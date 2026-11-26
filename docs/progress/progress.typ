#title[Knither: Midterm Progress Report]

Jonas Regehr and Thalia Archibald

= Introduction 
With Knither, we aim to build knitout abstractions and an image processing pipeline to automate the creation of 2D knit swatches from raster images. 

Our original midterm goal was to complete bitonal image processing and fabricate a teapot or similar image.


= Progress
== Code 
So far, we have implemented a basic dithering pipeline that uses the Pillow library to dither a raster image with variable size and greytones. We use Pillow's `quantize` function with Floyd-Steinberg dithering to transform a .png file into a .pgm file with the target dimensions and color palette. 

We have also started implementing a basic Rust DSL that maps closely to knitout operations. We have specialized any of the  machine-specific parts of the knitout specifcation to only concern the SWGN2 machine. Futhermore, the DSL uses strong types to restrict the domain of the operations to only what produces valid knitout. On top of this, we have built abstractions for cast-on. We follow Shima's recommended cast-on procedure which start with a waste yarn section made up of shima's cast on and a tube, which is joined using two rows of draw thread to the piece itself, which starts with a french welt cast-on. 

We have set up the work to fabricate an image, but left the task of rendering the dithered image to knitout for the rest of the project.

Here is a snippet from the dithering part of the pipeline:

```Python
def dither(img: Image.Image, levels: int) -> Image.Image:
    greys = [round(i * 255 / (levels - 1)) for i in range(levels)]
    palette = Image.new("P", (1, 1))
    palette.putpalette([g for g in greys for _ in range(3)])
    return img.convert("RGB").quantize(palette=palette, dither=Image.Dither.FLOYDSTEINBERG)
```

At the core of the Rust library is a DSL that closely models Knitout operations. Here is a sample of the strongly-typed model:

```rust
/// A Knitout operation.
pub enum Op {
    /// `in CS`: Indicate that the given carrier set should be brought into
    /// action from the yarn grippers when next used.
    In(CarrierSet),
    ...
    /// `stitch L T`: Before forming a loop, pull needle by `L` machine units
    /// (leading). After forming a loop, pull needle by `T` machine units
    /// (stitch).
    Stitch(Stitch),
    /// `knit D N CS`: Pull a loop formed in direction `D` by the yarns in
    /// carriers CS through the loops on needle `N`, dropping them in the
    /// process. Knitting with an empty carrier set will drop.
    Knit(Direction, Needle, CarrierSet),
    ...
}

/// A `stitch L T` operation giving two machine units to retract a needle.
/// Before forming a loop, pull needle by `L` machine units (leading). After
/// forming a loop, pull needle by `T` machine units (stitch). On SWGN2
/// machines, leading and stitch numbers are stored in an on-machine table, and
/// accessed by an index.
pub enum Stitch {
    /// `stitch -10 10` (SWGN2 stitch number 81).
    Ln10_S10,
    /// `stitch 0 10` (SWGN2 stitch number 82).
    L0_S10,
    /// `stitch 10 10` (SWGN2 stitch number 83).
    L10_S10,
    ...
    /// `x-stitch-number N`: Sets the stitch number (an abstract index into a
    /// machine table of stitch and leading values). Valid stitch numbers are
    /// 1-120.
    Number(u8),
}
```

We build up higher-level abstractions by emitting sequences of these operations. Currently, this portion is quite verbose and work could be done to abstract it further like KnitScript.

```rust
for n in (0..width).step_by(2) {
    self.emit(Op::Knit(
        Direction::Plus,
        Needle::new(NeedleKind::FrontHook, n),
        carrier.into(),
    ));
}
```

Then, we encapsulate the logical sections into procedures:

```rust
self.shima_waste_yarn_cast_on(carrier, width);
self.waste_yarn_tube(carrier, width);
self.draw_thread(Carrier::C1, width);
self.french_welt_cast_on(carrier, width);
self.emit(Op::OutHook(Carrier::C1.into()));
self.emit(Op::PresserMode(PresserMode::Auto));
```

== Fabrication
To test the cast-on part of DSL, we generated the knitout for a small tube with shima-style and french welt cast-on. We examined the structure using the knitout visualizer, and compared it to a known knitout file. 

We then fabricated this knitout file using the Shima Seiki WholeGarment SWG 091 N2. The knitout file compiled to DAT and then 000 without errors:

#figure(
image("fabricated.jpg", width:70%),
caption: "Fabricated test design"
)
We used the `DEFAULT.999` configuration file, which caused intermittent holes in the french welt part of the cast on. We plan on tuning the `.999` file for our final fabricated swatch.There was also a hole in the side of the cast-on tube, which we will investigate before the final report:

#figure(
image("hole_in_cast_on_tube.jpg", width:70%),
caption: "Hole in cast-on tube"
)

#figure(
image("with_draw_thread_removed.jpg", width:70%),
caption: "Test design with draw thread removed"
)


== Milestone progress
We made progress on understanding general knitting concept, the knitout file format, and Shima-specific details, and were able to fabricate a design using the abstractions in our DSL. We also made progress towards the goal of knitting dithered images by setting up the dithering pipeline and fixing the interface between the pipeline and our DSL. 

While we did not accomplish our original goal of fabricating a bitonal image by this point, we now have the foundations to do so during the rest of our project. 

= Future plan

== Roadmap 
In the remaining time, we plan to connect our DSL to the dithering pipeline in order to automatically fabricate a dithered swatch from a raster image. We also plan to implement a bind-off abstraction in our DSL.

== Anticipated Roadblocks
We anticipate challenges with fabricating dithered images with more than two colors. We have not yet investigated or developed the infrastructure to manage 4 carriers at once while fabricating the dithered image. We plan on more thoroughly examining Alexis's sunset logic as an example of 4-tone dithering.

