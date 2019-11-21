# My Toy Mesh Path Tracer

I ran into https://github.com/aras-p/ToyMeshPathTracer, which is an assignment used at Unity as a base for job interview tasks. I tried it in Rust.

Besides the language difference, I did not use SIMD (yet) and my BVH uses a better heuristic. 

## Performance
On my computer and using Sponza: 
- Aras's version ~930 K Rays/s 
- mine is ~1250 K Rays/s.

It should be noted that, on my version, the BVH take 230 seconds to build.
