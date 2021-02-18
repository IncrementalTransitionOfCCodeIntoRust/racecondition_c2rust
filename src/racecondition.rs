#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, main, register_tool)]
use ::racecondition_c2rust::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn setbuf(__stream: *mut FILE, __buf: *mut libc::c_char);
    #[no_mangle]
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fork() -> __pid_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type pid_t = __pid_t;
unsafe fn main_0() -> libc::c_int {
    let mut pid: pid_t = fork();
    if pid == 0 as libc::c_int {
        // long text by child
        charAtATime(b"output AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBBBBBBBBBBBBBB from child\n\x00"
                        as *const u8 as *const libc::c_char as
                        *mut libc::c_char);
    } else {
        // long text by parent
        charAtATime(b"output AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBBBBBBBBBBBBBB from parent\n\x00"
                        as *const u8 as *const libc::c_char as
                        *mut libc::c_char);
    }
    return 0;
}
unsafe extern "C" fn charAtATime(mut str: *mut libc::c_char) {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    setbuf(stdout, 0 as *mut libc::c_char);
    // do one character at a time to see race condition
    // this is due to Round Robin Scheduling
    // letting child and parent compete for the calling process
    ptr = str;
    loop  {
        let fresh0 = ptr;
        ptr = ptr.offset(1);
        c = *fresh0 as libc::c_int;
        if !(c != 0 as libc::c_int) { break ; }
        putc(c, stdout);
    };
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
