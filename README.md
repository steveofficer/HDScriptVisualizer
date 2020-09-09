This is an experimental application that uses Rust->WASM alongside React to perform
parsing HotDocs scripts from a component file and display the relationships between the various components.

This allows people to see how the various components are used within the script.

The project has 3 main parts to it.

1. A script parser and analyzer written in Rust and compiled as a Web Assembly Module. This code is located in `src/native`
2. A Web Worker that runs the Web Assembly Module on a background thread. This code is located in `src/web-worker`
3. A React application that allows a user to open a component file, triggers the Web Worker to run the analyzer, and then renders the
   result as a directed graph.