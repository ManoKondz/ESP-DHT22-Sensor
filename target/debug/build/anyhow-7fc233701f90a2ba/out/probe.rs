
    #![feature(error_generic_member_access)]

    use std::backtrace::Backtrace;
    use std::error::{self, Error, Request};
    use std::fmt::{self, Debug, Display};

    struct MyError(Thing);
    struct Thing;

    impl Debug for MyError {
        fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
            unimplemented!()
        }
    }

    impl Display for MyError {
        fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
            unimplemented!()
        }
    }

    impl Error for MyError {
        fn provide<'a>(&'a self, request: &mut Request<'a>) {
            request.provide_ref(&self.0);
        }
    }

    const _: fn(&dyn Error) -> Option<&Backtrace> = |err| error::request_ref::<Backtrace>(err);
