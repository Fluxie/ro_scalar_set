extern crate byteorder;

use std;
use std::io::Write;
use self::byteorder::NativeEndian;
use self::byteorder::WriteBytesExt;

/// Represents a value in the scalar set.
/// The values must be able to store booking information
/// for managing the contents of the set. These requirements
/// are captured in this trait.
pub trait Value
{
    /// Gets the index of the bucket for this value.
    fn get_bucket_index(
        &self,
        buckets: &Self,
    ) -> usize;

    /// Gets a zero.
    fn zero() -> Self;

    /// Gets the integer value as a size.
    fn as_index( &self ) -> usize;

    /// Converts the number of buckets into the value type.i
    fn from_bucket_count( bucket_count: &usize ) -> Self;

    /// Converts the number of set members into the value type.
    fn from_member_count( member_count: usize ) -> Self;

    /// Converts the given index into the value type.
    fn from_index( index: usize ) -> Self;

    /// Decrements the value by one.
    fn decrement( &self ) -> Self;

    /// Provides total ordering for elements.
    fn cmp(
        &self,
        other: &Self,
    ) -> std::cmp::Ordering;

    /// Serializes the value.
    fn serialize(
        &self,
        &mut Write,
    ) -> Result<(), std::io::Error>;
}

/// Implements Value trait for i32
impl Value for i32
{
    /// Gets the index of the bucket for this value.
    fn get_bucket_index(
        &self,
        buckets: &i32,
    ) -> usize
    {
        return ( self.clone() as usize ) % ( buckets.clone() as usize );
    }

    /// Gets a zero.
    fn zero() -> i32
    {
        return 0;
    }

    /// Gets the integer value as a size.
    fn as_index( &self ) -> usize
    {
        return self.clone() as usize;
    }

    /// Converts the specified value to a value that can be stored in the container.
    fn from_bucket_count( bucket_count: &usize ) -> i32
    {
        return bucket_count.clone() as i32;
    }

    /// Converts the number of set members into the value type.
    fn from_member_count( member_count: usize ) -> i32
    {
        return member_count as i32;
    }

    /// Converts the given value to index.
    fn from_index( index: usize ) -> i32
    {
        return index as i32;
    }

    /// Decrements the value by one.
    fn decrement( &self ) -> i32
    {
        return self - 1;
    }

    /// Provides total ordering for elements.
    fn cmp(
        &self,
        other: &i32,
    ) -> std::cmp::Ordering
    {
        std::cmp::Ord::cmp( self, other )
    }

    /// Serializes the value.
    fn serialize(
        &self,
        writer: &mut Write,
    ) -> Result<(), std::io::Error>
    {
        writer.write_i32::<NativeEndian>( *self )?;
        Ok( () )
    }
}

/// Implements Value trait for f32
/// * The largest permissible value is 2^24.
/// * Mainly implemented to provide an easy to use interface for sending the structure to GPU.
#[cfg(any(feature = "floats", test))]
impl Value for f32
{
    /// Gets the index of the bucket for this value.
    fn get_bucket_index(
        &self,
        buckets: &f32,
    ) -> usize
    {
        return ( self.clone() as usize ) % ( buckets.clone() as usize );
    }

    /// Gets a zero.
    fn zero() -> f32
    {
        return 0.0;
    }

    /// Gets the integer value as a size.
    fn as_index( &self ) -> usize
    {
        return self.clone() as usize;
    }

    /// Converts the specified value to a value that can be stored in the container.
    fn from_bucket_count( bucket_count: &usize ) -> f32
    {
        return bucket_count.clone() as f32;
    }

    /// Converts the number of set members into the value type.
    fn from_member_count( member_count: usize ) -> f32
    {
        return member_count as f32;
    }

    /// Converts the given value to index.
    fn from_index( index: usize ) -> f32
    {
        return index as f32;
    }

    /// Decrements the value by one.
    fn decrement( &self ) -> f32
    {
        return self - 1.0;
    }

    /// Provides total ordering for elements.
    fn cmp(
        &self,
        other: &f32,
    ) -> std::cmp::Ordering
    {
        let me = self.clone() as i32;
        let other = other.clone() as i32;
        std::cmp::Ord::cmp( &me, &other )
    }

