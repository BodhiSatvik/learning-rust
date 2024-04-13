## Ownership
Ownership is Rust’s most unique feature and has deep implications for the rest of the language. 
It enables Rust to make memory safety guarantees without needing a garbage collector, so it’s important to understand how ownership works. 
In this chapter, we’ll talk about ownership as well as several related features: borrowing, slices, and how Rust lays data out in memory.

First we need to understand garbage collection, which is essentially an algorithm that helps in memory management. 
Its pros are being (mostly) error free and having a faster write time. However, the user has no control over memory management, the garbange collection can happen whenever which can lead to effects in performance in a large program

Manual memory management is the exact opposite, where you have more control over memory, a faster runtime with smaller program sizes, but it's error prone and has slower write times. Also I'm too stupid to use C lol. 

Here's where the ownership model comes in apparently, where you have control over your memory, its mostly error free, fast runtime with smaller program sizes. But the learning curve is higher, and write times are slower like in manual management. 

That being said, here are 3 rules. Get them tattooed:
1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.



