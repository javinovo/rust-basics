# Examples based on the second edition of [*The Rust Programming Language* book](https://doc.rust-lang.org/book/second-edition)

[Rust](https://www.rust-lang.org) is a [procedural](https://en.wikipedia.org/wiki/Procedural_programming) strongly-typed language with some functional features like **closures** and **iterators**. Conceived as a systems programming language it also offers considerable concurrency support and interoperability with other languages.

It is not object-oriented, [arguably](https://doc.rust-lang.org/book/second-edition/ch17-01-what-is-oo.html). It features the following [algebraic data types](https://en.wikipedia.org/wiki/Algebraic_data_type):
* Product types: **tuples** and **structs**
* Sum types: **enums**

**Generics** and **traits** are used for type and behavior abstraction. 

There are neither nulls nor exceptions. *Option* and *Result* enums are generally used instead. **Pattern matching** makes it trivial to consume them. Moreover, the `?` operator helps dealing with *Result* by unpacking the result if successful or returning from the containing function with the error otherwise.

It ships with **cargo**, a modern package and build manager. Packages are called [**crates**](https://crates.io/). It provides commands for testing and documentation generation from code comments in Markdown notation. Even more, documentation can include examples that can be automatically tested with cargo.

## Memory safety

Rust's main focus is on memory safety without impacting performance. There is no garbage collector, yet the memory is freed automatically when it is no longer needed.

The central concept is *ownership*. Every piece of data in memory belongs to one and only one variable. This ownership can be transferred from one variable to another in which case the old owner is no longer valid. When the owner variable goes out of scope, the memory holding the owned data is freed. This prevents memory waste and avoids double free errors.

Owners can also lend the data to other variables through *references*. This is called *borrowing* and it may allow the borrower to read the data or change it. Thus, there are both mutable and immutable references. Several immutable references may coexist but there may only be one mutable reference. Furthermore, there is either a mutable reference or any number of immutable ones: no immutable references are allowed if there is a mutable one, and viceversa. This prevents data races.

All references have a *lifetime* and they may have dependencies on others in which case their lifetimes must be compatible. The compiler is often capable of inferring and enforcing these relationships. However, in some cases the dependency must be explicitly declared through lifetime annotations. This annotations don't change how long the references live: they simply relate the lifetimes of multiple references. This prevents dangling references.

References are mere pointers to data. Smart pointers are structs pointing to data they own and having additional metadata and capabilities. For instance, they implement the *Deref* and *Drop* traits. The former enables the use of the `*` dereference operator while the latter provides the destructor method in charge of freeing resources. Smart pointers help cope with the ownership and reference restrictions by adding a layer of indirection and counting references.

Some smart pointers include *unsafe code*. Rust can be thought of as a combination of two programming languages: Safe Rust and Unsafe Rust. The latter extends the former by allowing some extra things which are not safe but may be required in some situations such as writing low-level abstractions or talking to other languages.

## Concurrency

Rust offers *1:1 threading* meaning that each Rust thread maps to an OS thread. It doesn't natively support a green thread model where a number of language threads maps to a different number of OS threads. This improves performance and minimizes the compiled binary size. However, there are crates offering green threading.

Ownership rules play a vital role in preventing errors in concurrent programming at compile time: the type system and the borrow checker will make sure the code won’t end up with data races or invalid references. 

Most types implement both *Send* and *Sync* marker traits which allow for ownership transfer between threads and referencing from multiple threads, respectively. Furthermore, types composed entirely of types implementing them are automatically marked with them. Manually implementing these traits involves implementing unsafe code.

Transferring data between threads is favored over sharing memory. Message passing through channels is offered in this regard. However, if state must be shared the *Arc* and *Mutex* smart pointers provide fairly safe patterns to do so.