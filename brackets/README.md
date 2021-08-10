# Brackets

The task is to validate a sequence to contain only a valid set of brackets.

Sometimes a random sequence generator is included in the task, but we'll omit this.

Here's the original task on the [Rosetta Code](http://www.rosettacode.org/wiki/Balanced_brackets)

> **Task:**
> 1. Generate a string with   N   opening brackets   `[`   and with   N   closing brackets   `]`,   in some arbitrary order.
> 2. Determine whether the generated string is balanced; that is, whether it consists entirely of pairs of opening/closing brackets (in that order), none of which mis-nest.

And there is plenty of solutions that do the thing, 
but I think that it's very boring to count only brackets.
Hence, we'll stick to several known pairs of characters:
`[], (), {}`. This list could be extended, of course.

## Solutions

As always, keep in mind that solutions could be not optimal.

- [Recursive](src/v1_recursive.rs) - not the best implementation, but usable.
- [Stack](src/v2_stack.rs) - looks much better than a recursive one