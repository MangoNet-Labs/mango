# To add a new native Move function

1. Add a new `./mgo-framework/{name}.move` file or find an appropriate `.move`.
2. Add the signature of the function you are adding in `{name}.move`. 
3. Add the rust implementation of the function under `./mgo-framework/src/natives` with name `{name}.rs`.
4. Write some tests in `{name}_tests.move` and pass `run_framework_move_unit_tests`.
5. Optionally, run `cargo insta test` and `cargo insta review` since the mgo-framework build will change the empty genesis config.