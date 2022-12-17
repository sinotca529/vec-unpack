# `vec-unpack`
Creates a `Vec` containing the arguments.

`vecu!` unpacks iterable variables marked with `@`.

```
use vec_unpack::*;
let a = [2, 3];
let b = vecu![0, 1, @a, 4, 5];
assert_eq!(b, vec![0, 1, 2, 3, 4, 5]);
```
