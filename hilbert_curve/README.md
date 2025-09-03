Draw Hilbert curves and use them to create beautiful gardens.

To draw a Hilbert curve of order 5:
```
cargo run hilbert -o 5
```

To build a garden with 18 crops on a 10x10 grid:

```
cargo run garden -p A:5,B:5,C:5,D:5,E:5,F:5,G:5,H:5,I:5,J:5,K:5,L:5,M:5,N:5,O:8,P:8,Q:7,R:7 -w 10 -h 10 -v
```

The objective is to have continuous areas with a given crop, favoring some kind of compactness.
