//! A small utility function to perform automatic collections with [`bacon_rajan_cc`](../bacon_rajan_cc/index.html).

pub extern crate bacon_rajan_cc;

use bacon_rajan_cc::{
    Cc,
    Trace,
    number_of_roots_buffered,
    collect_cycles,
};

const CC_MAX_ROOTS: usize = 128;

/// Wraps [`Cc::new`](struct.Cc.html) with some logic to automatically track the number of root objects in a garbage cycle
/// and automatically collecting them when needed.
/// 
/// This function allows a maximum of `128` buffered roots before triggering a collection.
/// 
/// This function will perform a collection _before_ allocating a new [`Cc::new`](struct.Cc.html),
/// so it is recommended to manually call [`collect_cycles`](collect/fn.collect_cycles.html) after any code
/// where cycles are likely to be created has finished to perform final cleanup.
/// 
/// This function is meant to be a drop-in replacement for [`Cc::new`](struct.Cc.html) where needed,
/// but does not modify or otherwise touch [`Cc<T>`](struct.Cc.html).
/// 
/// [`Cc::new`](struct.Cc.html) should be preferred unless it is known that an arbitrary number of cycles
/// are likely to be created outside of the programmer's control and cleaning them up during normal
/// execution is desirable.
/// 
/// # Example
/// ```rust
/// use auto_cc::cc;
/// 
/// let x = cc( 42u8 );
/// ```
#[inline( always )]
pub fn cc<T: Trace>( value: T ) -> Cc<T> {
    if number_of_roots_buffered() >= CC_MAX_ROOTS {
        collect_cycles();
    }

    Cc::new( value )
}


#[cfg( test )]
mod tests {
    use std::cell::RefCell;

    use super::{
        CC_MAX_ROOTS,

        Trace,
        Cc,
        cc,

        bacon_rajan_cc::{
            Tracer,
            number_of_roots_buffered,
            collect_cycles
        },
    };

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
        for _ in 0 .. CC_MAX_ROOTS / 2 {
            create_cycle();
        }

        // verify we have all of our dead cycles
        assert_eq!( number_of_roots_buffered(), CC_MAX_ROOTS, "before collection" );

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
