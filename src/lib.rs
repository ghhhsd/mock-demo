mod arch;
mod stub;
mod mock;



#[macro_export]
macro_rules! stub_sync_fn {
    ($orig_func:path,$mock_func:path ,$($arg:tt)*) => {
        {
            use std::ffi::c_void;
            use crate::mock::Mock;
            let orig_func = $orig_func as *mut c_void;
            let mock_func = $mock_func as *mut c_void;
            Mock::add_mock(orig_func,mock_func)
        }
    };
}
#[macro_export]
macro_rules! stub_sync_fn1 {
    ($orig_func:path,$mock_func:path ) => {
        stub_sync_fn!($orig_func,$mock_func,A0)
    };
}
#[macro_export]
macro_rules! stub_sync_fn2 {
    ($orig_func:path,$mock_func:path ) => {
        stub_sync_fn!($orig_func,$mock_func,A0,A1)
    };
}



#[macro_export]
macro_rules! stub_async_fn {
    ($orig_func:path,$mock_func:path ,$($arg:tt)*) => {
        {
            use std::ffi::c_void;
            use std::future::Future;
            use crate::mock::Mock;

            fn get_poll_fn_pointer<$($arg)* F:Fn($($arg)*) -> Fut,Fut:Future<Output=O>, O>(_:F) -> *mut c_void {
                Fut::poll as *mut c_void
            }

            let orig_func = $orig_func as *mut c_void;
            let mock_func = $mock_func as *mut c_void;

            let orig_poll_func = get_poll_fn_pointer($orig_func);
            let mock_poll_func = get_poll_fn_pointer($mock_func);

            Mock::add_async_mock(orig_func,mock_func,orig_poll_func,mock_poll_func)
        }

    };
}


#[macro_export]
macro_rules! stub_async_fn0 {
    ($orig_func:path,$mock_func:path) => {
        stub_async_fn!($orig_func,$mock_func,)
    }
}
#[macro_export]
macro_rules! stub_async_fn1 {
    ($orig_func:path,$mock_func:path) => {
         stub_async_fn!($orig_func,$mock_func,A0,)
    }
}
#[macro_export]
macro_rules! stub_async_fn2 {
    ($orig_func:path,$mock_func:path) => {
         stub_async_fn!($orig_func,$mock_func,A0,A1,)
    }
}


#[cfg(test)]
mod demo_with_two_param {
    #[test]
    fn stub_test() {
        fn demo_without_param() -> i32 { 11 }
        fn demo_without_param1() -> i32 { 33 }
        let mock = stub_sync_fn1!(demo_without_param,demo_without_param1);
        assert!(mock.is_ok());
        assert_eq!(demo_without_param(), 33);
        let res = mock.unwrap().remove_mock();
        assert!(res.is_ok());
        assert_eq!(demo_without_param(), 11);


        fn demo_with_one_param(num: i32) -> i32 { num }

        fn demo_with_one_param1(num: i32) -> i32 { num + 22 }
        let mock = stub_sync_fn1!(demo_with_one_param,demo_with_one_param1);
        assert!(mock.is_ok());
        assert_eq!(demo_with_one_param(11), 33);
        let res = mock.unwrap().remove_mock();
        assert!(res.is_ok());
        assert_eq!(demo_with_one_param(11), 11);


        fn demo_with_two_param(num: i32, num1: i32) -> i32 { num + num1 }
        fn demo_with_two_param1(num: i32, num1: i32) -> i32 { num + num1 + 11 }
        let mock = stub_sync_fn2!(demo_with_two_param,demo_with_two_param1);
        assert!(mock.is_ok());
        assert_eq!(demo_with_two_param(11, 11), 33);
        let res = mock.unwrap().remove_mock();
        assert!(res.is_ok());
        assert_eq!(demo_with_two_param(11, 11), 22);
    }

    #[tokio::test]
    async fn stub_async_test() {
        async fn demo_without_param() -> i32 { 11 }
        async fn demo_without_param1() -> i32 { 33 }
        let mock = stub_async_fn0!(demo_without_param,demo_without_param1);
        assert!(mock.is_ok());
        assert_eq!(demo_without_param().await, 33);
        let res = mock.unwrap().remove_mock();
        assert!(res.is_ok());
        assert_eq!(demo_without_param().await, 11);


        async fn demo_with_one_param(num: i32) -> i32 { num }

        async fn demo_with_one_param1(num: i32) -> i32 { num + 22 }
        let mock = stub_async_fn1!(demo_with_one_param,demo_with_one_param1);
        assert!(mock.is_ok());
        assert_eq!(demo_with_one_param(11).await, 33);
        let res = mock.unwrap().remove_mock();
        assert!(res.is_ok());
        assert_eq!(demo_with_one_param(11).await, 11);


        async fn demo_with_two_param(num: i32, num1: i32) -> i32 { num + num1 }

        async fn demo_with_two_param1(num: i32, num1: i32) -> i32 { num + num1 + 11 }
        let mock = stub_async_fn2!(demo_with_two_param,demo_with_two_param1);
        assert!(mock.is_ok());
        assert_eq!(demo_with_two_param(11, 11).await, 33);
        let res = mock.unwrap().remove_mock();
        assert!(res.is_ok());
        assert_eq!(demo_with_two_param(11, 11).await, 22);
    }
}