# ELF Parser Code Notes

This is a work-in-progress, mostly for learning purposes, and
there are some things that could definitely be improved or that
should be done differently for a production application.

Here are a few possible ideas in that direction:

## Error handling

There is currently not much effort made to recover from errors:
This could be improved. It also hasn't been tested on a wide
range of binaries, and further testing could reveal some
unforseen edge cases.

## Macro for parsing data structs

All three data structs -- for main, program, and section headers
-- are parsed in essentially the same way. Each field of these
structs is either
a static array of `u8` or a simple unsigned integral type.
We could make a `#[derive]`
macro to automate adding a `parse` method to these structs, to reduce
code duplication. And, there are other existing crates, like `binary_serde`,
that do similar parsing, which we could potentially use.
