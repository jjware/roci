use libc::{c_int, c_uint, c_void, size_t, c_char};
use std::ptr;
use std::ffi::CString;

const MAX_ERROR_MESSAGE_SIZE: usize = 3024;

/*
#[derive(Debug)]
pub struct CString {
    data: Vec<c_uchar>
}

impl CString {
    fn new(capacity: usize) -> Self {
        CString { data: vec![0; capacity] }
    }

    fn as_mut_ptr(&mut self) -> *mut c_uchar {
        self.data.as_mut_ptr()
    }

    fn len(&self) -> usize {
        self.data.iter().position(|&x| x == 0).unwrap()
    }

    fn capacity(&self) -> usize {
        self.data.capacity()
    }
}

impl From<CString> for String {
    fn from(cstring: CString) -> String {
        let first_null_byte_index = cstring.len();
        String::from_utf8_lossy(&cstring.data[0..first_null_byte_index]).into_owned()
    }
}
*/

const OCI_DEFAULT: c_uint = 0;
const OCI_THREADED: c_uint = 1;

#[derive(Debug)]
pub enum Mode {
    Default,
    Threaded,
}

impl From<Mode> for c_uint {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Default => OCI_DEFAULT,
            Mode::Threaded => OCI_THREADED,
        }
    }
}

const OCI_SUCCESS: c_int = 0;
const OCI_SUCCESS_WITH_INFO: c_int = 1;
const OCI_ERROR: c_int = -1;
const OCI_NO_DATA: c_int = 100;
const OCI_INVALID_HANDLE: c_int = -2;


#[derive(Debug, PartialEq)]
pub enum ReturnCode {
    Success,
    SuccessWithInfo,
    Error,
    NoData,
    InvalidHandle,
}

impl From<c_int> for ReturnCode {
    fn from(code: c_int) -> Self {
        match code {
            OCI_SUCCESS => ReturnCode::Success,
            OCI_SUCCESS_WITH_INFO => ReturnCode::SuccessWithInfo,
            OCI_ERROR => ReturnCode::Error,
            OCI_NO_DATA => ReturnCode::NoData,
            OCI_INVALID_HANDLE => ReturnCode::InvalidHandle,
            _ => panic!(format!("unknown oci return code {}", code)),
        }
    }
}

const OCI_HTYPE_ENV: c_uint = 1;
const OCI_HTYPE_ERROR: c_uint = 2;
const OCI_HTYPE_SVCCTX: c_uint = 3;
const OCI_HTYPE_STMT: c_uint = 4;
const OCI_HTYPE_BIND: c_uint = 5;
const OCI_HTYPE_DEFINE: c_uint = 6;
const OCI_HTYPE_DESCRIBE: c_uint = 7;
const OCI_HTYPE_SERVER: c_uint = 8;
const OCI_HTYPE_SESSION: c_uint = 9;
const OCI_HTYPE_AUTHINFO: c_uint = 10;
const OCI_HTYPE_CPOOL: c_uint = 11;
const OCI_HTYPE_SPOOL: c_uint = 12;
const OCI_HTYPE_TRANS: c_uint = 13;
const OCI_HTYPE_COMPLEXOBJECT: c_uint = 14;

#[derive(Debug)]
pub enum HandleType {
    Env,
    Error,
    SvcCtx,
    Stmt,
    Bind,
    Define,
    Describe,
    Server,
    Session,
    AuthInfo,
    CPool,
    SPool,
    Trans,
    ComplexObject,
}

