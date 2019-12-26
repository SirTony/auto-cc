extern crate bacon_rajan_cc;

pub use bacon_rajan_cc::*;

use std::cell::RefCell;

const ROOT_RATIO:    f64   = 0.75;
const INITIAL_ROOTS: usize = 128;

thread_local!( static MAX_ROOTS: RefCell<usize> = RefCell::new( INITIAL_ROOTS ) );

/// Wraps `Cc::new` with some logic to automatically track the number of allocated
/// objects and perform a collection when needed.
/// 
/// This function will perform a collection _before_ allocating a new `Cc::new`,
/// so it is recommended to manually call `collect_cycles` after any code
/// where cycles are likely to be created has finished to perform final cleanup.
pub fn cc<T: Trace>( value: T ) -> Cc<T> {
    // first, check the number of roots the cycle collector knows about ...
    MAX_ROOTS.with( | roots | {
        let mut roots = roots.borrow_mut();

        // ... if the number of roots meets or exceeds the maximum allowed ...
        if number_of_roots_buffered() >= *roots {
            // ... then perform a collection ...
            collect_cycles();

            // if the number of roots still isn't below threshhold, increase the threshold.
            if ( number_of_roots_buffered() as f64 ) > ( ( *roots as f64 ) * ROOT_RATIO ) {
                *roots = ( ( *roots as f64 ) / ROOT_RATIO ) as usize;
            }
        }
    } );

    // finally, create and return our value
    Cc::new( value )
}

#[cfg( test )]
mod tests {
    use super::*;

    fn create_cycle() {
        struct List( Vec<Cc<RefCell<List>>> );
        impl Trace for List {
            fn trace( &self, tracer: &mut Tracer ) {
                self.0.trace( tracer );
            }
        }

        {
            let a = cc( RefCell::new( List( Vec::new() ) ) );
            let b = cc( RefCell::new( List( Vec::new() ) ) );

            {
                let mut a = a.borrow_mut();
                a.0.push( b.clone() );
            }

            {
                let mut b = b.borrow_mut();
                b.0.push( a.clone() );
            }
        }
    }

    #[test]
    fn auto_collection() {
        // there obviously won't be any roots when we first start
        assert_eq!( number_of_roots_buffered(), 0, "start" );

        // each cycle has 2 items, so we create exactly INITIAL_ROOTS `Cc` objects
        // putting us right at the collection threshold so the next cycle created
        // will trip automatic collection
        for _ in 0 .. INITIAL_ROOTS / 2 {
            create_cycle();
        }

        // verify we have all of our dead cycles
        assert_eq!( number_of_roots_buffered(), INITIAL_ROOTS, "before collection" );

        // creating another cycle should trip automatic collection
        create_cycle();

        // we should have exactly 1 dead cycle, because collection happens before allocation
        assert_eq!( number_of_roots_buffered(), 2, "after collection" );

        // remove the last cycle
        collect_cycles();

        // ensure it's actually gone
        assert_eq!( number_of_roots_buffered(), 0, "finished" );
    }
}
