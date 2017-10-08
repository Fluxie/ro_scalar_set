# Read-only Scalar Set
A read-only scalar set written in Rust.

## Purpose

A light-weight scalar set for testing whether a scalar member is a member of the set or not. 
The whole data structure is based on a single buffer or slice. This enables attaching to an existing buffer or a slice, As such no copies are required when constructing the set from a stream.

## Details

The structure of the buffer:
| # buckets | # members | index to the first member in the 1st bucket | index to the first member in the 2nd bucket | ... | index to the first member of the last bucket | 1st bucket | 2nd bucket | ... | last bucket |
