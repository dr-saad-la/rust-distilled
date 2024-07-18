# Summary

Introduction

-   [Introduction](intro.md)
-   [Rust history](rust-history.md)

# Rust Environment Setup
-   [Environment Setup](chap_01/introduction.md)
    -   [Rust Environment Setup](chap_01/install_rust.md)
    -   [Setup Rust Kernel](chap_01/setup_jupyterlab.md)
    -   [Update Rust](chap_01/update_rust.md)
    -   [Uninstall Rust](chap_01/uninstall_rust.md)

# Rust Command Line Tools
-   [Command Line Tool](chap_02/intro.md)
    -   [help](chap_02/cargo_help.md)
    -   [create rust app](chap_02/new.md)
    -   [check](chap_02/check.md)
    -   [build](chap_02/build.md)
    -   [clean](chap_02/clean.md)
    -   [Structuring Rust Applications](chap_02/rust_apps.md)
    -   [Advanced Rust Projects Structure](chap_02/advanced_rust_apps.md)

# Rust Essentials
-   [Primitive Data Types](chap_03/intro.md)
    -   [integer_dtypes](chap_03/integer_dtypes.md)
    -   [Floats](chap_03/float_dtypes.md)
    -   [Booleans](chap_03/booleans.md)
    -   [Characters](chap_03/char_type.md)
    -   [Constants](chap_03/constants.md)
    -   [Type conversion](chap_03/type_conversion.md)
    -   [numeric operations](chap_03/num_ops.md)
    -   [character operations](chap_03/char_ops.md)
    -   [Creating Ranges](chap_03/ranges.md)

# Control Flow

-   [Flow Control](chap_04/intro.md)
    -   [Conditionals](chap_04/conditionals.md)
        -   [If statement](chap_04/if_statement.md)
        -   [else if](chap_04/else_if.md)
        -   [If else expression](chap_04/if_else_expression.md)
    -   [Loops](chap_04/loops.md)
        -   [loop](chap_04/loop.md)
        -   [while loop](chap_04/while_loop.md)
        -   [for loop](chap_04/for_loop.md)
        -   [Advanced for loop](chap_04/advanced_for.md)
        -   [Break and Continue](chap_04/break_continue_intro.md)
            -   [break](chap_04/break.md)
            -   [continue](chap_04/continue.md)
        - [Advanced Loop Concepts](chap_04/advanced_loops_concepts.md)
    -   [Match statement](chap_04/match.md)

# Rust Compound Data Types
-   [Data Structures](chap_05/intro.md)
    -   [Arrays](chap_05/arrays.md)
    -   [Vectors](chap_05/vectors.md)
    -   [Tuples](chap_05/tuples.md)
    -   [HashMaps](chap_05/hashmaps.md)
    -   [HashSets](chap_05/hashsets.md)
    -   [Data Structure Operations](chap_05/data_structure_ops.md)
    -   [Iterations](chap_05/iterating.md)

# Enumerations in Rust
-   [Enum Data Type](chap_06/intro.md)
    -   [Creating Enums](chap_06/creating_enums.md)
    -   [Using Enums](chap_06/using_enums.md)
    -   [Enums with data](chap_06/enums_with_data.md)
    -   [Enums as Modules](chap_06/enums_as_modules.md)
    -   [Option Enum](chap_06/option.md)
    -   [Result Enum](chap_06/result_enum.md)
    -   [Enum in Practice](chap_06/enum_project.md)

# Ownership and Borrowing
-   [Variables Scope](chap_07/intro.md)
    -   [Local Scope](chap_07/local_scope.md)
    -   [Static Local Scope](chap_07/static_local.md)
    -   [Global Scope](chap_07/global_scope.md)
    -   [Static Global Scope](chap_07/static_global.md)
    -   [Static Mutables](chap_07/static_mutables.md)
- [String Object](chap_08/intro.md)
    - [String objects](chap_08/string-objs.md)
    - [String Slices](chap_08/string-slices.md)
    - [String Slices Ops](chap_08/string-slice-ops.md)
    - [Collections Operations](chap_08/intro.md)
        - [Array Slices](chap_08/arrays-slices.md)
        - [Array Slice Ops](chap_08/array-slice-ops.md)
    - [Copy and Move](chap_08/intro.md) 
        - [Copy](chap_08/copy.md)
        - [Move](chap_08/move.md)
        - [Clone](chap_08/clone.md)
    - [Borrowing](chap_08/intro.md)
        - [Simple Borrowing](chap_08/simple-borrow.md)
        - [Borrow Checker](chap_08/borrow-checker.md)

# Functions
- [Functions](chap_09/intro.md)
    - [Simple Functions](chap_09/simple-funcs.md)
    - [Default Parameters](chap_09/func-defaults.md)
    - [Higher-Order Functions](chap_09/higher-order.md)
    - [Closures](chap_09/closures.md)
    - [Function Pointers](chap_09/func-pointers.md)

