# Flattening Graphs

This is a Rust implementation of a directed graph data structure that's flat.
By flat we mean it is stored as `Vec<Node>` and uses strongly a typed index
`NodeRef` which wraps a `usize` for pointers.

The implementation is inspired by a similar model for ASTs[^1] graphs are
pervasive in compilers and program analysis (lifting assembly to a graph based
IL for example). This model allows building graphs without much gymnastics with
the borrow checker. To learn more about how graphs play a role in optimizing
compilers or compilers in general there's a nice set of notes in [^2]

The code is pretty bare bones and was written to remind myself for the nth time
how to implement it. Making it generic with arenas or an even fancier allocator
might be an interesting project but for most use cases you can just rip out what
you need an change it.

[^1]: [Flattening ASTs](https://www.cs.cornell.edu/~asampson/blog/flattening.html)

[^2]: [Notes on Graphs in Optimizing Compilers](https://www.cs.umb.edu/~offner/files/flow_graph.pdf)
