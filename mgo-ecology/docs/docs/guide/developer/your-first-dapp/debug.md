---
title: Debugging
description: Debugging
sidebar_position: 4
---


At the moment, there isn't a debugger for Move. To aid with debugging, however, you can use the `std::debug` module to print out arbitrary values. To do so, first import the debug module in your source file:

```move
use std::debug;
```

Then in places where you want to print out a value `v`, regardless of its type, add the following code:

```move
debug::print(&v);
```

or the following if `v` is already a reference:

```move
debug::print(v);
```

The debug module also provides a function to print out the current stacktrace:

```move
debug::print_stack_trace();
```

Alternatively, any call to abort or assertion failure also prints the stacktrace at the point of failure.
