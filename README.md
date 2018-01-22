# Examples based on the second edition of [*The Rust Programming Language* book](https://doc.rust-lang.org/book/second-edition)

[Rust](https://www.rust-lang.org) is an imperative strongly-typed language with some functional features. Conceived as a systems programming language it also offers considerable concurrency support.

It is NOT object-oriented, featuring the following [algebraic data types](https://en.wikipedia.org/wiki/Algebraic_data_type):
* Product types: **tuples** and **structs**
* Sum types: **enums**

**Generics** and **traits** are used for type and behavior abstraction. 

There are neither exceptions nor nulls. *Result* and *Option* enums are generally used instead.

It ships with **cargo**, a modern package and build manager.

## Memory safety

Rust's main focus is on memory safety without impacting performance. There is no garbage collector, yet the memory is freed automatically when it is no longer needed.

The central concept is *ownership*. Every piece of data in memory belongs to one and only one variable. This ownership can be transferred from one variable to another in which case the old owner is no longer valid. When the owner variable goes out of scope, the memory holding the owned data is freed. This prevents memory waste and avoids double free errors.

Owners can also *borrow* the data through *references* and this borrow may be read-only or it may allow to change the data. Thus, there are both mutable and immutable references. Several immutable references may coexist but there may only be one mutable reference. Furthermore, there is either a mutable reference or any number of immutable ones: no immutable references are allowed if there is a mutable one, and viceversa. This prevents data races.

All references have a *lifetime* and they may have dependencies on others in which case their lifetimes must be compatible. The compiler is often capable of inferring and enforcing these relationships. However, in some cases the dependency must be explicitly declared through lifetime annotations. This annotations don't change how long the references live: they simply relate the lifetimes of multiple references. This prevents dangling references.
