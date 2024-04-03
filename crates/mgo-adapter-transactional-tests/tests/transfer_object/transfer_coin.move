// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

// tests TransferObject with a MGO coin

//# init --accounts A B C

//# programmable --sender C --inputs @A
//> TransferObjects([Gas], Input(0))

//# view-object 0,2

//# transfer-object 0,2 --sender A --recipient B

//# view-object 0,2
