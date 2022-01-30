use crate::translate::*;
use crate::{StaticType, Type};

pub trait PointerType: StaticType + Clone + Sized + 'static {
    // rustdoc-stripper-ignore-next
    /// Pointer type name.
    ///
    /// This must be unique in the whole process.
    const NAME: &'static str;
}

// rustdoc-stripper-ignore-next
/// Register a `Type` ID for `*mut T`.
#[doc(alias = "g_pointer_type_register_static")]
pub fn register_pointer_type<T>() -> Type
where
    *mut T: PointerType,
{
    unsafe {
        from_glib(gobject_ffi::g_pointer_type_register_static(
            <*mut T as PointerType>::NAME.to_glib_none().0,
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::subclass::pointer::PointerType;
    use crate::subclass::register_pointer_type;
    use crate::translate::*;
    use crate::value::{ToValue, ValueType};
    use crate::{Pointer, StaticType, Type};
    use std::ptr::{self, NonNull};

    struct MyType;

    // XXX: The following is for demonstration purposes only,
    // and {c,sh}ould be generated by a macro instead
    type MyPointer = *mut MyType;

    impl PointerType for MyPointer {
        const NAME: &'static str = "MyType";
    }

    impl StaticType for MyPointer {
        fn static_type() -> Type {
            static TYPE: once_cell::sync::Lazy<Type> =
                once_cell::sync::Lazy::new(|| register_pointer_type::<MyType>());
            *TYPE
        }
    }

    impl ValueType for MyPointer {
        type Type = Self;
    }

    impl FromGlib<ffi::gpointer> for MyPointer {
        unsafe fn from_glib(val: ffi::gpointer) -> Self {
            val as Self
        }
    }

    impl IntoGlib for MyPointer {
        type GlibType = MyPointer;

        fn into_glib(self) -> Self::GlibType {
            self as _
        }
    }

    #[test]
    fn test_register() {
        assert!(MyPointer::static_type().is_valid());
    }

    #[test]
    fn test_value() {
        let mut b = MyType;
        let p: MyPointer = &mut b;
        let v = p.to_value();
        let p2 = v.get::<MyPointer>().unwrap();
        assert_eq!(p, p2);

        let p3 = v.get::<Pointer>().unwrap();
        assert_eq!(p as *mut _, p3);

        let p4 = v.get::<Option<NonNull<MyType>>>().unwrap();
        assert_eq!(NonNull::new(p), p4);

        let v_null = ptr::null_mut::<MyType>().to_value();
        let p5 = v_null.get::<Option<NonNull<MyType>>>().unwrap();
        assert_eq!(None, p5);

        let p6 = v_null.get::<NonNull<MyType>>();
        assert_eq!(p5, p6.ok());

        assert!(v.is::<MyPointer>());
        assert!(v.is::<Pointer>());

        assert_eq!(v.type_().name(), MyPointer::NAME);
    }
}
