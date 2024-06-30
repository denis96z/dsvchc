use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ffi::c_void;
use std::rc::Rc;

use dlopen::wrapper::{Container, WrapperApi};
use serde::Serialize;
use serde_json::value::RawValue;

pub struct Plugins {
    plugins: BTreeMap<String, Plugin>,
}

#[derive(Serialize)]
struct PluginsChecksResult {
    #[serde(rename = "ok")]
    ok: bool,

    #[serde(rename = "plugins")]
    plugins: BTreeMap<String, PluginChecksResult>,
}

impl Plugins {
    pub fn new() -> Self {
        Self {
            plugins: BTreeMap::new(),
        }
    }

    pub fn load(&mut self, name: &str, plugin_path: &str, conf_path: &str) -> Result<(), String> {
        self.plugins
            .insert(name.into(), Plugin::load(plugin_path, conf_path)?);
        Ok(())
    }

    pub fn add_check(
        &mut self,
        plugin_name: &str,
        check_name: &str,
        conf_path: &str,
    ) -> Result<(), String> {
        self.plugins
            .get_mut(plugin_name)
            .unwrap()
            .add_check(check_name, conf_path)
    }

    pub fn perform_checks(&mut self) -> Result<String, String> {
        let mut r = PluginsChecksResult {
            ok: true,
            plugins: BTreeMap::new(),
        };
        for (name, plugin) in &mut self.plugins {
            let xr = plugin.perform_checks()?;
            if !xr.ok {
                r.ok = false;
            }
            r.plugins.insert(name.into(), xr);
        }
        Ok(serde_json::to_string(&r).unwrap())
    }

    pub fn release(&mut self) -> Result<(), String> {
        for plugin in self.plugins.values_mut() {
            plugin.release()?;
        }
        Ok(())
    }
}

pub struct Plugin {
    ctx: Rc<RefCell<PluginCtx>>,
    chks: BTreeMap<String, PluginCheck>,
}

#[derive(Serialize)]
pub struct PluginChecksResult {
    #[serde(rename = "ok")]
    ok: bool,

    #[serde(rename = "checks")]
    checks: BTreeMap<String, PluginCheckResult>,
}

impl Plugin {
    pub fn load(plugin_path: &str, conf_path: &str) -> Result<Self, String> {
        let cnt = Rc::new(unsafe { Container::load(plugin_path) }.unwrap());

        let mut ctx = PluginCtx::create(Rc::clone(&cnt));
        ctx.initialize(conf_path)?;

        Ok(Self {
            ctx: Rc::new(RefCell::new(ctx)),
            chks: BTreeMap::new(),
        })
    }

    pub fn add_check(&mut self, name: &str, conf_path: &str) -> Result<(), String> {
        let mut chk = PluginCheck::create(Rc::clone(&self.ctx));
        chk.initialize(conf_path)?;

        self.chks.insert(name.into(), chk);
        Ok(())
    }

    pub fn perform_checks(&mut self) -> Result<PluginChecksResult, String> {
        let mut r = PluginChecksResult {
            ok: true,
            checks: BTreeMap::new(),
        };
        for (name, chk) in &mut self.chks {
            let xr = chk.perform()?;
            if !xr.ok {
                r.ok = false;
            }
            r.checks.insert(name.into(), xr);
        }
        Ok(r)
    }

    pub fn release(&mut self) -> Result<(), String> {
        for chk in self.chks.values_mut() {
            chk.finalize()?;
        }
        self.ctx.borrow_mut().finalize()?;
        Ok(())
    }
}

const PLUGIN_ERR_CODE_OK: i32 = 0;
const PLUGIN_ERR_CODE_CHECK_FAIL: i32 = 3;

struct PluginCtx {
    ctx: *mut c_void,
    cnt: Rc<Container<PluginApi>>,
}

impl PluginCtx {
    fn create(cnt: Rc<Container<PluginApi>>) -> Self {
        Self {
            ctx: unsafe { cnt.dsvchc_plugin_ctx_create() },
            cnt,
        }
    }

    fn initialize(&mut self, conf_path: &str) -> Result<(), String> {
        unsafe {
            if self
                .cnt
                .dsvchc_plugin_ctx_initialize(self.ctx, &PluginStr::from_str_ref(conf_path))
                != PLUGIN_ERR_CODE_OK
            {
                return Err((*self.cnt.dsvchc_plugin_ctx_get_error(self.ctx)).to_string());
            }
        }
        Ok(())
    }

    fn finalize(&mut self) -> Result<(), String> {
        unsafe {
            if self.cnt.dsvchc_plugin_ctx_finalize(self.ctx) != PLUGIN_ERR_CODE_OK {
                return Err((*self.cnt.dsvchc_plugin_ctx_get_error(self.ctx)).to_string());
            }
        }
        Ok(())
    }
}

impl Drop for PluginCtx {
    fn drop(&mut self) {
        unsafe { self.cnt.dsvchc_plugin_ctx_destroy(self.ctx) };
    }
}

struct PluginCheck {
    chk: *mut c_void,
    ctx: Rc<RefCell<PluginCtx>>,
}

