# 4byte Collider

A simple script to find function signatures that have colliding 4byte selectors.

## Usage

1) Insert the signature for your target function in the `TARGET` global variable. Remember, signatures should take the form `"function_name(arg1_type,arg2_type,...)"` with no spaces between the types.

2) Decide on the structure for the functions you are testing. Currently, it's set up to test different versions of function names related to fees, all of which use 4 argument types, each of which is greater than 20 bytes. You can customize this structure by changing the imported files and arragement of the `signature` string on L55.

3) Add all the words you want to test into each of the files in `words/`.

4) Run with `cargo run` and wait for it to print successful results. Only 1 in ~4 billion signatures will have colliding selectors, so it can take a few hours to find one.
