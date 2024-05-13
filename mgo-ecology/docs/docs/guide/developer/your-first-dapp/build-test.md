---
title: Build and Test Packages
description: Build and Test Packages
sidebar_position: 2
---


If you followed [Write a Move Package](./write-package), you have a basic module that you need to build. If you didn't, then either start with that topic or use your package, substituting that information where appropriate.

## Building your package

Make sure your terminal or console is in the directory that contains your package (`my_first_package` if you're following along). Use the following command to build your package:

```bash
mgo move build
```

A successful build returns a response similar to the following:

```bash
UPDATING GIT DEPENDENCY https://github.com/MangoNet-Labs/mango.git
INCLUDING DEPENDENCY Mango
INCLUDING DEPENDENCY MoveStdlib
BUILDING my_first_package
```

If the build fails, you can use the verbose error messaging in output to troubleshoot and resolve root issues.

Now that you have designed your asset and its accessor functions, it's time to test the package code before publishing.

## Testing a package {#testing-a-package}

Mango includes support for the Move testing framework that enables you to write unit tests that analyze Move code much like test frameworks for other languages (for example, the built-in Rust testing framework or the JUnit framework for Java).

An individual Move unit test is encapsulated in a public function that has no parameters, no return values, and has the `#[test]` annotation. The testing framework executes such functions when you call the `mango move test` command from the package root (`my_move_package` directory as per the current running example):

```bash
mgo move test
```

If you execute this command for the package created in [Write a Package](./write-package), you see the following output. Unsurprisingly, the test result has an `OK` status because there are no tests written yet to fail.

```bash
BUILDING Mgo
BUILDING MoveStdlib
BUILDING my_first_package
Running Move unit tests
Test result: OK. Total tests: 0; passed: 0; failed: 0
```

To actually test your code, you need to add test functions. Start with adding a basic test function to the `my_module.move` file, inside the module definition:

```move
    #[test]
    public fun test_sword_create() {

        // Create a dummy TxContext for testing
        let ctx = tx_context::dummy();

        // Create a sword
        let sword = Sword {
            id: object::new(&mut ctx),
            magic: 42,
            strength: 7,
        };

        // Check if accessor functions return correct values
        assert!(magic(&sword) == 42 && strength(&sword) == 7, 1);
    }
```

As the code shows, the unit test function (`test_sword_create()`) creates a dummy instance of the `TxContext` struct and assigns it to `ctx`. The function then creates a sword object using `ctx` to create a unique identifier and assigns `42` to the `magic` parameter and `7` to `strength`. Finally, the test calls the `magic` and `strength` accessor functions to verify that they return correct values.

The function passes the dummy context, `ctx`, to the `object::new` function as a mutable reference argument (`&mut`), but passes `sword` to its accessor functions as a read-only reference argument, `&sword`.

Now that you have a test function, run the test command again:

```bash
mgo move test
```

After running the `test` command, however, you get a compilation error instead of a test result:

```bash
error[E06001]: unused value without 'drop'
   ┌─ ./sources/my_module.move:60:65
   │
 4 │       struct Sword has key, store {
   │              ----- To satisfy the constraint, the 'drop' ability would need to be added here
   ·
27 │           let sword = Sword {
   │               ----- The local variable 'sword' still contains a value. The value does not have the 'drop' ability and must be consumed before the function returns
   │ ╭─────────────────────'
28 │ │             id: object::new(&mut ctx),
29 │ │             magic: 42,
30 │ │             strength: 7,
31 │ │         };
   │ ╰─────────' The type 'MyFirstPackage::my_module::Sword' does not have the ability 'drop'
   · │
34 │           assert!(magic(&sword) == 42 && strength(&sword) == 7, 1);
   │                                                                   ^ Invalid return
```

The error message contains all the necessary information to debug the code. The faulty code is meant to highlight one of the Move language's safety features.

The `Sword` struct represents a game asset that digitally mimics a real-world item. Obviously, a real sword cannot simply disappear (though it can be explicitly destroyed), but there is no such restriction on a digital one. In fact, this is exactly what's happening in the `test` function - you create an instance of a `Sword` struct that simply disappears at the end of the function call. If you saw something disappear before your eyes, you'd be dumbfounded, too.

One of the solutions (as suggested in the error message), is to add the `drop` ability to the definition of the `Sword` struct, which would allow instances of this struct to disappear (be dropped). The ability to drop a valuable asset is not a desirable asset property in this case, so another solution is needed. Another way to solve this problem is to transfer ownership of the `sword`.

To get the test to work, add the following line to the beginning of the testing function to import the Transfer module:

```move
use mgo::transfer;
```

With the Transfer module imported, add the following lines to the end of the test function (after the `!assert` call) to transfer ownership of the `sword` to a freshly created dummy address:

```move
// Create a dummy address and transfer the sword
let dummy_address = @0xCAFE;
transfer::transfer(sword, dummy_address);
```

Run the test command again. Now the output shows a single successful test has run:

```
BUILDING MoveStdlib
BUILDING Mgo
BUILDING my_first_package
Running Move unit tests
[ PASS    ] 0x0::my_module::test_sword_create
Test result: OK. Total tests: 1; passed: 1; failed: 0
```

:::tip

Use a filter string to run only a matching subset of the unit tests. With a filter string provided, the `mango move test` checks the fully qualified (`<address>::<module_name>::<fn_name>`) name for a match.

:::

Example:

```bash
mgo move test sword
```

The previous command runs all tests whose name contains `sword`.

You can discover more testing options through:
```bash
mango move test -h
```

## Mango-specific testing

The previous testing example is largely pure Move and isn't specific to Mango beyond using some Mango packages, such as `mango::tx_context` and `mango::transfer`. While this style of testing is already useful for writing Move code for Mango, you might also want to test additional Mango-specific features. In particular, a Move call in Mango is encapsulated in a Mango transaction, and you might want to test interactions between different transactions within a single test (for example, one transaction creating an object and the other one transferring it).
Mango-specific testing is supported through the `test_scenario` module that provides Mango-related testing functionality otherwise unavailable in pure Move and its testing framework.

The `test_scenario` module provides a scenario that emulates a series of Mango transactions, each with a potentially different user executing them. A test using this module typically starts the first transaction using the `test_scenario::begin` function. This function takes an address of the user executing the transaction as its argument and returns an instance of the `Scenario` struct representing a scenario.

An instance of the `Scenario` struct contains a per-address object pool emulating Mango object storage, with helper functions provided to manipulate objects in the pool. After the first transaction finishes, subsequent test transactions start with the `test_scenario::next_tx` function. This function takes an instance of the `Scenario` struct representing the current scenario and an address of a user as arguments.
Update your `my_module.move` file to include entry functions callable from Mango that implement `sword` creation and transfer. With these in place, you can then add a multi-transaction test that uses the `test_scenario` module to test these new capabilities. Put these functions after the accessors (Part 5 in comments).

```move
    public fun sword_create(magic: u64, strength: u64, recipient: address, ctx: &mut TxContext) {
        use mango::transfer;

        // create a sword
        let sword = Sword {
            id: object::new(ctx),
            magic: magic,
            strength: strength,
        };
        // transfer the sword
        transfer::transfer(sword, recipient);
    }

    public fun sword_transfer(sword: Sword, recipient: address, _ctx: &mut TxContext) {
        use mango::transfer;
        // transfer the sword
        transfer::transfer(sword, recipient);
    }
```

The code of the new functions uses struct creation and Mango-internal modules (`TxContext` and `Transfer`) in a way similar to what you have seen in the previous sections. The important part is for the entry functions to have correct signatures.

With the new entry functions included, add another test function to make sure they behave as expected.

```move
    #[test]
    fun test_sword_transactions() {
        use mango::test_scenario;

        // create test addresses representing users
        let admin = @0xBABE;
        let initial_owner = @0xCAFE;
        let final_owner = @0xFACE;

        // first transaction to emulate module initialization
        let scenario_val = test_scenario::begin(admin);
        let scenario = &mut scenario_val;
        {
            init(test_scenario::ctx(scenario));
        };
        // second transaction executed by admin to create the sword
        test_scenario::next_tx(scenario, admin);
        {
            // create the sword and transfer it to the initial owner
            sword_create(42, 7, initial_owner, test_scenario::ctx(scenario));
        };
        // third transaction executed by the initial sword owner
        test_scenario::next_tx(scenario, initial_owner);
        {
            // extract the sword owned by the initial owner
            let sword = test_scenario::take_from_sender<Sword>(scenario);
            // transfer the sword to the final owner
            sword_transfer(sword, final_owner, test_scenario::ctx(scenario))
        };
        // fourth transaction executed by the final sword owner
        test_scenario::next_tx(scenario, final_owner);
        {
            // extract the sword owned by the final owner
            let sword = test_scenario::take_from_sender<Sword>(scenario);
            // verify that the sword has expected properties
            assert!(magic(&sword) == 42 && strength(&sword) == 7, 1);
            // return the sword to the object pool
            test_scenario::return_to_sender(scenario, sword)
            // or uncomment the line below to destroy the sword instead
            // test_utils::destroy(sword)
        };
        test_scenario::end(scenario_val);
    }
```

There are some details of the new testing function to pay attention to. The first thing the code does is create some addresses that represent users participating in the testing scenario. The assumption is that there is one game administrator user and two regular users representing players. The test then creates a scenario by starting the first transaction on behalf of the administrator address.
The administrator executes the second transaction. The transaction creates a `sword` where the `initial_owner` is the receiver.

The initial owner then executes the third transaction (passed as an argument to the `test_scenario::next_tx` function), who then transfers the `sword` they now own to the final owner. In pure Move there is no notion of Mango storage; consequently, there is no easy way for the emulated Mango transaction to retrieve it from storage. This is where the `test_scenario` module helps - its `take_from_sender` function allows an address-owned object of a given type (`Sword`) executing the current transaction to be available for Move code manipulation. For now, assume that there is only one such object. In this case, the test transfers the object it retrieves from storage to another address.

:::tip

Transaction effects, such as object creation and transfer become visible only after a given transaction completes. For example, if the second transaction in the running example created a `sword` and transferred it to the administrator's address, it would only become available for retrieval from the administrator's address (via `test_scenario`, `take_from_sender`, or `take_from_address` functions) in the third transaction.

:::

The final owner executes the fourth and final transaction that retrieves the `sword` object from storage and checks if it has the expected properties. Remember, as described in [Testing a package](#testing-a-package), in the pure Move testing scenario, after an object is available in Move code (after creation or retrieval from emulated storage), it cannot simply disappear.

In the pure Move testing function, the function transfers the `sword` object to the fake address to handle the disappearing problem. The `test_scenario` package provides a more elegant solution, however, which is closer to what happens when Move code actually executes in the context of Mango - the package simply returns the `sword` to the object pool using the `test_scenario::return_to_sender` function. For scenarios where returning to the sender is not desirable or if you would like to simply destroy the object, the `test_utils` module also provides the generic `destroy<T>` function, that can be used on any type `T` regardless of its ability. It is advisable to check out other useful functions in the [`test_scenario`](https://github.com/MangoNet-Labs/mango/blob/main/crates/mango-framework/packages/mango-framework/sources/test/test_scenario.move) and [`test_utils`](https://github.com/MangoNet-Labs/mango/blob/main/crates/mango-framework/packages/mango-framework/sources/test/test_utils.move) modules as well.

Run the test command again to see two successful tests for our module:

```bash
BUILDING Mgo
BUILDING MoveStdlib
BUILDING my_first_package
Running Move unit tests
[ PASS    ] 0x0::my_module::test_sword_create
[ PASS    ] 0x0::my_module::test_sword_transactions
Test result: OK. Total tests: 2; passed: 2; failed: 0
```

Now that you have built a tested package, the next topic walks you through publishing your package to a Mango network.