#[derive(Serialize)]
pub struct PluginCheckResult {
    #[serde(rename = "ok")]
    ok: bool,

    #[serde(rename = "extra")]
    extra: Box<RawValue>,
}

impl PluginCheck {
    fn create(ctx: Rc<RefCell<PluginCtx>>) -> Self {
        let x = ctx.borrow_mut().ctx;
        Self {
            chk: unsafe {
                Rc::clone(&ctx)
                    .borrow_mut()
                    .cnt
                    .dsvchc_plugin_check_create(x)
            },
            ctx: Rc::clone(&ctx),
        }
    }

    fn initialize(&mut self, conf_path: &str) -> Result<(), String> {
        let x = self.ctx.borrow_mut().ctx;
        unsafe {
            if self.ctx.borrow_mut().cnt.dsvchc_plugin_check_initialize(
                x,
                self.chk,
                &PluginStr::from_str_ref(conf_path),
            ) != PLUGIN_ERR_CODE_OK
            {
                return Err((*self
                    .ctx
                    .borrow_mut()
                    .cnt
                    .dsvchc_plugin_check_get_error(x, self.chk))
                .to_string());
            }
        }
        Ok(())
    }

    fn perform(&mut self) -> Result<PluginCheckResult, String> {
        let x = self.ctx.borrow_mut().ctx;
        unsafe {
            let c = self
                .ctx
                .borrow_mut()
                .cnt
                .dsvchc_plugin_check_perform(x, self.chk);
            if c == PLUGIN_ERR_CODE_OK || c == PLUGIN_ERR_CODE_CHECK_FAIL {
                Ok(PluginCheckResult {
                    ok: c == PLUGIN_ERR_CODE_OK,
                    extra: RawValue::from_string(
                        (*self
                            .ctx
                            .borrow_mut()
                            .cnt
                            .dsvchc_plugin_check_get_result_json(x, self.chk))
                        .to_string(),
                    )
                    .unwrap(),
                })
            } else {
                Err((*self
                    .ctx
                    .borrow_mut()
                    .cnt
                    .dsvchc_plugin_check_get_error(x, self.chk))
                .to_string())
            }
        }
    }

    fn finalize(&mut self) -> Result<(), String> {
        let x = self.ctx.borrow_mut().ctx;
        unsafe {
            if self
                .ctx
                .borrow_mut()
                .cnt
                .dsvchc_plugin_check_finalize(x, self.chk)
                != PLUGIN_ERR_CODE_OK
            {
                return Err((*self
                    .ctx
                    .borrow_mut()
                    .cnt
                    .dsvchc_plugin_check_get_error(x, self.chk))
                .to_string());
            }
        }
        Ok(())
    }
}

impl Drop for PluginCheck {
    fn drop(&mut self) {
        let x = self.ctx.borrow_mut().ctx;
        unsafe {
            self.ctx
                .borrow_mut()
                .cnt
                .dsvchc_plugin_check_destroy(x, self.chk)
        };
    }
}

#[repr(C)]
struct PluginStr {
    size: usize,
    data: *const u8,
}

impl PluginStr {
    unsafe fn from_str_ref(s: &str) -> Self {
        Self {
            size: s.len(),
            data: s.as_bytes().as_ptr(),
        }
    }

    unsafe fn to_str_ref(&self) -> &str {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.data, self.size))
    }

    unsafe fn to_string(&self) -> String {
        self.to_str_ref().into()
    }
}

#[derive(WrapperApi)]
struct PluginApi {
    dsvchc_plugin_ctx_create: unsafe extern "C" fn() -> *mut c_void,

    dsvchc_plugin_ctx_initialize:
        unsafe extern "C" fn(ctx: *mut c_void, conf_path: *const PluginStr) -> i32,

    dsvchc_plugin_check_create: unsafe extern "C" fn(ctx: *mut c_void) -> *mut c_void,

    dsvchc_plugin_check_initialize: unsafe extern "C" fn(
        ctx: *mut c_void,
        check: *mut c_void,
        conf_path: *const PluginStr,
    ) -> i32,

    dsvchc_plugin_check_perform: unsafe extern "C" fn(ctx: *mut c_void, check: *mut c_void) -> i32,

    dsvchc_plugin_check_get_result_json:
        unsafe extern "C" fn(ctx: *mut c_void, check: *mut c_void) -> *const PluginStr,

    dsvchc_plugin_check_finalize: unsafe extern "C" fn(ctx: *mut c_void, check: *mut c_void) -> i32,

    dsvchc_plugin_check_get_error:
        unsafe extern "C" fn(ctx: *mut c_void, check: *mut c_void) -> *const PluginStr,

    dsvchc_plugin_check_destroy: unsafe extern "C" fn(ctx: *mut c_void, check: *mut c_void),

    dsvchc_plugin_ctx_finalize: unsafe extern "C" fn(ctx: *mut c_void) -> i32,

    dsvchc_plugin_ctx_get_error: unsafe extern "C" fn(ctx: *mut c_void) -> *const PluginStr,

    dsvchc_plugin_ctx_destroy: unsafe extern "C" fn(ctx: *mut c_void),
}
