# Notes

## Ownership, Borrowing and References

### Ownership

C, C# -> Memory Management Control Issue
Garbage Collector solved this issue, but created a new issued. [stopping/resuming the program]

Ownership introduced by Rust to solve memory safety issues and high performance at the same time.

What is Ownership?
Every value has a sing owner [every variable has one value, and itis its sole owner].

Ownership Rules

---------------

1. Each value is Rust has a variable that's its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped
