mod ro_scalar_set;

#[cfg(test)]
mod tests {

    use ro_scalar_set;

    #[test]
    fn empty_set_is_empty() {
        
        let values: [i32; 0] = [];
        let set = ro_scalar_set::RoScalarSet::new( &values );
        assert_eq!( set.size(), 0 );
    }

    #[test]
    fn set_member_is_found() {
        
        let values: [i32; 1] = [5];
        let set = ro_scalar_set::RoScalarSet::new( &values );
        assert_eq!( set.size(), 1 );

        // Ensure the member is found.
        assert!( set.contains( 5 ));

        // Ensure some other potential members are not found.
        assert!( ! set.contains( 4 ));
        assert!( ! set.contains( 6 ));
    }

    #[test]
    fn members_from_different_buckets_are_found() {
        
        // Create the set.
        let values: Vec<i32> = (0..25).collect();
        let set = ro_scalar_set::RoScalarSet::new( &values );
        assert_eq!( set.size(), values.len() );
        assert_eq!( set.bucket_count(), 2 );

        // Ensure the member is found.
        assert!( set.contains( 4 ));
        assert!( set.contains( 5 ));

        // Ensure some other potential members are not found.        
        assert!( ! set.contains( 26 ));
        assert!( ! set.contains( 27 ));
    }
}
