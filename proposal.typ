#title[Knither: Abstracting 2D Knitting]

Thalia Archibald and Jonas Regehr

= Introduction

*Problem statement*:
The tooling for producing knitting machine operations is difficult to use.
Usually, a user has to write lots of code by hand to process an image and generate Knitout operations.
This requires a mastery of the format and very tied to the individual image.
We propose to build an image processing pipeline to automate this for 2D swatches with dithering.

*Related work*:
- The Knitout file format @Knitout is a platform-independent description of low-level operations shared between different knitting machine models.
  It's a common format produced by many tools, which can then be compiled to machine-specific operations and validated.
  It has no control flow or abstraction and is very low level.
- The knitting compiler @knitting-compiler is the seminal knitting abstraction.
  It provides an OpenSCAD-like model of composing knit objects out of primitives like tubes and sheets.
  This enables models to be easily scaled and joints to be rotated.
  The inputs are expressed graphically.
- KnitScript @KnitScript is a domain-specific programming language that provides facilities for specifying knitting machine operations in a high-level language.
  They implement control flow and variable bindings.
  Programs are expressed in terms of primitives: needles, loops, carriers, yarn, sheets, and pass directions.
  Unlike the knitting compiler, tubes are implemented in terms of sheets.
  As a limitation, it's more difficult to perform transformations such as flipping or composition.
- Co-Dithering @CoDithering produces double-sided swatches with independent images on either side.
  Before this work, the back would be the inverse image or a checkerboard.
  They solve it optimally with dynamic programming.
- Autoknit @Autoknit takes a 3D STL model, identifies whether its 3D surfaces are knitable, and generates Knitout operations to fabricate it.
  As discussed in the KnitScript paper, this has the limitation that it cannot produce a well-fitted sweater, because 3D models don't encode enough knitting domain information.
- QUILT @QUILT makes 2D knitting design modular.
  This enables a user to produce smaller knitting programs that are merged into a larger design.

= Approach

*Computational*:
We are building abstractions for the knitting machine.
To make it tractable for the class and deliver an interesting result, we are focusing on producing knit swatch patterns from 2D raster images.
This will be implemented as an image processing pipeline that produces Knitout operations.

The knitting machine has a limited color palette, allowing up to 4 colors in a horizontal span or up 10 total colors in an object.
This makes dithering a good fit for representing images with more colors.
We will use several objects as case studies as we increase the capabilities of our system: for two-color dithering, we will use the Utah teapot swatch; for four-color, Alexis' sunset swatch; and for adaptively switching colors to handle images with more than four colors, Alexis' Pokémon pixel art idea.
We will pull in ideas from the Co-Dithering paper.

As a stretch goal, once we master producing Knitout instructions for 2D objects, we could branch out into building an OpenSCAD-style object composition system.
We would have sheet and tube primitives like the knitting compiler and since sheets and tubes are rectangular, we could apply our 2D raster image texturing to 3D objects.
If we were to apply textures to scanned models, we would need a surface-stable dithering algorithm like _Obra Dinn_ instead.

*Fabrication*:
We are using the Shima Seiki industrial knitting machine.
It will be challenging to learn the knitting terminology and idioms.
Since there's limited access to the machine and DAT to 000 compiler, the feedback loop for the fabricability of our Knitout output will be difficult.
We have the Knitout to DAT compiler, which will catch some errors, and plan on lots of iteration and collaboration in the knitting lab.

*Resources and budget*:
We will be using existing resources established in the knitting lab.
We do not expect to incur much of a budget burden.

= Expected Results

*Final demo*:
We aim to have a completed pipeline for processing a raster image to a knit swatch using four colors.
As a stretch goal, we could develop an algorithm to utilize colors more than four colors adaptively across an image.
We will show swatches produced from a variety of images illustrating challenges in the system.

*Midterm milestone*:
By this milestone, we aim to complete bitonal image processing and fabricate a teapot or similar image.

