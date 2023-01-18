#[macro_export] macro_rules! multidex {
    (
         name => $name:ident, 
         item => $item:ident, 

        $(hashed_unique     => [ $(($tag_a:ident, $sortby_a:ident,   $hasher_a:ident)),* ])?

        $(ordered_unique    => [ $(($tag_b:ident, $identity_b:ident, $comparator_b:ident)),* ])?

        $(ordered_nonunique => [ $(($tag_c:ident, $identity_c:ident, $comparator_c:ident)),* ])?

    ) => {

    }
}

