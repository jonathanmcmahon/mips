# mips
A MIPS processor simulator written in rust.

## MIPS processor diagram ##

![](images/MIPS_datapath_control.gif)

(Source: [Ruye Wang](http://fourier.eng.hmc.edu/e85_old/lectures/processor/node6.html))

## Design notes ##

* Clock cycles can be simulated by iterating over all CPU components and calling a cycle() function on each, which will execute whatever activities that component performs during a cycle. One complete iteration over all components corresponds to one clock cycle.

* Separate compiler will convert MIPS assembly instructions to hex instructions and the processor will take such a compiled file as input. The file will contain one hex instruction per line.

## Design references ##

[Detailed, step-by-step MIPS design](https://www.cise.ufl.edu/~mssz/CompOrg/CDA-proc.html)

[RISC Architecture - MIPS (Stanford)](https://cs.stanford.edu/people/eroberts/courses/soco/projects/risc/mips/index.html)

[MIPS Processor (Multiple Cycle)](http://fourier.eng.hmc.edu/e85_old/lectures/processor/node6.html)

[MIPS Architecture (Wikipedia)](https://en.wikipedia.org/wiki/MIPS_architecture)

[Design of Pipelined MIPS Processor](https://www.cs.cmu.edu/afs/cs/academic/class/15740-f97/public/info/pipeline-slide.pdf)

[MIPS Converter (Instruction to Hex)](https://www.eg.bucknell.edu/~csci320/mips_web/)