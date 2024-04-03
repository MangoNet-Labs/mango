// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

module type_params::m1 {
    use mgo::transfer;

    public entry fun transfer_object<T: key + store>(o: T, recipient: address) {
        transfer::public_transfer(o, recipient);
    }


}
