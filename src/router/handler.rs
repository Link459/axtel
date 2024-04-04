use std::{future::Future, marker::PhantomData, pin::Pin};

use crate::http::{
    request::{FromRequest, FromRequestParts, Request},
    response::{IntoResponse, Response},
};

pub trait Handler: Send + Sync {
    //type Future: Future<Output = Response> + Send;
    fn call(&self, request: Request) -> HandlerFuture;
}

pub trait IntoHandler<Input>
where
    Self: Sized,
{
    type Handler: Handler;
    fn into_handler(self) -> Self::Handler;
}

pub struct FunctionHandler<T, F> {
    f: F,
    _phantom_data: PhantomData<fn() -> T>,
}

pub type HandlerFuture = Pin<Box<dyn Future<Output = Response> + Send>>;

macro_rules! impl_function_handler {
    (
        [$($ty:ident),*],$last:ident
    ) => {
        #[allow(non_snake_case,unused_parens,unused_variables)]
        impl<F, Fut,  Res,  $($ty,)* $last> Handler for FunctionHandler<($($ty,)* $last),F>
        where
            F: Fn($($ty,)* $last) -> Fut + Clone +Send + Sync + 'static,
            Fut: Future<Output = Res> + Send ,
            Res: IntoResponse,
            $( $ty: FromRequestParts + Send + Sync  , )*
            $last: FromRequest + Send
        {
            //type Future = HandlerFuture;//Pin<Box<dyn Future<Output = Response> + Send>>;

            fn call(&self, req: Request) -> HandlerFuture {
                let f = self.f.clone();
                Box::pin(async move {
                        let (parts,body) = req.into_parts();
                    $(
                        let $ty = match $ty::from_request_parts(&parts) {
                            Ok(value) => value,
                            //Err(rejection) => return rejection.into_response(),
                            Err(_) => panic!("failed to extract from request"),
                        };
                    )*
                    let req = Request::from_parts(parts,body);
                    let $last = match $last::from_request(req) {
                        Ok(value) => value,
                        Err(_) => panic!("failed to extract from request"),
                    };

                    //let res = (self.f)($($ty,)*).await;
                    let res = (f)($($ty,)* $last).await;

                    res.into_response()
                })
            }
        }
    };
}
impl<F, Fut, Res> Handler for FunctionHandler<(), F>
where
    F: Fn() -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send,
    Res: IntoResponse,
{
    fn call(&self, _req: Request) -> HandlerFuture {
        let f = self.f.clone();
        Box::pin(async move {
            let res = (f)().await;
            res.into_response()
        })
    }
}
//impl_function_handler!([],);
impl_function_handler!([], T1);
impl_function_handler!([T1], T2);
impl_function_handler!([T1, T2], T3);
impl_function_handler!([T1, T2, T3], T4);
impl_function_handler!([T1, T2, T3, T4], T5);
impl_function_handler!([T1, T2, T3, T4, T5], T6);
impl_function_handler!([T1, T2, T3, T4, T5, T6], T7);
impl_function_handler!([T1, T2, T3, T4, T5, T6, T7], T8);
impl_function_handler!([T1, T2, T3, T4, T5, T6, T7, T8], T9);
impl_function_handler!([T1, T2, T3, T4, T5, T6, T7, T8, T9], T10);
impl_function_handler!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10], T11);
impl_function_handler!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], T12);
impl_function_handler!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12], T13);
impl_function_handler!(
    [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13],
    T14
);

macro_rules! impl_into_handler {
    (

        [$($ty:ident),*],$last:ident
    ) => {
        #[allow(unused_parens)]
        impl<Fut,Res ,F,$($ty,)* $last> IntoHandler<( $($ty,)* $last)> for F
            where
                F: Fn($($ty,)* $last) -> Fut + Clone + Sync + Send + 'static,
                Res: IntoResponse,
                Fut: Future<Output = Res> + Send  ,
   				$($ty: FromRequestParts + Send + Sync,)*
                $last: FromRequest + Send + Sync,
        {
            type Handler = FunctionHandler<($($ty,)* $last), Self>;

            fn into_handler(self) -> Self::Handler {
                FunctionHandler {
                    f: self,
                    _phantom_data: Default::default(),
                }
            }
        }
    }
}

impl<Fut, Res, F> IntoHandler<()> for F
where
    F: Fn() -> Fut + Clone + Sync + Send + 'static,
    Res: IntoResponse,
    Fut: Future<Output = Res> + Send,
{
    type Handler = FunctionHandler<(), Self>;

    fn into_handler(self) -> Self::Handler {
        FunctionHandler {
            f: self,
            _phantom_data: Default::default(),
        }
    }
}

impl_into_handler!([], T1);
impl_into_handler!([T1], T2);
impl_into_handler!([T1, T2], T3);
impl_into_handler!([T1, T2, T3], T4);
impl_into_handler!([T1, T2, T3, T4], T5);
impl_into_handler!([T1, T2, T3, T4, T5], T6);
impl_into_handler!([T1, T2, T3, T4, T5, T6], T7);
impl_into_handler!([T1, T2, T3, T4, T5, T6, T7], T8);
impl_into_handler!([T1, T2, T3, T4, T5, T6, T7, T8], T9);
impl_into_handler!([T1, T2, T3, T4, T5, T6, T7, T8, T9], T10);
impl_into_handler!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10], T11);
impl_into_handler!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11], T12);
impl_into_handler!([T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12], T13);
impl_into_handler!(
    [T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13],
    T14
);

pub type BoxedHandler = Box<dyn Handler>;