impl From<HandleType> for c_uint {
    fn from(handle_type: HandleType) -> Self {
        match handle_type {
            HandleType::Env => OCI_HTYPE_ENV,
            HandleType::Error => OCI_HTYPE_ERROR,
            HandleType::SvcCtx => OCI_HTYPE_SVCCTX,
            HandleType::Stmt => OCI_HTYPE_STMT,
            HandleType::Bind => OCI_HTYPE_BIND,
            HandleType::Define => OCI_HTYPE_DEFINE,
            HandleType::Describe => OCI_HTYPE_DESCRIBE,
            HandleType::Server => OCI_HTYPE_SERVER,
            HandleType::Session => OCI_HTYPE_SESSION,
            HandleType::AuthInfo => OCI_HTYPE_AUTHINFO,
            HandleType::CPool => OCI_HTYPE_CPOOL,
            HandleType::SPool => OCI_HTYPE_SPOOL,
            HandleType::Trans => OCI_HTYPE_TRANS,
            HandleType::ComplexObject => OCI_HTYPE_COMPLEXOBJECT,
        }
    }
}

impl From<c_uint> for HandleType {
    fn from(typ: c_uint) -> Self {
        match typ {
            OCI_HTYPE_ENV => HandleType::Env,
            OCI_HTYPE_ERROR => HandleType::Error,
            OCI_HTYPE_SVCCTX => HandleType::SvcCtx,
            OCI_HTYPE_STMT => HandleType::Stmt,
            OCI_HTYPE_BIND => HandleType::Bind,
            OCI_HTYPE_DEFINE => HandleType::Define,
            OCI_HTYPE_DESCRIBE => HandleType::Describe,
            OCI_HTYPE_SERVER => HandleType::Server,
            OCI_HTYPE_SESSION => HandleType::Session,
            OCI_HTYPE_AUTHINFO => HandleType::AuthInfo,
            OCI_HTYPE_CPOOL => HandleType::CPool,
            OCI_HTYPE_SPOOL => HandleType::SPool,
            OCI_HTYPE_TRANS => HandleType::Trans,
            OCI_HTYPE_COMPLEXOBJECT => HandleType::ComplexObject,
            _ => panic!(format!("unknown oci handle type {}", typ)),
        }
    }
}

#[repr(C)]
struct Handle { _private: [u8; 0] }

#[derive(Debug)]
pub struct Env { _handle: *mut Handle }

impl Env {
    pub fn new(mode: Mode) -> Result<Env, ReturnCode> {
        let handle: *mut Handle = ptr::null_mut();
        let null_ptr = ptr::null();

        let result: ReturnCode = unsafe {
            OCIEnvNlsCreate(
                &handle,
                mode.into(),
                null_ptr,
                null_ptr,
                null_ptr,
                null_ptr,
                0,
                null_ptr,
                0,
                0,
            ).into()
        };

        match result {
            ReturnCode::Success => Ok(Env { _handle: handle }),
            _ => Err(result),
        }
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        handle_free(self._handle, HandleType::Env)
    }
}

#[derive(Debug)]
pub struct Error { _handle: *mut Handle }

impl Error {
    pub fn new(env: &Env) -> Result<Error, ReturnCode> {
        handle_alloc(&env, HandleType::Error)
            .map(|x| Error { _handle: x })
    }
}

impl Drop for Error {
    fn drop(&mut self) {
        handle_free(self._handle, HandleType::Error)
    }
}

#[derive(Debug)]
pub struct CPool { _handle: *mut Handle }

impl CPool {
    pub fn new(env: &Env) -> Result<CPool, ReturnCode> {
        handle_alloc(&env, HandleType::CPool)
            .map(|x| CPool { _handle: x })
    }
}

impl Drop for CPool {
    fn drop(&mut self) {
        handle_free(self._handle, HandleType::CPool)
    }
}

fn handle_free(handle: *mut Handle, handle_type: HandleType) {
    let result: ReturnCode = unsafe {
        OCIHandleFree(handle as *mut c_void, handle_type.into()).into()
    };

    match result {
        ReturnCode::Success => (),
        _ => error!("unable to close handle: {:?}", result)
    }
}

fn handle_alloc(env: &Env, handle_type: HandleType) -> Result<*mut Handle, ReturnCode> {
    let handle: *mut c_void = ptr::null_mut();

    let result = unsafe {
        OCIHandleAlloc(
            env._handle as *const c_void,
            &handle,
            handle_type.into(),
            0,
            ptr::null(),
        ).into()
    };

    match result {
        ReturnCode::Success => Ok(handle as *mut Handle),
        _ => Err(result),
    }
}


