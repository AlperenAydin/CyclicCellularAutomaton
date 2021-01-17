# CyclicCellularAutomaton
An implementation of Cyclic Cellular Automaton in Rust

An explanation can be found [here](https://en.wikipedia.org/wiki/Cyclic_cellular_automaton)

TODO: 
~~* Add parallelism~~
* Improve memory management
* Maybe internet connection

I had assumed that the slowness of the program came from the calculation of new generations. After the adding the benches, it seems the slowness comes from the encoding of the frames into gifs. I should proabaly find a different way of taking care of this.