# Structures
- [Structures](chap_10/intro.md)
    - [Defining Structs](chap_10/defining-structs.md)
    - [Tuple Structs](chap_10/tuple-structs.md)
    - [Unit-Like Structs](chap_10/unit-like-structs.md)
    - [Struct Methods](chap_10/struct-methods.md)
    - [Associated Functions](chap_10/associated-funcs.md)
    - [Struct Lifetime](chap_10/struct-lifetime.md)

# Traits
- [Traits](chap_11/intro.md)
    - [Defining Traits](chap_11/defining-traits.md)
    - [Trait Bounds](chap_11/trait-bounds.md)
    - [Default Implementations](chap_11/default-impls.md)
    - [Trait Objects](chap_11/trait-objects.md)
    - [Supertraits](chap_11/supertraits.md)
    - [Trait Inheritance](chap_11/trait-inheritance.md)

# Generics
- [Generics](chap_12/intro.md)
    - [Defining Generics](chap_12/defining-generics.md)
    - [Generic Functions](chap_12/generic-funcs.md)
    - [Generic Structs](chap_12/generic-structs.md)
    - [Generic Enums](chap_12/generic-enums.md)
    - [Generic Traits](chap_12/generic-traits.md)
    - [Lifetimes in Generics](chap_12/lifetimes-generics.md)

# Advanced Concepts
- [Advanced Concepts](chap_13/intro.md)
    - [Smart Pointers](chap_13/smart-pointers.md)
    - [Deref Trait](chap_13/deref-trait.md)
    - [Drop Trait](chap_13/drop-trait.md)
    - [Interior Mutability](chap_13/interior-mutability.md)
    - [Concurrency](chap_13/concurrency.md)
    - [Asynchronous Programming](chap_13/async-programming.md)

# Multithreading and Concurrency
- [Multithreading](chap_14/intro.md)
    - [Creating Threads](chap_14/creating-threads.md)
    - [Thread Safety](chap_14/thread-safety.md)
    - [Message Passing](chap_14/message-passing.md)
    - [Shared State](chap_14/shared-state.md)
    - [Mutexes](chap_14/mutexes.md)
    - [Atomic Types](chap_14/atomic-types.md)
- [Concurrency](chap_15/intro.md)
    - [Concurrency Patterns](chap_15/concurrency-patterns.md)
    - [Async-Await](chap_15/async-await.md)
    - [Futures and Promises](chap_15/futures-promises.md)
    - [Streams](chap_15/streams.md)
    - [Concurrency Libraries](chap_15/concurrency-libraries.md)

# Error Handling
- [Error Handling](chap_16/intro.md)
    - [Panic and Unwind](chap_16/panic-unwind.md)
    - [Result and Option](chap_16/result-option.md)
    - [Custom Error Types](chap_16/custom-errors.md)
    - [Error Handling Best Practices](chap_16/error-handling-best-practices.md)

# Modules and Packages
- [Modules](chap_17/intro.md)
    - [Defining Modules](chap_17/defining-modules.md)
    - [Module Hierarchy](chap_17/module-hierarchy.md)
    - [Re-exports](chap_17/re-exports.md)
    - [Path Imports](chap_17/path-imports.md)
- [Packages](chap_18/intro.md)
    - [Creating Packages](chap_18/creating-packages.md)
    - [Publishing Packages](chap_18/publishing-packages.md)
    - [Managing Dependencies](chap_18/dependencies.md)
    - [Versioning](chap_18/versioning.md)

# Testing
- [Testing](chap_19/intro.md)
    - [Unit Tests](chap_19/unit-tests.md)
    - [Integration Tests](chap_19/integration-tests.md)
    - [Testing Frameworks](chap_19/testing-frameworks.md)
    - [Writing Effective Tests](chap_19/effective-tests.md)
    - [Test Driven Development](chap_19/tdd.md)

# Memory Management
- [Memory Management](chap_20/intro.md)
    - [Heap vs Stack](chap_20/heap-vs-stack.md)
    - [Ownership Rules](chap_20/ownership-rules.md)
    - [Lifetimes](chap_20/lifetimes.md)
    - [Borrowing and References](chap_20/borrowing-references.md)
 
# Macros
- [Macros](chap_21/intro.md)
    - [Declarative Macros](chap_21/declarative-macros.md)
    - [Procedural Macros](chap_21/procedural-macros.md)
    - [Macro Rules](chap_21/macro-rules.md)
    - [Custom Derive](chap_21/custom-derive.md)
    - [Attribute-like Macros](chap_21/attribute-macros.md)
    - [Function-like Macros](chap_21/function-macros.md)

# Performance Optimization
- [Performance Optimization](chap_22/intro.md)
    - [Profiling](chap_22/profiling.md)
    - [Benchmarking](chap_22/benchmarking.md)
    - [Code Optimization](chap_22/code-optimization.md)
    - [Memory Optimization](chap_22/memory-optimization.md)
    - [Concurrency Optimization](chap_22/concurrency-optimization.md)
