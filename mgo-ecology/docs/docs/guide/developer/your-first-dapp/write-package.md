---
title: Write a Move Package
description: Write a Move Package
sidebar_position: 1
---


Before you can build a Move package and run its code, you must first [install Mango binaries](../geting-started/install-mango) for your operating system.

### Create the package

To begin, open a terminal or console at the location you plan to store your package. Use the `mgo move new` command to create an empty Move package with the name `my_first_package`:

``` shell
mgo move new my_first_package
```

Running the previous command creates a directory with the name you provide (`my_first_package`). The command populates the new directory with a skeleton Move project that consists of a `sources` directory and a `Move.toml` manifest file. Open the manifest with a text editor to review its contents:

```shell
cat my_first_package/Move.toml
[package]
name = "my_first_package"
version = "0.0.1"

[dependencies]
Mango = { git = "https://github.com/MangoNet-Labs/mango.git", subdir = "crates/mango-framework/packages/mango-framework", rev = "framework/devnet" }

[addresses]
my_first_package = "0x0"
```

### Defining the package

You have a package now but it doesn't do anything. To make your package useful, you must add logic contained in `.move` source files that define *modules*. Use a text editor or the command line to create your first package source file named `my_module.move` in the `sources` directory of the package:

``` shell
touch my_first_package/sources/my_module.move
```

Populate the `my_module.move` file with the following code:

```move
module my_first_package::my_module {

    // Part 1: Imports
    use mango::object::{Self, UID};
    use mango::transfer;
    use mango::tx_context::{Self, TxContext};

    // Part 2: Struct definitions
    struct Sword has key, store {
        id: UID,
        magic: u64,
        strength: u64,
    }

    struct Forge has key, store {
        id: UID,
        swords_created: u64,
    }

    // Part 3: Module initializer to be executed when this module is published
    fun init(ctx: &mut TxContext) {
        let admin = Forge {
            id: object::new(ctx),
            swords_created: 0,
        };
        // Transfer the forge object to the module/package publisher
        transfer::transfer(admin, tx_context::sender(ctx));
    }

    // Part 4: Accessors required to read the struct attributes
    public fun magic(self: &Sword): u64 {
        self.magic
    }

    public fun strength(self: &Sword): u64 {
        self.strength
    }

    public fun swords_created(self: &Forge): u64 {
        self.swords_created
    }

    // Part 5: Public/entry functions (introduced later in the tutorial)

    // Part 6: Private functions (if any)

}
```

The comments in the preceding code highlight different parts of a typical Move source file.

* **Part 1: Imports** - Code reuse is a necessity in modern programming. Move supports this concept with imports that allow your module to use types and functions declared in other modules. In this example, the module imports from `object`, `transfer`, and `tx_context` modules. These modules are available to the package because the `Move.toml` file defines the Mango dependency (along with the `mango` named address) where they are defined.

* **Part 2: Struct declarations** - Structs define types that a module can create or destroy. Struct definitions can include abilities provided with the `has` keyword. The structs in this example, for instance, have the `key` ability, which indicates that these structs are Mango objects that you can transfer between addresses. The `store` ability on the structs provide the ability to appear in other struct fields and be transferred freely.

* **Part 3: Module initializer** - A special function that is invoked exactly once when the module publishes.

* **Part 4: Accessor functions** - These functions allow the fields of the module's structs to be read from other modules.

After you save the file, you have a complete Move package.

In the next topic, you learn to build and test your package to get it ready for publishing.
