use std;

/// The remainder of the
pub trait Value {

    /// Gets the index of the bucket for this value.
    fn get_bucket_index( &self, buckets: &Self ) -> usize;

    /// Gets a zero.
    fn zero() -> Self;    

    /// Gets the integer value as a size.
    fn as_index( &self ) -> usize;

    /// Converts the number of buckets into the value type.
    fn from_bucket_count( bucket_count: &usize ) -> Self;

    /// Converts the number of set members into the value type.
    fn from_member_count( member_count: usize ) -> Self;

    /// Converts the given index into the value type.
    fn from_index( index: usize ) -> Self;

    /// Decrements the value by one.
    fn decrement( &self ) -> Self;                  
}

/// Implements value trait for i32
impl Value for i32
{
    /// Gets the index of the bucket for this value.
    fn get_bucket_index( &self, buckets: &i32 ) -> usize {
        return ( self.clone() as usize ) % ( buckets.clone() as usize ) ;
    }

    /// Gets a zero.
    fn zero() -> i32 { return 0; }

    /// Gets the integer value as a size.
    fn as_index( &self ) -> usize { return self.clone() as usize; }

    /// Converts the specified value to a value that can be stored in the container.
    fn from_bucket_count( bucket_count: &usize ) -> i32 { return bucket_count.clone() as i32; }

    /// Converts the number of set members into the value type.
    fn from_member_count( member_count: usize ) -> i32 { return member_count as i32; }

    /// Converts the given value to index.
    fn from_index( index: usize ) -> i32 { return index as i32; }    

    /// Decrements the value by one.
    fn decrement( &self ) -> i32 { return self - 1; }
}

/// The index of the number of members in the set.
const SIZE_INDEX: usize = 1;

/// The index of the first bucket.
const FIRST_BUCKET_INDEX: usize = 2;

/// Defines how the data is stored in the scalar set.
 enum Storage<'a, TA, TB> 
    where TB: 'a {

    /// The data is stored as a vector.
    Vector { data: Vec<TA> },

    /// The data is stored as a slice.
    Slice { data: &'a[TB] }
 }

#[allow(dead_code)]
pub struct RoScalarSet<'a, T>
where T: std::cmp::Ord + std::clone::Clone + Value + 'a {    
    _storage: Storage<'a, T, T>
}

impl<'a, T> RoScalarSet<'a, T>
    where T: std::cmp::Ord + std::clone::Clone + Value + 'a {    

    /// Returns a new integer hash set holding the specified values.
    ///
    /// # ArgumentRoScalarSets
    ///
    /// * 'values' Holds the values stored in the hash set.
    #[allow(dead_code)]
    pub fn new (
            values: &[T]
    ) -> RoScalarSet<T> {
            
        // Determine the number of buckets. We introduce a 5% overhead.
        let bucket_count: usize = ( values.len() as f64 * 0.05 ).ceil() as usize;        
        let mut storage: Vec<T> = vec!( T::zero(); ( FIRST_BUCKET_INDEX + bucket_count + 1 + values.len() ) );
    
        // Store the number of buckets.
        let buckets = T::from_bucket_count( &bucket_count );
        storage[ 0 ] = buckets.clone();

        // Store the number of members.
        storage[ SIZE_INDEX ] = T::from_member_count( values.len() );
        
        // Count the values required for each bucket.
        let mut values_in_buckets: Vec<i32> = vec!( 0; bucket_count );
        for v in values {            
            let bucket = v.get_bucket_index( &buckets );
            values_in_buckets[ bucket ] += 1;
        }
        
        // Set bucket pointers to point the end of each bucket.
        // They will be decrements one-by-one when the buckets are filled.
        let first_bucket: usize = FIRST_BUCKET_INDEX;
        let data_start: usize = FIRST_BUCKET_INDEX + bucket_count + 1;
        let mut previous_bucket_end = data_start;
        for b in 0..bucket_count  {

            // The end of the previous bucket is the start of the previous bucket.
            let index = previous_bucket_end + values_in_buckets[ b ].as_index();
            storage[ b + first_bucket ] = T::from_index( index );
            previous_bucket_end = index;
        }

        // Fix the end of the last bucket.
        storage[ first_bucket + bucket_count ] = T::from_index( storage.len() );
        
        // Put values into buckets.
        for v in values {
            
            // Determine bucket for the value.
            let bucket_id = v.get_bucket_index( &buckets );
            
            // Make room for the new value.
            storage[ bucket_id + first_bucket ] = storage[ bucket_id + first_bucket ].decrement();
            let value_index: usize = storage[ bucket_id + first_bucket ].as_index();
            storage[ value_index ] = v.clone();
        }
        
        // Sort each bucket to enable binary search.
        for b in 0..bucket_count  {
        
            // Determine the location of the bucket.
            let begin: usize = storage[ b + first_bucket ].as_index();
            let end: usize = storage[ b + first_bucket + 1 ].as_index();
            if end < begin {
                panic!( "Invalid bucket: {}", b );
            }
            
            // Get a splice for sorting.
            let ( _, remainder ) =  storage.split_at_mut( begin );
            let ( bucket,  _ )  = remainder.split_at_mut( end - begin );
            bucket.sort();
        }        

        let storage: Storage<T, T> = Storage::Vector { data: storage };
        return RoScalarSet { _storage: storage  };
    }

    /// Attaches an integer hash set to a slice holding the values.
    ///
    /// # ArgumentRoScalarSets
    ///
    /// * 'values' Holds the values stored in the hash set.
    #[allow(dead_code)]
    pub fn attach (
            buffer: &[T]
    ) -> RoScalarSet<T> {
        let storage: Storage<T, T> = Storage::Slice { data: buffer };
        return RoScalarSet { _storage: storage  };
    }

    /// Checks whether the given value exists in the set or not.
    #[allow(dead_code)]
    pub fn contains(
        &self,
        value: T
    ) -> bool {
        
        // Get the bucket associated with the value.
        let bucket = self.get_bucket( &value );
        
        // Locate the value.
        match bucket.binary_search( &value ) {
            Ok( _ ) => return true,
            Err(_) => return false,
        }        
    }

    /// Gets the number of members in the set.
    #[allow(dead_code)]
    pub fn size(
        &self
    ) -> usize {
        let storage = self.borrow_storage();
        return storage[ SIZE_INDEX ].as_index();
    }

    /// Gets the number of buckets.
    #[allow(dead_code)]
    pub fn bucket_count(
        &self
    ) -> usize {
        let storage = self.borrow_storage();
        return storage[ 0 ].as_index();
    }
    
    /// Gets a read-only slice containing the values of a bucket.
    fn get_bucket(
        &self,
        value: &T
    ) -> &[T] {
        
        // Determine the bucket.
        let storage = self.borrow_storage();
        let bucket_id: usize = value.get_bucket_index( &storage[ 0 ] );
        let bucket_index = bucket_id + FIRST_BUCKET_INDEX;
        let begin: usize = storage[ bucket_index ].as_index();
        let end: usize = storage[ ( bucket_index + 1 ) ].as_index();        
        let  ( _, remainder ) = storage.split_at( begin );
        let  ( bucket, _ )  = remainder.split_at( end - begin );
        return bucket;        
    }


    /// Borrows the storage for accessing the values.
    fn borrow_storage(
        &'a self
    ) -> &'a [T] {
        let s: &'a [T] = match &self._storage {
            &Storage::Vector { ref data } => data.as_slice(),
            &Storage::Slice { ref data } => data,
       
        };
        return s;
    }
}
