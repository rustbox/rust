#![allow(incomplete_features)]
#![feature(adt_const_params)]

#[derive(PartialEq, Eq)]
struct ImplementsConstParamTy;
impl std::marker::ConstParamTy for ImplementsConstParamTy {}

struct CantParam(ImplementsConstParamTy);

impl std::marker::ConstParamTy for CantParam {}
//~^ error: the type `CantParam` does not `#[derive(Eq)]`

#[derive(std::marker::ConstParamTy)]
//~^ error: the type `CantParamDerive` does not `#[derive(Eq)]`
struct CantParamDerive(ImplementsConstParamTy);

fn check<T: std::marker::ConstParamTy>() {}

fn main() {
    check::<ImplementsConstParamTy>();
}
