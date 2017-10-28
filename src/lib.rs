

pub use self::ro_scalar_set::RoScalarSet;
pub use self::ro_scalar_set::Value;
pub mod ro_scalar_set;

#[cfg(test)]
mod tests {

    use std;
    use std::iter::FromIterator;
    use ro_scalar_set;

    #[test]
    fn empty_set_is_empty_i32() {
        
        // Run the test.
        empty_set_is_empty::<i32>();
    }

    #[test]
    fn set_member_is_found_i32() {
        
        // Prepare the test.
        let values: [i32; 1] = [5];
        let values_in_set = [ 5 ];
        let negative_test = [ 4, 6 ];

        // Run the test.
        set_member_is_found( &values, &values_in_set, &negative_test );
    }

    #[test]
    fn members_from_different_buckets_are_found_i32() {

        // Prepare test set.
        let values: Vec<i32> = (0..25).collect();
        let negative_test = [ 26, 27 ];

        // Run the test.
        members_from_different_buckets_are_found( &values, 2, &values, &negative_test );
    }

    #[test]
    fn attaching_to_buffer_succeeds_i32() {

        // Prepare test set.
        let buffer = [ 1, 1, 4, 5, 1 ];
        let values_in_set = [ 1 ];
        let adjacent_values = [ 0, 2 ];

        // Run the test.
        attaching_to_buffer_succeeds( &buffer, &values_in_set, &adjacent_values );
    }

    #[test]
    fn empty_set_is_empty_f32() {

        // Run the test.
        empty_set_is_empty::<f32>();
    }

    #[test]
    fn set_member_is_found_f32() {

        // Prepare the test.
        let values: [f32; 1] = [5.0];
        let values_in_set = [ 5.0 ];
        let negative_test = [ 4.0, 6.0 ];

        // Run the test.
        set_member_is_found( &values, &values_in_set, &negative_test );
    }

    #[test]
    fn members_from_different_buckets_are_found_f32() {

        // Prepare test set.
        let values: Vec<i32> = (0..25).collect();
        let values: Vec<f32> = Vec::from_iter( values.into_iter().map( |x| x as f32 ) );
        let negative_test = [ 26.0, 27.0 ];

        // Run the test.
        members_from_different_buckets_are_found( &values, 2, &values, &negative_test );
    }

    #[test]
    fn attaching_to_buffer_succeeds_f32() {

        // Prepare test set.
        let buffer = [ 1.0, 1.0, 4.0, 5.0, 1.0 ];
        let values_in_set = [ 1.0 ];
        let adjacent_values = [ 0.0, 2.0 ];

        // Run the test.
        attaching_to_buffer_succeeds( &buffer, &values_in_set, &adjacent_values );
    }

    /// Vefiries that empty set is empty.
    fn empty_set_is_empty< T >()
        where T: ro_scalar_set::Value + std::clone::Clone {

        let values: Vec<T> = Vec::new();
        let set = ro_scalar_set::RoScalarSet::new( &values );
        assert_eq!( set.size(), 0 );
    }

    // Vefiries that finding a value from a set with one value succeeds.
    fn set_member_is_found<T>(
        values: &[T],
        values_in_set: &[T],
        negative_test: &[T],
    ) where T: ro_scalar_set::Value + std::clone::Clone {

        // This should be a simple test.
        assert_eq!( values.len(), 1  );

        let set = ro_scalar_set::RoScalarSet::new( values );
        assert_eq!( set.size(), 1 );

        // Assert contents.
        assert( &set, values_in_set, negative_test );
    }

    /// Verifies that finding values from a set with multiple buckets succeeds.
    fn members_from_different_buckets_are_found< T >(
        values: &Vec<T>,
        buckets: usize,
        values_in_set: &[T],
        negative_test: &[T],
    ) where T: ro_scalar_set::Value + std::clone::Clone {

        // This set should verify that more than one bucket is used.
        assert!( buckets > 1 );

        // Create the set.
        let set = ro_scalar_set::RoScalarSet::new( &values );
        assert_eq!( set.size(), values.len() );
        assert_eq!( set.bucket_count(), buckets );

        assert( &set, values_in_set, negative_test );
    }

    /// Checks that attaching to an existing buffer succeeds.
    fn attaching_to_buffer_succeeds< T >(
        buffer: &[T],
        values_in_set: &[T],
        negative_test: &[T],
    ) where T: ro_scalar_set::Value + std::clone::Clone {

        // Attach to a buffer.
        let set = ro_scalar_set::RoScalarSet::attach( &buffer ).unwrap().0;

        // Assert.
        assert( &set, values_in_set, negative_test );
    }

    /// Asserts the existance of values in a set.
    fn assert< T >(
        set: &ro_scalar_set::RoScalarSet< T >,
        values_in_set: &[T],
        negative_test: &[T],
    ) where T: ro_scalar_set::Value + std::clone::Clone {

        // Ensure the single member in the set is found.
        for v in values_in_set {
            assert!( set.contains( v ));
        }

        // Negative test, i.e. test for values that should not be in the set.
        for nv in negative_test {
            assert!( ! set.contains( nv ));
        }
    }
}
