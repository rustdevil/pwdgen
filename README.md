# pwdgen
A primitive password generator in Rust

## Usage
Generate a password with `pwdgen <option> <length> [s | silent]`. If no option and length is supplied, a password with 16 chars and the "all" configuration will be generated.

Passing `s` or `silent` makes pwdgen only return the password with nothing else.

There are four options: `upper`, `lower`, `numerical`, and `all`.
