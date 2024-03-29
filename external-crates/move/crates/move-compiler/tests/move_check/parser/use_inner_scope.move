address 0x2 {
module M {

    fun t() {
        use 0x2::Mgo;
        use 0x2::Mgo as M;
        use 0x2::Mgo::baz;
        use 0x2::Salsa::{Self, foo as bar, foo};
        let x = {
            use 0x2::Mgo;
            use 0x3::Mgo as M;
            use 0x3::Mgo::baz;
            use 0x3::Salsa::{Self, foo as bar, foo};

            0
        }; x;
        {
            {
                {
                    {
                        {
                            {
                                {
                                    {
                                        {
                                            {
                                                use 0x2::Mgo;
                                                use 0x3::Mgo as M;
                                                use 0x3::Mgo::baz;
                                                use 0x3::Salsa::{Self, foo as bar, foo};
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };

        while (true) {
            use 0x2::Mgo;
            use 0x3::Mgo as M;
            use 0x3::Mgo::baz;
            use 0x3::Salsa::{Self, foo as bar, foo};

        }
    }
}
}
