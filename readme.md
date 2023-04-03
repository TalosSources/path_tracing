# Path Tracer

This is a simple path tracer written in pure rust (no GL) and running on the cpu (multi-threaded) for now. It is probably underfeatured and slow but it exists mainly for (self-)didactic purposes.
Currently, it supports : 
 * diffuse glossy and specular reflections
 * specular refractions 
 * the combining of the two via fresnel equations
 * semi-specular surfaces
 * spheres and planes primitives

Hopefully, more features can be added later. (see roadmap.md)

## Usage

Simply run the main.rs file, with cargo run or by compiling it.
It isn't documented yet, but one can change several options, and build scenes by looking at how it is done in the source code.
