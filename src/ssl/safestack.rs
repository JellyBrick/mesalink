/*
 *   __  __                 _     _       _
 *  |  \/  | ___  ___  __ _| |   (_)_ __ | | __
 *  | |\/| |/ _ \/ __|/ _` | |   | | '_ \| |/ /
 *  | |  | |  __/\__ \ (_| | |___| | | | |   <
 *  |_|  |_|\___||___/\__,_|_____|_|_| |_|_|\_\
 *
 * Copyright (c) 2017-2018, The MesaLink Authors.
 * All rights reserved.
 *
 * This work is licensed under the terms of the BSD 3-Clause License.
 * For a copy, see the LICENSE file.
 *
 */

use libc::c_int;
use ssl::err::{ErrorCode, MesalinkInnerResult};
use ssl::error_san::*;
use ssl::x509::MESALINK_X509_NAME;
use ssl::{MesalinkOpaquePointerType, MAGIC, MAGIC_SIZE};
use ssl::{SSL_FAILURE, SSL_SUCCESS};
use std::ptr;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct MESALINK_STACK_MESALINK_X509_NAME {
    magic: [u8; MAGIC_SIZE],
    pub stack: Vec<MESALINK_X509_NAME>,
}

impl MesalinkOpaquePointerType for MESALINK_STACK_MESALINK_X509_NAME {
    fn check_magic(&self) -> bool {
        self.magic == *MAGIC
    }
}

impl MESALINK_STACK_MESALINK_X509_NAME {
    pub fn new(names: Vec<MESALINK_X509_NAME>) -> MESALINK_STACK_MESALINK_X509_NAME {
        MESALINK_STACK_MESALINK_X509_NAME {
            magic: *MAGIC,
            stack: names,
        }
    }
}

#[no_mangle]
pub extern "C" fn mesalink_sk_X509_NAME_new_null() -> *mut MESALINK_STACK_MESALINK_X509_NAME {
    let stack = MESALINK_STACK_MESALINK_X509_NAME::new(vec![]);
    Box::into_raw(Box::new(stack)) as *mut MESALINK_STACK_MESALINK_X509_NAME
}

#[no_mangle]
pub extern "C" fn mesalink_sk_X509_NAME_num(
    stack_ptr: *const MESALINK_STACK_MESALINK_X509_NAME,
) -> c_int {
    check_inner_result!(inner_mesalink_sk_X509_NAME_num(stack_ptr), SSL_FAILURE)
}

#[allow(non_snake_case)]
fn inner_mesalink_sk_X509_NAME_num(
    stack_ptr: *const MESALINK_STACK_MESALINK_X509_NAME,
) -> MesalinkInnerResult<c_int> {
    let stack = sanitize_const_ptr_for_ref(stack_ptr)?;
    Ok(stack.stack.len() as c_int)
}

#[no_mangle]
pub extern "C" fn mesalink_sk_X509_NAME_value(
    stack_ptr: *const MESALINK_STACK_MESALINK_X509_NAME,
    index: c_int,
) -> *const MESALINK_X509_NAME {
    check_inner_result!(
        inner_mesalink_sk_X509_NAME_value(stack_ptr, index),
        ptr::null()
    )
}

#[allow(non_snake_case)]
fn inner_mesalink_sk_X509_NAME_value(
    stack_ptr: *const MESALINK_STACK_MESALINK_X509_NAME,
    index: c_int,
) -> MesalinkInnerResult<*const MESALINK_X509_NAME> {
    let stack = sanitize_const_ptr_for_ref(stack_ptr)?;
    let item = stack
        .stack
        .get(index as usize)
        .ok_or(error!(ErrorCode::MesalinkErrorBadFuncArg))?;
    Ok(item as *const MESALINK_X509_NAME)
}

#[no_mangle]
pub extern "C" fn mesalink_sk_X509_NAME_push(
    stack_ptr: *mut MESALINK_STACK_MESALINK_X509_NAME,
    item_ptr: *const MESALINK_X509_NAME,
) -> c_int {
    check_inner_result!(
        inner_mesalink_sk_X509_NAME_push(stack_ptr, item_ptr),
        SSL_FAILURE
    )
}

#[allow(non_snake_case)]
fn inner_mesalink_sk_X509_NAME_push(
    stack_ptr: *mut MESALINK_STACK_MESALINK_X509_NAME,
    item_ptr: *const MESALINK_X509_NAME,
) -> MesalinkInnerResult<c_int> {
    let stack = sanitize_ptr_for_mut_ref(stack_ptr)?;
    let item = sanitize_const_ptr_for_ref(item_ptr)?;
    stack.stack.push(item.clone());
    Ok(SSL_SUCCESS)
}

#[no_mangle]
pub extern "C" fn mesalink_sk_X509_NAME_free(stack_ptr: *mut MESALINK_STACK_MESALINK_X509_NAME) {
    let _ = check_inner_result!(inner_mesalink_sk_X509_NAME_free(stack_ptr), SSL_FAILURE);
}

#[allow(non_snake_case)]
fn inner_mesalink_sk_X509_NAME_free(
    stack_ptr: *mut MESALINK_STACK_MESALINK_X509_NAME,
) -> MesalinkInnerResult<c_int> {
    let _ = sanitize_ptr_for_mut_ref(stack_ptr)?;
    let _ = unsafe { Box::from_raw(stack_ptr) };
    Ok(SSL_SUCCESS)
}