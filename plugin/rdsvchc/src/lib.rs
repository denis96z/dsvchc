#[repr(C)]
pub struct PluginStr {
    size: usize,
    data: *const u8,
}

#[no_mangle]
pub unsafe extern "C" fn dsvchc_plugin_ctx_create() -> *mut Ctx {
    Box::into_raw(Box::new(Ctx::new()))
}

#[no_mangle]
pub unsafe extern "C" fn dsvchc_plugin_ctx_initialize(
    _ctx: *mut Ctx,
    _conf_path: *const PluginStr,
) -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn dsvchc_plugin_check_create(_ctx: *mut Ctx) -> *mut usize {
    Box::into_raw(Box::new(0))
}

#[no_mangle]
pub unsafe extern "C" fn dsvchc_plugin_check_initialize(
    _ctx: *mut Ctx,
    _check_idx: *mut usize,
    _conf_path: *const PluginStr,
) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn dsvchc_plugin_check_perform(_ctx: *mut Ctx, _check_idx: *mut usize) -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn dsvchc_plugin_check_get_result_json(
    _ctx: *mut Ctx,
    _check_idx: *mut usize,
) -> *const PluginStr {
    const JSTR: &str = "{}";

    const JSON_OK: PluginStr = PluginStr {
        size: JSTR.len(),
        data: JSTR.as_ptr(),
    };

    &JSON_OK
}

#[no_mangle]
pub unsafe extern "C" fn dsvchc_plugin_check_finalize(
    _ctx: *mut Ctx,
    _check_idx: *mut usize,
) -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn dsvchc_plugin_check_get_error(
    _ctx: *mut Ctx,
    _check_idx: *mut usize,
) -> *const PluginStr {
    unreachable!()
}

#[no_mangle]
pub unsafe extern "C" fn dsvchc_plugin_check_destroy(_ctx: *mut Ctx, check_idx: *mut usize) {
    drop(Box::from_raw(check_idx));
}

#[no_mangle]
pub unsafe extern "C" fn dsvchc_plugin_ctx_finalize(_ctx: *mut Ctx) -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn dsvchc_plugin_ctx_get_error(_ctx: *mut Ctx) -> *const PluginStr {
    unreachable!()
}

#[no_mangle]
pub unsafe extern "C" fn dsvchc_plugin_ctx_destroy(ctx: *mut Ctx) {
    drop(Box::from_raw(ctx));
}

pub struct Ctx {
    _checks: Vec<Check>,
}

impl Ctx {
    fn new() -> Self {
        Self {
            _checks: Vec::new(),
        }
    }
}

pub struct Check {}
