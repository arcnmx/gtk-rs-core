// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use std::fmt;

#[cfg(any(feature = "v1_2", feature = "dox"))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum EulerOrder {
    Default,
    Xyz,
    Yzx,
    Zxy,
    Xzy,
    Yxz,
    Zyx,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v1_2", feature = "dox"))]
impl fmt::Display for EulerOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EulerOrder::{}", match *self {
            EulerOrder::Default => "Default",
            EulerOrder::Xyz => "Xyz",
            EulerOrder::Yzx => "Yzx",
            EulerOrder::Zxy => "Zxy",
            EulerOrder::Xzy => "Xzy",
            EulerOrder::Yxz => "Yxz",
            EulerOrder::Zyx => "Zyx",
            _ => "Unknown",
        })
    }
}

#[cfg(any(feature = "v1_2", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for EulerOrder {
    type GlibType = ffi::graphene_euler_order_t;

    fn to_glib(&self) -> ffi::graphene_euler_order_t {
        match *self {
            EulerOrder::Default => ffi::GRAPHENE_EULER_ORDER_DEFAULT,
            EulerOrder::Xyz => ffi::GRAPHENE_EULER_ORDER_XYZ,
            EulerOrder::Yzx => ffi::GRAPHENE_EULER_ORDER_YZX,
            EulerOrder::Zxy => ffi::GRAPHENE_EULER_ORDER_ZXY,
            EulerOrder::Xzy => ffi::GRAPHENE_EULER_ORDER_XZY,
            EulerOrder::Yxz => ffi::GRAPHENE_EULER_ORDER_YXZ,
            EulerOrder::Zyx => ffi::GRAPHENE_EULER_ORDER_ZYX,
            EulerOrder::__Unknown(value) => value
        }
    }
}

#[cfg(any(feature = "v1_2", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::graphene_euler_order_t> for EulerOrder {
    fn from_glib(value: ffi::graphene_euler_order_t) -> Self {
        skip_assert_initialized!();
        match value {
            -1 => EulerOrder::Default,
            0 => EulerOrder::Xyz,
            1 => EulerOrder::Yzx,
            2 => EulerOrder::Zxy,
            3 => EulerOrder::Xzy,
            4 => EulerOrder::Yxz,
            5 => EulerOrder::Zyx,
            value => EulerOrder::__Unknown(value),
        }
    }
}

