//! Declarative macros of Rust to achieve custom derives.

pub fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

/// Do syscall and print the result
#[macro_export]
macro_rules! call {
    ($x: ident) => {
        {
            // println!("{}: {:?}", type_of(&$x), $x);
            println!("---- calling {}: {}", type_of(&$x), serde_json::to_string(&$x).unwrap());
            let res = $x.call();
            println!("---- result from {} syscall: {:?}", type_of(&$x), res);
            res
        }
    };
}

/// Do syscall using libc wrapper and print the result
#[macro_export]
macro_rules! call_libc {
    ($x: ident) => {
        {
            // println!("{}: {:?}", type_of(&$x), $x);
            println!("---- calling {}: {}", type_of(&$x), serde_json::to_string(&$x).unwrap());
            let res = $x.call_libc();
            println!("---- result from {} syscall: {:?}", type_of(&$x), res);
            res
        }
    };
}


// let open = Open::generate(gen);

// let res = call!(open);

/// This macro can **instantiate** a system call (with arguments filled *correctly*) and make the corresponding raw syscall.
#[macro_export]
macro_rules! testcall {
    ($x: ty, $g: ident) => {
        {
            let syscall_ds = <$x>::generate($g);
            
            let res = call!(syscall_ds);
            (syscall_ds, res)
        }
    };
    ($x: ty, $g: ident, $n: expr ) => {
        {
            for i in 0..$n {
                let syscall_ds = <$x>::generate($g);
            
                let res = call!(syscall_ds);
                println!("---- after {}: {}", type_of(&syscall_ds), serde_json::to_string(&syscall_ds).unwrap());
            }
        }
    };
}

/// This macro can **instantiate** a system call (with arguments filled *correctly*) and make the corresponding libc syscall.
#[macro_export]
macro_rules! testcall_libc {
    ($x: ty, $g: ident) => {
        {
            let syscall_ds = <$x>::generate($g);
            
            let res = call_libc!(syscall_ds);
            (syscall_ds, res)
        }
    };
    ($x: ty, $g: ident, $n: expr ) => {
        {
            for i in 0..$n {
                let syscall_ds = <$x>::generate($g);
            
                let res = call_libc!(syscall_ds);
                println!("---- after {}: {}", type_of(&syscall_ds), serde_json::to_string(&syscall_ds).unwrap());
            }
        }
    };
}


