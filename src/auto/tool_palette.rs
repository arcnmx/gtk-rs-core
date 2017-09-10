// This file was generated by gir (6a48033) from gir-files (db49619)
// DO NOT EDIT

use Container;
use DestDefaults;
use Orientable;
use Scrollable;
use SelectionData;
use TargetEntry;
use ToolItem;
use ToolItemGroup;
use ToolPaletteDragTargets;
use ToolbarStyle;
use Widget;
use ffi;
use gdk;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ToolPalette(Object<ffi::GtkToolPalette>): Container, Widget, Orientable, Scrollable;

    match fn {
        get_type => || ffi::gtk_tool_palette_get_type(),
    }
}

impl ToolPalette {
    pub fn new() -> ToolPalette {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_tool_palette_new()).downcast_unchecked()
        }
    }

    pub fn get_drag_target_group() -> Option<TargetEntry> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_drag_target_group())
        }
    }

    pub fn get_drag_target_item() -> Option<TargetEntry> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_drag_target_item())
        }
    }
}

impl Default for ToolPalette {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ToolPaletteExt {
    fn add_drag_dest<P: IsA<Widget>>(&self, widget: &P, flags: DestDefaults, targets: ToolPaletteDragTargets, actions: gdk::DragAction);

    fn get_drag_item(&self, selection: &SelectionData) -> Option<Widget>;

    fn get_drop_group(&self, x: i32, y: i32) -> Option<ToolItemGroup>;

    fn get_drop_item(&self, x: i32, y: i32) -> Option<ToolItem>;

    fn get_exclusive(&self, group: &ToolItemGroup) -> bool;

    fn get_expand(&self, group: &ToolItemGroup) -> bool;

    fn get_group_position(&self, group: &ToolItemGroup) -> i32;

    fn get_icon_size(&self) -> i32;

    fn get_style(&self) -> ToolbarStyle;

    fn set_drag_source(&self, targets: ToolPaletteDragTargets);

    fn set_exclusive(&self, group: &ToolItemGroup, exclusive: bool);

    fn set_expand(&self, group: &ToolItemGroup, expand: bool);

    fn set_group_position(&self, group: &ToolItemGroup, position: i32);

    fn set_icon_size(&self, icon_size: i32);

    fn set_style(&self, style: ToolbarStyle);

    fn unset_icon_size(&self);

    fn unset_style(&self);

    fn get_property_icon_size_set(&self) -> bool;

    fn set_property_icon_size_set(&self, icon_size_set: bool);

    fn get_property_toolbar_style(&self) -> ToolbarStyle;

    fn set_property_toolbar_style(&self, toolbar_style: ToolbarStyle);

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_size_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_toolbar_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ToolPalette> + IsA<glib::object::Object>> ToolPaletteExt for O {
    fn add_drag_dest<P: IsA<Widget>>(&self, widget: &P, flags: DestDefaults, targets: ToolPaletteDragTargets, actions: gdk::DragAction) {
        unsafe {
            ffi::gtk_tool_palette_add_drag_dest(self.to_glib_none().0, widget.to_glib_none().0, flags.to_glib(), targets.to_glib(), actions.to_glib());
        }
    }

    fn get_drag_item(&self, selection: &SelectionData) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_drag_item(self.to_glib_none().0, selection.to_glib_none().0))
        }
    }

    fn get_drop_group(&self, x: i32, y: i32) -> Option<ToolItemGroup> {
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_drop_group(self.to_glib_none().0, x, y))
        }
    }

    fn get_drop_item(&self, x: i32, y: i32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_drop_item(self.to_glib_none().0, x, y))
        }
    }

    fn get_exclusive(&self, group: &ToolItemGroup) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_palette_get_exclusive(self.to_glib_none().0, group.to_glib_none().0))
        }
    }

    fn get_expand(&self, group: &ToolItemGroup) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_palette_get_expand(self.to_glib_none().0, group.to_glib_none().0))
        }
    }

    fn get_group_position(&self, group: &ToolItemGroup) -> i32 {
        unsafe {
            ffi::gtk_tool_palette_get_group_position(self.to_glib_none().0, group.to_glib_none().0)
        }
    }

    fn get_icon_size(&self) -> i32 {
        unsafe {
            ffi::gtk_tool_palette_get_icon_size(self.to_glib_none().0)
        }
    }

    fn get_style(&self) -> ToolbarStyle {
        unsafe {
            from_glib(ffi::gtk_tool_palette_get_style(self.to_glib_none().0))
        }
    }

    fn set_drag_source(&self, targets: ToolPaletteDragTargets) {
        unsafe {
            ffi::gtk_tool_palette_set_drag_source(self.to_glib_none().0, targets.to_glib());
        }
    }

    fn set_exclusive(&self, group: &ToolItemGroup, exclusive: bool) {
        unsafe {
            ffi::gtk_tool_palette_set_exclusive(self.to_glib_none().0, group.to_glib_none().0, exclusive.to_glib());
        }
    }

    fn set_expand(&self, group: &ToolItemGroup, expand: bool) {
        unsafe {
            ffi::gtk_tool_palette_set_expand(self.to_glib_none().0, group.to_glib_none().0, expand.to_glib());
        }
    }

    fn set_group_position(&self, group: &ToolItemGroup, position: i32) {
        unsafe {
            ffi::gtk_tool_palette_set_group_position(self.to_glib_none().0, group.to_glib_none().0, position);
        }
    }

    fn set_icon_size(&self, icon_size: i32) {
        unsafe {
            ffi::gtk_tool_palette_set_icon_size(self.to_glib_none().0, icon_size);
        }
    }

    fn set_style(&self, style: ToolbarStyle) {
        unsafe {
            ffi::gtk_tool_palette_set_style(self.to_glib_none().0, style.to_glib());
        }
    }

    fn unset_icon_size(&self) {
        unsafe {
            ffi::gtk_tool_palette_unset_icon_size(self.to_glib_none().0);
        }
    }

    fn unset_style(&self) {
        unsafe {
            ffi::gtk_tool_palette_unset_style(self.to_glib_none().0);
        }
    }

    fn get_property_icon_size_set(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "icon-size-set".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_icon_size_set(&self, icon_size_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon-size-set".to_glib_none().0, Value::from(&icon_size_set).to_glib_none().0);
        }
    }

    fn get_property_toolbar_style(&self) -> ToolbarStyle {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "toolbar-style".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_property_toolbar_style(&self, toolbar_style: ToolbarStyle) {
        let toolbar_style = toolbar_style.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "toolbar-style".to_glib_none().0, Value::from(&toolbar_style).to_glib_none().0);
        }
    }

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon-size",
                transmute(notify_icon_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_icon_size_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon-size-set",
                transmute(notify_icon_size_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_toolbar_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::toolbar-style",
                transmute(notify_toolbar_style_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_icon_size_trampoline<P>(this: *mut ffi::GtkToolPalette, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToolPalette> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToolPalette::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_icon_size_set_trampoline<P>(this: *mut ffi::GtkToolPalette, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToolPalette> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToolPalette::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_toolbar_style_trampoline<P>(this: *mut ffi::GtkToolPalette, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ToolPalette> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ToolPalette::from_glib_borrow(this).downcast_unchecked())
}