    /// Serializes the value.
    fn serialize(
        &self,
        writer: &mut Write,
    ) -> Result<(), std::io::Error>
    {
        writer.write_f32::<NativeEndian>( *self )?;
        Ok( () )
    }
}

/// The index of the number of members in the set.
const SIZE_INDEX: usize = 1;

/// The index of the first bucket.
const FIRST_BUCKET_INDEX: usize = 2;

/// Defines how the data is stored in the scalar set.
enum Storage<'a, TA, TB>
where
    TB: 'a,
{
    /// The data is stored as a vector.
    Vector
    { data: Vec<TA> },

    /// The data is stored as a slice.
    Slice
    { data: &'a [TB] },
}

/// Scalar set cabable of storing values with Value trait.
pub struct RoScalarSet<'a, T>
where
    T: std::clone::Clone + Value + 'a,
{
    _storage: Storage<'a, T, T>,
}

impl<'a, T> Clone for RoScalarSet<'a, T>
where
    T: std::clone::Clone + Value + 'a,
{
    /// Returns a copy of the value.
    fn clone( &self ) -> Self
    {
        RoScalarSet::new( self.borrow_values() )
    }

    /// Performs copy-assignment from source.
    fn clone_from(
        &mut self,
        source: &Self,
    )
    {
        let storage = RoScalarSet::create_storage( source.borrow_values() );
        self._storage = storage;
    }
}

impl<'a, T> RoScalarSet<'a, T>
where
    T: std::clone::Clone + Value + 'a,
{
    /// Returns a new integer hash set holding the specified values.
    ///
    /// # ArgumentRoScalarSets
    ///
    /// * 'values' Holds the values stored in the hash set.
    pub fn new<'b, 'c>( values: &'c [T] ) -> RoScalarSet<'b, T>
    {
        let storage = RoScalarSet::create_storage( values );
        return RoScalarSet { _storage: storage };
    }

    /// Attaches an integer hash set to a slice holding the values.
    ///
    /// # ArgumentRoScalarSets
    ///
    /// * 'values' Holds the values stored in the hash set.
    pub fn attach<'b>( buffer: &'b [T] ) -> Result<( RoScalarSet<'b, T>, &'b [T] ), &str>
    {
        // Determine the length of this scalar set.
        if buffer.len() < 4
        {
            return Err( "The buffer is to small" );
        }
        let buckets = buffer[0].as_index();
        let values = buffer[SIZE_INDEX].as_index();
        let total_size = FIRST_BUCKET_INDEX + ( buckets + 1 ) + values;

        // Attach.
        let ( set, remainder ) = buffer.split_at( total_size );
        let storage: Storage<T, T> = Storage::Slice { data: set };
        return Ok( ( RoScalarSet { _storage: storage }, remainder ) );
    }

    /// Checks if any the sets have anything in common.
    pub fn any(
        &self,
        other: &RoScalarSet<T>,
    ) -> bool
    {
        // Determine the smaller set.
        let small;
        let large;
        if other.size() < self.size()
        {
            small = other;
            large = self;
        }
        else
        {
            small = self;
            large = other;
        }

        // For better performance we iterate the smaller set.
        for s in small.borrow_values()
        {
            if large.contains( s )
            {
                return true;
            }
        }

        // Values was not found.
        return false;
    }

    /// Checks whether the given value exists in the set or not.
    pub fn contains(
        &self,
        value: &T,
    ) -> bool
    {
        // Get the bucket associated with the value.
        let bucket = self.get_bucket( &value );

        // Locate the value.
        match bucket.binary_search_by( |probe| probe.cmp( &value ) )
        {
            Ok( _ ) => return true,
            Err( _ ) => return false,
        }
    }

    /// Gets the number of members in the set.
    pub fn size( &self ) -> usize
    {
        let storage = self.borrow_storage();
        return storage[SIZE_INDEX].as_index();
    }

    /// Gets the number of buckets.
    pub fn bucket_count( &self ) -> usize
    {
        let storage = self.borrow_storage();
        return storage[0].as_index();
    }

    /// Serializes the scalar set into the given writer.
    pub fn serialize<W>(
        &self,
        writer: &mut W,
    ) -> Result<(), std::io::Error>
    where
        W: Write,
    {
        // Write our intern
        let storage = self.borrow_storage();
        for v in storage
        {
            v.serialize( writer )?;
        }
        Ok( () )
    }

    /// Gets a read-only slice containing the values of a bucket.
    fn get_bucket(
        &self,
        value: &T,
    ) -> &[T]
    {
        // Determine the bucket.
        let storage = self.borrow_storage();
        let bucket_id: usize = value.get_bucket_index( &storage[0] );
        let bucket_index = bucket_id + FIRST_BUCKET_INDEX;
        let begin: usize = storage[bucket_index].as_index();
        let end: usize = storage[( bucket_index + 1 )].as_index();
        let ( _, remainder ) = storage.split_at( begin );
        let ( bucket, _ ) = remainder.split_at( end - begin );
        return bucket;
    }

    /// Borrows the values of the set.
    fn borrow_values( &'a self ) -> &'a [T]
    {
        // Split the value part from the storage.
        let start = FIRST_BUCKET_INDEX + self.bucket_count() + 1;
        let ( _, values ) = self.borrow_storage().split_at( start );
        return values;
    }


    /// Borrows the storage for accessing the values.
    fn borrow_storage( &'a self ) -> &'a [T]
    {
        let s: &'a [T] = match &self._storage
        {
            &Storage::Vector { ref data } => data.as_slice(),
            &Storage::Slice { ref data } => data,
        };
        return s;
    }

    /// Creates a new storage buffer from the given values.
    fn create_storage( values: &[T] ) -> Storage<'a, T, T>
    {
        // Determine the number of buckets. We introduce a 5% overhead.
        let bucket_count: usize = ( values.len() as f64 * 0.05 ).ceil() as usize;
        let mut storage: Vec<T> = vec![T::zero(); ( FIRST_BUCKET_INDEX + bucket_count + 1 + values.len() )];

        // Store the number of buckets.
        let buckets = T::from_bucket_count( &bucket_count );
        storage[0] = buckets.clone();

        // Store the number of members.
        storage[SIZE_INDEX] = T::from_member_count( values.len() );

        // Count the values required for each bucket.
        let mut values_in_buckets: Vec<i32> = vec![0; bucket_count];
        for v in values
        {
            let bucket = v.get_bucket_index( &buckets );
            values_in_buckets[bucket] += 1;
        }

        // Set bucket pointers to point the end of each bucket.
        // They will be decrements one-by-one when the buckets are filled.
        let first_bucket: usize = FIRST_BUCKET_INDEX;
        let data_start: usize = FIRST_BUCKET_INDEX + bucket_count + 1;
        let mut previous_bucket_end = data_start;
        for b in 0..bucket_count
        {
            // The end of the previous bucket is the start of the previous bucket.
            let index = previous_bucket_end + values_in_buckets[b].as_index();
            storage[b + first_bucket] = T::from_index( index );
            previous_bucket_end = index;
        }

        // Fix the end of the last bucket.
        storage[first_bucket + bucket_count] = T::from_index( storage.len() );

        // Put values into buckets.
        for v in values
        {
            // Determine bucket for the value.
            let bucket_id = v.get_bucket_index( &buckets );

            // Make room for the new value.
            storage[bucket_id + first_bucket] = storage[bucket_id + first_bucket].decrement();
            let value_index: usize = storage[bucket_id + first_bucket].as_index();
            storage[value_index] = v.clone();
        }

        // Sort each bucket to enable binary search.
        for b in 0..bucket_count
        {
            // Determine the location of the bucket.
            let begin: usize = storage[b + first_bucket].as_index();
            let end: usize = storage[b + first_bucket + 1].as_index();
            if end < begin
            {
                panic!( "Invalid bucket: {}", b );
            }

            // Get a splice for sorting.
            let ( _, remainder ) = storage.split_at_mut( begin );
            let ( bucket, _ ) = remainder.split_at_mut( end - begin );
            bucket.sort_by( |a, b| a.cmp( b ) );
        }

        let storage: Storage<'a, T, T> = Storage::Vector { data: storage };
        storage
    }
}
