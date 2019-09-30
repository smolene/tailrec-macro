# tailrec-macro

This macro converts a tail-recursive function into a loop, consuming only O(n) space. Please don't use this in your projects, it is not tested and won't be maintained, it's just a proof of concept.
This macro is not on crates.io because it is incomplete.
**NOTE: ONLY FUNCTIONS WITH AN *EXPLICIT* RETURN WILL BE ELIMINATED.**

# How to use it?
It takes your function and looks for *explicit returns of your function* and replaces it to loop instead.

To view the result of expansion, clone the repo and execute `cargo expand --bin tailrec`, that's the best way to understand what's going on.

This function that calculates the sum of elements from 0 to n goes from:

```
#[tailrec]
fn h(n: u64, acc: u64) -> u64 {
        if n == 0 {
            acc
        } else {
            return h(n - 1, acc + n); // Please note the explicit return
        }
    }
}
```
to
```
fn f(n: u64, acc: u64) -> u64 {
        let mut n = n;
        let mut acc = acc;
        loop {
            let res = {
                {
                    if n == 0 {
                        acc
                    } else {
                        {
                            let arguments__ = (n - 1, acc + n);
                            n = arguments__.0;
                            acc = arguments__.1;
                            continue;
                        }
                    }
                }
            };
            return res;
        }
    }
}
```

# How does this work?

First, the macro redeclares the parameters of the function as mutable (iteration requires mutability most of the time).

```
fn f(n: u64, acc: u64) -> u64 {
        let mut n = n;
        let mut acc = acc;
        ...
```

Then, the program enters a `loop { ... }` where we assign the result of the function to `res`, before returning it.

```
        loop {
            let res = { original_function_body_modified };
            return res;
        }
```

Then, we modifiy the interior of the original function body to replace all `return f(a, b, c)` to a variable assignment and a `continue`.

```
return h(n - 1, acc + n);
```

becomes

```
let arguments__ = (n - 1, acc + n); // <-- this tuple is just the interior of the function call copy-pasted
n = arguments__.0; // variables n, acc are assigned in the order of their declaration in the signature.
acc = arguments__.1;
continue;
```

We use `continue` to simulate the recursive call.
Note that we use the `let arguments__ = (tuple)` to make sure variables are assigned all at the same time, like a function call.

# How to contribute?

IDK, I will probably forget about this repo in a week lol but plz file issues if you find some problems.
