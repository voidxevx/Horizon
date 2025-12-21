#pragma once
#include <cstdint>

////////////////////////////////////////
// INTEROPERABLE PRIMITIVE DATA TYPES //
////////////////////////////////////////

struct _PrimitiveDataContainer;
extern "C"
{
	void delete_primitive_data_inst(_PrimitiveDataContainer*);
	_PrimitiveDataContainer* new_primitive_data_inst_f32(float);
	_PrimitiveDataContainer* new_primitive_data_inst_f64(double);
	_PrimitiveDataContainer* new_primitive_data_inst_i32(int32_t);
	_PrimitiveDataContainer* new_primitive_data_inst_i64(int64_t);
	_PrimitiveDataContainer* new_primitive_data_inst_u32(uint32_t);
	_PrimitiveDataContainer* new_primitive_data_inst_u64(uint64_t);
	float primitive_data_inst_as_f32(_PrimitiveDataContainer*);
	double primitive_data_inst_as_f64(_PrimitiveDataContainer*);
	int32_t primitive_data_inst_as_i32(_PrimitiveDataContainer*);
	int64_t primitive_data_inst_as_i64(_PrimitiveDataContainer*);
	uint32_t primitive_data_inst_as_u32(_PrimitiveDataContainer*);
	uint64_t primitive_data_inst_as_u64(_PrimitiveDataContainer*);
}

struct PrimitiveDataInstance
{
	_PrimitiveDataContainer* ptr;

	PrimitiveDataInstance(float v)    { ptr = new_primitive_data_inst_f32(v); }
	PrimitiveDataInstance(double v)   { ptr = new_primitive_data_inst_f64(v); }
	PrimitiveDataInstance(int32_t v)  { ptr = new_primitive_data_inst_i32(v); }
	PrimitiveDataInstance(int64_t v)  { ptr = new_primitive_data_inst_i64(v); }
	PrimitiveDataInstance(uint32_t v) { ptr = new_primitive_data_inst_u32(v); }
	PrimitiveDataInstance(uint64_t v) { ptr = new_primitive_data_inst_u64(v); }

	~PrimitiveDataInstance() { delete_primitive_data_inst(ptr); }

	inline operator float()    { return primitive_data_inst_as_f32(ptr); }
	inline operator double()   { return primitive_data_inst_as_f64(ptr); }
	inline operator int32_t()  { return primitive_data_inst_as_i32(ptr); }
	inline operator int64_t()  { return primitive_data_inst_as_i64(ptr); }
	inline operator uint32_t() { return primitive_data_inst_as_u32(ptr); }
	inline operator uint64_t() { return primitive_data_inst_as_u64(ptr); }

	inline operator _PrimitiveDataContainer* () { return ptr; }
};






////////////////////////////////////
// PRIMITIVE DATA TYPE ALLOCATORS //
////////////////////////////////////

struct _PrimitiveDataAllocator;
extern "C"
{
	_PrimitiveDataAllocator* new_primitive_data_alloc();
	void delete_primitive_data_alloc(_PrimitiveDataAllocator*);
	void primitive_data_alloc_add_property(_PrimitiveDataAllocator*, uint64_t, uint64_t);
}

struct PrimitiveDataAllocator
{
	_PrimitiveDataAllocator* ptr;

	PrimitiveDataAllocator() { ptr = new_primitive_data_alloc(); }
	~PrimitiveDataAllocator() { delete_primitive_data_alloc(ptr); }
	inline void addProperty(uint64_t id, uint64_t type) { primitive_data_alloc_add_property(ptr, id, type); }

	inline operator _PrimitiveDataAllocator* () { return ptr; }
};