pub fn error_get(error: &Error, recordno: u32) -> Result<String, ReturnCode> {
    let sql_state: *mut c_char = ptr::null_mut();
    let mut messagep: *mut c_char = ptr::null_mut();
    let mut errcodep: c_int = 0;

    let result = unsafe {
        OCIErrorGet(
            error._handle as *mut c_void,
            recordno as c_uint,
            sql_state,
            &mut errcodep,
            messagep,
            3024 as c_uint,
            HandleType::Error.into(),
        ).into()
    };

    match result {
        ReturnCode::Success => {
            let c_string = unsafe { CString::from_raw(messagep) };
            Ok(c_string.into_string().unwrap())
        },
        _ => Err(result)
    }
}


pub fn connection_pool_create(
    env: &Env,
    error: &Error,
    cpool: &CPool,
    dblink: &str,
    conn_min: u32,
    conn_max: u32,
    conn_incr: u32,
    pool_username: &str,
    pool_password: &str,
    mode: Mode,
) -> Result<String, ReturnCode> {
    let c_pool_name: *mut c_char = ptr::null_mut();
    let c_pool_name_len: *mut c_uint = ptr::null_mut();
    let c_dblink: CString = CString::new(dblink).expect("failed to convert dblink");
    let c_pool_username: CString = CString::new(pool_username).expect("failed to convert username");
    let c_pool_password: CString = CString::new(pool_password).expect("failed to convert password");

    let result: ReturnCode = unsafe {
        OCIConnectionPoolCreate(
            env._handle,
            error._handle,
            cpool._handle,
            &c_pool_name,
            &c_pool_name_len,
            c_dblink.as_ptr(),
            dblink.len() as c_uint,
            conn_min as c_uint,
            conn_max as c_uint,
            conn_incr as c_uint,
            c_pool_username.as_ptr(),
            pool_username.len() as c_uint,
            c_pool_password.as_ptr(),
            pool_password.len() as c_uint,
            mode.into(),
        ).into()
    };

    match result {
        ReturnCode::Success => {
            let c_string = unsafe { CString::from_raw(c_pool_name) };
            Ok(c_string.into_string().unwrap())
        },
        _ => Err(result)
    }
}

extern "C" {
    fn OCIEnvNlsCreate(
        envhpp: &*mut Handle,
        mode: c_uint,
        ctxp: *const c_void,
        maloc_fp: *const c_void,
        raloc_fp: *const c_void,
        mfree_fp: *const c_void,
        xtramemsz: size_t,
        usrmempp: *const c_void,
        charset: c_uint,
        ncharset: c_uint,
    ) -> c_int;

    fn OCIHandleFree(hndlp: *mut c_void, hndtyp: c_uint) -> c_int;

    fn OCIErrorGet(
        hndlp: *mut c_void,
        recordno: c_uint,
        sqlstate: *mut c_char,
        errcodep: *mut c_int,
        bufp: *mut c_char,
        bufsiz: c_uint,
        hnd_type: c_uint,
    ) -> c_int;

    fn OCIHandleAlloc(
        parenthp: *const c_void,
        hndlpp: &*mut c_void,
        hnd_type: c_uint,
        xtramemsz: size_t,
        usrmempp: *const c_void,
    ) -> c_int;

    fn OCIConnectionPoolCreate(
        envhp: *mut Handle,
        errhp: *mut Handle,
        poolhp: *mut Handle,
        pool_name: &*mut c_char,
        pool_name_len: &*mut c_uint,
        dblink: *const c_char,
        dblink_len: c_uint,
        conn_min: c_uint,
        conn_max: c_uint,
        conn_incr: c_uint,
        pool_username: *const c_char,
        pool_user_len: c_uint,
        pool_password: *const c_char,
        pool_pass_len: c_uint,
        mode: c_uint,
    ) -> c_int;
}