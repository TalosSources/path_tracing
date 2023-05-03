# Path Tracer

This is a simple path tracer written in pure rust (no GL) and running on the cpu (multi-threaded) for now. It is probably underfeatured and slow but it exists mainly for (self-)didactic purposes.
Currently, it supports : 
 * diffuse, glossy and specular reflections
 * specular refractions 
 * the combining of the two via fresnel equations
 * semi-specular surfaces
 * spheres, planes and rectangle primitives

Hopefully, more features can be added later. (see roadmap.md)

## Usage

Simply run the main.rs file, with cargo run or by compiling it.
It isn't documented yet, but one can change several options, and build scenes by looking at how it is done in the source code.

## Examples
Here are some renders made with the engine :

![alt text](https://github.com/TalosSources/path_tracing/blob/main/ressources/example_1.png?raw=true)

![alt text](https://github.com/TalosSources/path_tracing/blob/main/ressources/example_2.png?raw=true)

![alt text](https://github.com/TalosSources/path_tracing/blob/main/ressources/example_3.png?raw=true)
