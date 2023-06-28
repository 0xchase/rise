# Rise Framework

Rise is a binary analysis framework written in Rust, built on [rizin](https://github.com/rizinorg/rizin).

### Features
 - Rust bindings to Rizin
 - A custom Rust API build around the bindings
 - A new kinds of reverse engineering user interface
 - Custom decompiler written in haskell

### Disassembly
 - Write custom analysis data structure to store analysis information efficiently

### Emulation
 - Efficiently emulate binaries using the IL
 - Add hooks for relevant system calls and library calls

### User Interface
 - **Summary View**: An automated report on the functionality of the binary
 - **Function List**: List of functions in an namespace tree
 - **Global Call Graph**: A filterable global call graph
 - **Linear Disassembly**: Linear disassembly of function
 - **Graph Disassembly**: Graph disassembly of function
 - **Decompiler**: Decompiler view
