
////////////////////////////////////////
// INTEROPERABLE PRIMITIVE DATA TYPES //
////////////////////////////////////////

use std::collections::HashMap;

pub enum _PrimitiveDataInsatance {
    Float32(f32),
    Float64(f64),
    Int32(i32),
    Int64(i64),
    Uint32(u32),
    Uint64(u64),
}

// used as an opaque struct that holds the complex data instance
#[repr(packed)]
pub struct _PrimitiveDataContainer(_PrimitiveDataInsatance);

#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn delete_primitive_data_inst(pd: *mut _PrimitiveDataContainer) {
    if !pd.is_null() {
        Box::from_raw(pd);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn new_primitive_data_inst_f32(value: f32) -> *mut _PrimitiveDataContainer {
    Box::into_raw(Box::new(_PrimitiveDataContainer(_PrimitiveDataInsatance::Float32(value))))
}
#[unsafe(no_mangle)]
pub extern "C" fn new_primitive_data_inst_f64(value: f64) -> *mut _PrimitiveDataContainer {
    Box::into_raw(Box::new(_PrimitiveDataContainer(_PrimitiveDataInsatance::Float64(value))))
}
#[unsafe(no_mangle)]
pub extern "C" fn new_primitive_data_inst_i32(value: i32) -> *mut _PrimitiveDataContainer {
    Box::into_raw(Box::new(_PrimitiveDataContainer(_PrimitiveDataInsatance::Int32(value))))
}
#[unsafe(no_mangle)]
pub extern "C" fn new_primitive_data_inst_i64(value: i64) -> *mut _PrimitiveDataContainer {
    Box::into_raw(Box::new(_PrimitiveDataContainer(_PrimitiveDataInsatance::Int64(value))))
}
#[unsafe(no_mangle)]
pub extern "C" fn new_primitive_data_inst_u32(value: u32) -> *mut _PrimitiveDataContainer {
    Box::into_raw(Box::new(_PrimitiveDataContainer(_PrimitiveDataInsatance::Uint32(value))))
}
#[unsafe(no_mangle)]
pub extern "C" fn new_primitive_data_inst_u64(value: u64) -> *mut _PrimitiveDataContainer {
    Box::into_raw(Box::new(_PrimitiveDataContainer(_PrimitiveDataInsatance::Uint64(value))))
}

#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn primitive_data_inst_as_f32(pd: *mut _PrimitiveDataContainer) -> f32 {
    match (*pd).0 {
        _PrimitiveDataInsatance::Float32(v) => v,
        _PrimitiveDataInsatance::Float64(v) => v as f32,
        _PrimitiveDataInsatance::Int32(v) => v as f32,
        _PrimitiveDataInsatance::Int64(v) => v as f32,
        _PrimitiveDataInsatance::Uint32(v) => v as f32,
        _PrimitiveDataInsatance::Uint64(v) => v as f32,
    }
}

#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn primitive_data_inst_as_f64(pd: *mut _PrimitiveDataContainer) -> f64 {
    match (*pd).0 {
        _PrimitiveDataInsatance::Float32(v) => v as f64,
        _PrimitiveDataInsatance::Float64(v) => v,
        _PrimitiveDataInsatance::Int32(v) => v as f64,
        _PrimitiveDataInsatance::Int64(v) => v as f64,
        _PrimitiveDataInsatance::Uint32(v) => v as f64,
        _PrimitiveDataInsatance::Uint64(v) => v as f64,
    }
}

#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn primitive_data_inst_as_i32(pd: *mut _PrimitiveDataContainer) -> i32 {
    match (*pd).0 {
        _PrimitiveDataInsatance::Float32(v) => v as i32,
        _PrimitiveDataInsatance::Float64(v) => v as i32,
        _PrimitiveDataInsatance::Int32(v) => v,
        _PrimitiveDataInsatance::Int64(v) => v as i32,
        _PrimitiveDataInsatance::Uint32(v) => v as i32,
        _PrimitiveDataInsatance::Uint64(v) => v as i32,
    }
}

#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn primitive_data_inst_as_i64(pd: *mut _PrimitiveDataContainer) -> i64 {
    match (*pd).0 {
        _PrimitiveDataInsatance::Float32(v) => v as i64,
        _PrimitiveDataInsatance::Float64(v) => v as i64,
        _PrimitiveDataInsatance::Int32(v) => v as i64,
        _PrimitiveDataInsatance::Int64(v) => v,
        _PrimitiveDataInsatance::Uint32(v) => v as i64,
        _PrimitiveDataInsatance::Uint64(v) => v as i64,
    }
}

#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn primitive_data_inst_as_u32(pd: *mut _PrimitiveDataContainer) -> u32 {
    match (*pd).0 {
        _PrimitiveDataInsatance::Float32(v) => v as u32,
        _PrimitiveDataInsatance::Float64(v) => v as u32,
        _PrimitiveDataInsatance::Int32(v) => v as u32,
        _PrimitiveDataInsatance::Int64(v) => v as u32,
        _PrimitiveDataInsatance::Uint32(v) => v,
        _PrimitiveDataInsatance::Uint64(v) => v as u32,
    }
}

#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn primitive_data_inst_as_u64(pd: *mut _PrimitiveDataContainer) -> u64 {
    match (*pd).0 {
        _PrimitiveDataInsatance::Float32(v) => v as u64,
        _PrimitiveDataInsatance::Float64(v) => v as u64,
        _PrimitiveDataInsatance::Int32(v) => v as u64,
        _PrimitiveDataInsatance::Int64(v) => v as u64,
        _PrimitiveDataInsatance::Uint32(v) => v as u64,
        _PrimitiveDataInsatance::Uint64(v) => v,
    }
}






////////////////////////////////////
// PRIMITIVE DATA TYPE ALLOCATORS //
////////////////////////////////////

pub struct _PrimitiveDataAllocator{
    properties: HashMap<u64, (u64, u64)>, // maps property ids to their index and type
    allocation_size: u64,
}

impl _PrimitiveDataAllocator {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            allocation_size: 0,
        }
    }

    pub fn add_property(&mut self, id: u64, type_: u64) {
        self.properties.insert(id, (self.allocation_size, type_));
        self.allocation_size += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn new_primitive_data_alloc() -> *mut _PrimitiveDataAllocator {
    Box::into_raw(Box::new(_PrimitiveDataAllocator::new()))
}

#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn delete_primitive_data_alloc(pda: *mut _PrimitiveDataAllocator) {
    if !pda.is_null() {
        Box::from_raw(pda);
    }
}

#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn primitive_data_alloc_add_property(pda: *mut _PrimitiveDataAllocator, id: u64, type_: u64) {
    (*pda).add_property(id, type_);
}