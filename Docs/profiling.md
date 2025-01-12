#Flamegraph

This is a third-party tool that I use for profiling this crate.

Note that this only profiles the time that the code is actually running, and
not, for example, time spent sleeping or waiting for IO. In the long-term that
shouldn't be too much of a concern for us.

##Usage

```
cargo --color=always flamegraph --dev -- ...

# Then use a web-browser to look at the output image
```
