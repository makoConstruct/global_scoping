use std::cell::{Ref, RefCell};


pub struct Contextual<T>{
    pub v: RefCell<T>,
}
impl<T> Contextual<T> {
    pub fn new(value: T) -> Self {
        Self { v: RefCell::new(value) }
    }
    pub fn borrow<'a>(&'a self) -> Ref<'a, T> {
        self.v.borrow()
    }
}

pub fn with_context<T, R>(c:&Contextual<T>, v:T, f:impl FnOnce()-> R)-> R {
    let old = c.v.replace(v);
    let result = f();
    c.v.replace(old);
    result
}

mod test {
    use super::*;
    
    thread_local! {
        static CONTEXT: Contextual<i32> = Contextual::new(9);
    }
    
    #[test]
    fn test_with_context() {
        let c = Contextual::new(0);
        with_context(&c, 1, || {
            assert_eq!(*c.borrow(), 1);
        });
        assert_eq!(*c.borrow(), 0);
    }
    
    #[test]
    fn test_thread_local() {
        // oh... you gotta do the with thing? :/  
        CONTEXT.with(|c| {
            with_context(&c, 3, || {
                CONTEXT.with(|c| {
                    assert_eq!(*c.borrow(), 3);
                });
            });
            assert_eq!(*c.borrow(), 9);
        });
    }
}