use libc::{c_int, c_uint, c_void, size_t, c_uchar};
use ::std::ptr;

const MAX_ERROR_MESSAGE_SIZE: usize = 3024;

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

#[repr(C)]
pub struct Env { _private: [u8; 0] }

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


#[derive(Debug)]
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

pub fn env_nls_create(envhpp: &*mut Env, mode: Mode) -> ReturnCode {
    let null_ptr = ptr::null();
    unsafe {
        OCIEnvNlsCreate(
            &envhpp,
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
    }
}

pub fn error_get(
    hndlp: *mut c_void,
    recordno: c_uint,
    errcodep: *mut c_int,
    message: &mut CString,
    hnd_type: HandleType,
) -> ReturnCode {
    let sql_state: *mut c_uchar = ptr::null_mut();
    unsafe {
        OCIErrorGet(
            hndlp,
            recordno,
            sql_state,
            errcodep,
            message.as_mut_ptr(),
            message.capacity() as c_uint,
            hnd_type.into(),
        ).into()
    }
}

pub fn handle_free(hndl: *mut c_void, typ: HandleType) -> ReturnCode {
    unsafe { OCIHandleFree(hndl, typ.into()).into() }
}

#[link(name = "clntsh")]
extern "C" {
    fn OCIEnvNlsCreate(
        envhpp: &*mut Env,
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
        sqlstate: *mut c_uchar,
        errcodep: *mut c_int,
        bufp: *mut c_uchar,
        bufsiz: c_uint,
        hnd_type: c_uint,
    ) -> c_int;
}