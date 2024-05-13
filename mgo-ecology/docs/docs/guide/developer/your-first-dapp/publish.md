---
title: Publish a Package
description: Publish a Package
sidebar_position: 3
---

Before you can call functions in a Move package (beyond an emulated Mango execution scenario), that package must be available on the Mango network. When you publish a package, you are actually creating an immutable Mango object on the network that anyone can access.

To publish your package to the Mango network, you can use the `mgo client publish` command of the Mango Client CLI. After the package becomes available on the network, you can then use the  `mgo client call` command to access the functions available in the package.

### Module initializers

Each module in a package can include a special initializer function that runs at publication time. The goal of an initializer function is to pre-initialize module-specific data (for example, to create singleton objects). The initializer function must have the following properties for it to execute at publication:

- Function name must be `init`
- The parameter list must end with either a `&mut TxContext` or a `&TxContext` type
- No return values
- Private visibility
- Optionally, the parameter list starts by accepting the module's one-time witness by value

For example, the following `init` functions are all valid:

- `fun init(ctx: &TxContext)`
- `fun init(ctx: &mut TxContext)`
- `fun init(otw: EXAMPLE, ctx: &TxContext)`
- `fun init(otw: EXAMPLE, ctx: &mut TxContext)`

While the `mango move` command does not support publishing explicitly, you can still test module initializers using the testing framework by dedicating the first transaction to executing the initializer function.

Continuing the fantasy game example, the `init` function should create a `Forge` object.

```move
    /// Module initializer to be executed when this module is published
    fun init(ctx: &mut TxContext) {
        let admin = Forge {
            id: object::new(ctx),
            swords_created: 0,
        };

        // transfer the forge object to the module/package publisher
        transfer::transfer(admin, tx_context::sender(ctx));
    }
```

The tests you have so far call the `init` function, but the initializer function itself isn't tested to ensure it properly creates a `Forge` object. To test this functionality, modify the `sword_create` function to take the forge as a parameter and to update the number of
created swords at the end of the function:

```move
    /// Constructor for creating swords
    public fun new_sword(
        forge: &mut Forge,
        magic: u64,
        strength: u64,
        ctx: &mut TxContext,
    ): Sword {
        forge.swords_created = forge.swords_created + 1;
        // ...
    }
```

Now, create a function to test the module initialization:

```move
    #[test_only] use mango::test_scenario as ts;

    #[test_only] const ADMIN: address = @0xAD;

    #[test]
    public fun test_module_init() {
        let ts = ts::begin(@0x0);

        // first transaction to emulate module initialization.
        {
            ts::next_tx(&mut ts, ADMIN);
            init(ts::ctx(&mut ts));
        };

        // second transaction to check if the forge has been created
        // and has initial value of zero swords created
        {
            ts::next_tx(&mut ts, ADMIN);

            // extract the Forge object
            let forge: Forge = ts::take_from_sender(&mut ts);

            // verify number of created swords
            assert!(swords_created(&forge) == 0, 1);

            // return the Forge object to the object pool
            ts::return_to_sender(&mut ts, forge);
        };

        ts::end(ts);
    }
```

As the new test function shows, the first transaction (explicitly) calls the initializer. The next transaction checks if the `Forge` object has been created and properly initialized.

If you try to run tests on the whole package at this point, you encounter compilation errors in the existing tests because of the
`new_sword` function signature change. The changes required for the tests to run again is an exercise left for you. 