#bibliography(style: "association-for-computing-machinery", full: true, bytes("
@online{Knitout,
author  = {McCann, James},
title   = {The {Knitout} (.k) File Format},
version = {0.6},
url     = {https://textiles-lab.github.io/knitout/knitout.html},
urldate = {2026-10-23}
}

@article{knitting-compiler,
author = {James McCann and Lea Albaugh and Vidya Narayanan and April Grow and Wojciech Matusik and Jennifer Mankoff and Jessica K. Hodgins},
title = {A Compiler for 3D Machine Knitting},
journal = {Proceedings of ACM Transactions on Graphics (TOG)},
year = {2016},
month = {July},
volume = {35},
number = {4},
}

@inproceedings{KnitScript,
author = {Hofmann, Megan and Albaugh, Lea and Wang, Tongyan and Mankoff, Jennifer and Hudson, Scott E},
title = {KnitScript: A Domain-Specific Scripting Language for Advanced Machine Knitting},
year = {2023},
isbn = {9798400701320},
publisher = {Association for Computing Machinery},
address = {New York, NY, USA},
url = {https://doi.org/10.1145/3586183.3606789},
doi = {10.1145/3586183.3606789},
abstract = {Knitting machines can fabricate complex fabric structures using robust industrial fabrication machines. However, machine knitting's full capabilities are only available through low-level programming languages that operate on individual machine operations. We present KnitScript, a domain-specific machine knitting scripting language that supports computationally driven knitting designs. KnitScript provides a comprehensive virtual model of knitting machines, giving access to machine-level capabilities as they are needed while automating a variety of tedious and error-prone details. Programmers can extend KnitScript with Python programs to create more complex programs and user interfaces. We evaluate the expressivity of KnitScript through a user study where nine machine knitters used KnitScript code to modify knitting patterns. We demonstrate the capabilities of KnitScript through three demonstrations where we create: a program for generating knitted figures of randomized trees, a parameterized hat template that can be modified with accessibility features, and a pattern for a parametric mixed-material lampshade. KnitScript advances the state of machine-knitting research by providing a platform to develop and share complex knitting algorithms, design tools, and patterns. 1},
booktitle = {Proceedings of the 36th Annual ACM Symposium on User Interface Software and Technology},
articleno = {21},
numpages = {21},
location = {San Francisco, CA, USA},
series = {UIST '23}
}

@inproceedings{CoDithering,
author = {Xu, Yue and McCann, James},
title = {Co-Dithering for Jacquard Knitting},
year = {2025},
isbn = {9798400720345},
publisher = {Association for Computing Machinery},
address = {New York, NY, USA},
url = {https://doi.org/10.1145/3745778.3766646},
doi = {10.1145/3745778.3766646},
booktitle = {Proceedings of the ACM Symposium on Computational Fabrication},
articleno = {19},
numpages = {14},
keywords = {knitting, colorwork, double-bed jacquard, dithering},
series = {SCF '25}
}

@article{Autoknit,
author = {Narayanan, Vidya and Albaugh, Lea and Hodgins, Jessica and Coros, Stelian and McCann, James},
title = {Automatic Machine Knitting of 3D Meshes},
journal = {ACM Trans. Graph.},
issue_date = {August 2018},
volume = {37},
number = {3},
month = aug,
year = {2018},
issn = {0730-0301},
pages = {35:1--35:15},
articleno = {35},
numpages = {15},
url = {http://doi.acm.org/10.1145/3186265},
doi = {10.1145/3186265},
acmid = {3186265},
publisher = {ACM},
address = {New York, NY, USA},
keywords = {3D printing, CAM, Knitting, automatic knitting},
}

@inproceedings{QUILT,
author = {Hester, Jack and Law, Sebastian and Hofmann, Megan},
title = {QUILT: Supporting Modular Design of Machine-Knitting Programs},
year = {2025},
isbn = {9798400720376},
publisher = {Association for Computing Machinery},
address = {New York, NY, USA},
url = {https://doi.org/10.1145/3746059.3747608},
doi = {10.1145/3746059.3747608},
abstract = {Knitting machines can manufacture complex layered, textured, and multi-material fabrics and garments. With new programming languages and interfaces there is greater access to the machine's capabilities. Developers can now create machine instructions that produce a fabric sample or garment. Such files can be easily shared with others, but modifying and combining these samples requires extensive expertise in knitting-specific programming languages, substantial effort, and time. Knit programming offers little support for modular design. We take a step towards modular knitting-machine programming and present QUILT: Quality Unification Infrastructure for Loop-based Textiles. QUILT enables knit programmers to create swatches from knitting programs and lay these swatches out spatially on a 2-dimensional grid. We use three novel knit-program merging algorithms to merge the connected swatches into a quilt program. The knitted structures of each swatch remain unchanged, and our algorithms ensure that the swatches are joined by a seamless boundary that maintains the constraints of knitting-machine programming and knitted-structure construction.},
booktitle = {Proceedings of the 38th Annual ACM Symposium on User Interface Software and Technology},
articleno = {101},
numpages = {26},
keywords = {Machine Knitting; Digital Fabrication; Computer-Aided Design},
location = {Busan, Republic of Korea},
series = {UIST '25}
}
"))
