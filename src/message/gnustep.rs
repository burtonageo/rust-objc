use runtime::{Object, Imp, Sel, Super, self};

pub fn msg_send_fn<R>(obj: *mut Object, sel: Sel) -> (Imp, *mut Object) {
    let imp_fn = unsafe {
        runtime::objc_msg_lookup(obj, sel)
    };
    (imp_fn, obj)
}

pub fn msg_send_super_fn<R>(sup: &Super, sel: Sel) -> (Imp, *mut Object) {
    let imp_fn = unsafe {
        runtime::objc_msg_lookup_super(sup, sel)
    };
    (imp_fn, sup.receiver)
}
