#![allow(dead_code)]

use std::ffi::CString;
use singletonum::{Singleton, SingletonInit};
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::types::StructType;
use inkwell::types::PointerType;
use inkwell::values::PointerValue;
use inkwell::values::BasicValueEnum;
use inkwell::AddressSpace;
use super::super::memory::mem_representation::MemoryRepresentationType;
use super::env::EnvDataType;
use super::rt_data_type::RuntimeDataType;
use super::rt_data_type::RuntimeDataFieldToIndex;
use super::rt_data_type::RuntimeDataFieldToName;
use super::rt_data_type::RuntimeDataTypeFields::Gas;
use super::rt_data_type::RuntimeDataTypeFields::GasPrice;
use super::rt_data_type::RuntimeDataTypeFields::CallData;
use super::rt_data_type::RuntimeDataTypeFields::CallDataSize;
use super::rt_data_type::RuntimeDataTypeFields::Value;
use super::rt_data_type::RuntimeDataTypeFields::Code;
use super::rt_data_type::RuntimeDataTypeFields::CodeSize;
use super::rt_data_type::RuntimeDataTypeFields::Address;
use super::rt_data_type::RuntimeDataTypeFields::Sender;
use super::rt_data_type::RuntimeDataTypeFields::Depth;

//use super::rt_data_type::NUM_RUNTIME_DATA_FIELDS;

#[derive(Debug, Singleton)]

// RuntimeType is the struct that the JIT will build to pass
// arguments from the VM to the contract at runtime

pub struct RuntimeType
{
    rt_type: StructType,
    rt_ptr_type: PointerType,
}

unsafe impl Sync for RuntimeType {}
unsafe impl Send for RuntimeType {}

impl SingletonInit for RuntimeType {
    type Init = Context;
    fn init(context: &Context) -> Self {
        let rt_data_ptr = RuntimeDataType::get_instance(&context).get_ptr_type();
        let env_ptr = EnvDataType::get_instance(&context).get_ptr_type();
        let mem_ptr = MemoryRepresentationType::get_instance(&context).get_type();

        let fields = [rt_data_ptr.into(), env_ptr.into(), mem_ptr.into()];
        let rt_struct = context.opaque_struct_type("Runtime");
        rt_struct.set_body(&fields, false);

        RuntimeType {
            rt_type : rt_struct,
            rt_ptr_type : rt_struct.ptr_type(AddressSpace::Generic)
        }
    }
}

impl RuntimeType {
    pub fn get_type(&self) -> StructType {
        self.rt_type
    }

    pub fn get_ptr_type(&self) -> PointerType {
        self.rt_ptr_type
    }

    pub fn is_runtime_type(a_struct: &StructType) -> bool {
        if !a_struct.is_sized() {
            return false;
        }

        if a_struct.count_fields() != 3 {
            return false;
        }

        if a_struct.is_packed() {
            return false;
        }

        if a_struct.is_opaque() {
            return false;
        }

        if a_struct.get_name() != Some(&*CString::new("Runtime").unwrap()) {
            return false;
        }

        let field1 = a_struct.get_field_type_at_index(0).unwrap();
        if !field1.is_pointer_type() {
            return false;
        }

        let field1_element_t = field1.as_pointer_type().get_element_type();
        if !field1_element_t.is_struct_type() {
            return false;
        }

        if !RuntimeDataType::is_rt_data_type(&field1_element_t.as_struct_type()) {
            return false;
        }

        let field2 = a_struct.get_field_type_at_index(1).unwrap();
        if !field2.is_pointer_type() {
            return false;
        }

        let field2_element_t = field2.as_pointer_type().get_element_type();
        if !field2_element_t.is_struct_type() {
            return false;
        }

        if !EnvDataType::is_env_data_type(&&field2_element_t.as_struct_type()) {
            return false;
        }

        let field3 = a_struct.get_field_type_at_index(2).unwrap();
        if !field3.is_struct_type() {
            return false;
        }

        let field3_element_t = field3.as_struct_type();
        if !MemoryRepresentationType::is_mem_representation_type(&field3_element_t) {
            return false;
        }

        true
    }
}

pub struct RuntimeTypeManager {
    m_data_ptr: BasicValueEnum,
    m_mem_ptr: PointerValue,
    m_env_ptr: BasicValueEnum,
    m_rt_data_elts: [BasicValueEnum; 10],
}

impl RuntimeTypeManager {
    pub fn new(context: &Context, builder: &Builder) -> RuntimeTypeManager {

        let rt_ptr = RuntimeTypeManager::get_runtime_ptr_with_builder(&context, &builder);
        unsafe {
            let data_p = builder.build_load (builder.build_struct_gep(rt_ptr.into_pointer_value(), 0, ""), "dataPtr");
            assert_eq!(data_p.get_type().into_pointer_type(), RuntimeDataType::get_instance(context).get_ptr_type());

            let mem_p = builder.build_struct_gep(rt_ptr.into_pointer_value(), 2, "mem");

            assert_eq!(mem_p.get_type(), MemoryRepresentationType::get_instance(&context).get_ptr_type());

            let env_p = builder.build_load (builder.build_struct_gep(rt_ptr.into_pointer_value(), 1, ""), "env");
            assert_eq!(env_p.get_type().into_pointer_type(), EnvDataType::get_instance(&context).get_ptr_type());

            let data = builder.build_load (*data_p.as_pointer_value(), "data");

            RuntimeTypeManager {
                m_data_ptr: data_p,
                m_mem_ptr: mem_p,
                m_env_ptr: env_p,
                m_rt_data_elts: [builder.build_extract_value(data.into_struct_value(),
                                                             Gas.to_index() as u32,
                                                             Gas.to_name()),
                                 builder.build_extract_value(data.into_struct_value(),
                                                             GasPrice.to_index() as u32,
                                                             GasPrice.to_name()),
                                 builder.build_extract_value(data.into_struct_value(),
                                                             CallData.to_index() as u32,
                                                             CallData.to_name()),
                                 builder.build_extract_value(data.into_struct_value(),
                                                             CallDataSize.to_index() as u32,
                                                             CallDataSize.to_name()),
                                 builder.build_extract_value(data.into_struct_value(),
                                                             Value.to_index() as u32,
                                                             Value.to_name()),
                                 builder.build_extract_value(data.into_struct_value(),
                                                             Code.to_index() as u32,
                                                             Code.to_name()),
                                 builder.build_extract_value(data.into_struct_value(),
                                                             CodeSize.to_index() as u32,
                                                             CodeSize.to_name()),
                                 builder.build_extract_value(data.into_struct_value(),
                                                             Address.to_index() as u32,
                                                             Address.to_name()),
                                 builder.build_extract_value(data.into_struct_value(),
                                                             Sender.to_index() as u32,
                                                             Sender.to_name()),
                                 builder.build_extract_value(data.into_struct_value(),
                                                             Depth.to_index() as u32,
                                                             Depth.to_name())],
            }
        }
    }

    fn get_runtime_ptr_with_builder(context: & Context, builder: & Builder) -> BasicValueEnum {
        // The parent of the first basic block is a function
        
        let bb = builder.get_insert_block();
        assert!(bb != None);
        
        let func = bb.unwrap().get_parent();
        assert!(func != None);
        let func_val = func.unwrap();
        
        // The first argument to a function is a pointer to the runtime
        assert!(func_val.count_params() > 0);

        let runtime_ptr = func_val.get_first_param().unwrap();
        assert_eq!(runtime_ptr.get_type().into_pointer_type(), RuntimeType::get_instance(context).get_ptr_type());

        runtime_ptr
    }

    pub fn get_env_ptr(self) -> BasicValueEnum {
        self.m_env_ptr
    }

    pub fn get_data_ptr(self) -> BasicValueEnum {
        self.m_data_ptr
    }

    pub fn get_mem_ptr(self) -> PointerValue {
        self.m_mem_ptr
    }

    pub fn get_address(self) -> BasicValueEnum {
        self.m_rt_data_elts[Address.to_index()]
    }

    pub fn get_sender(self) -> BasicValueEnum {
        self.m_rt_data_elts[Sender.to_index()]
    }

    pub fn get_value(self) -> BasicValueEnum {
        self.m_rt_data_elts[Value.to_index()]
    }

    pub fn get_depth(self) -> BasicValueEnum {
        self.m_rt_data_elts[Depth.to_index()]
    }

}

#[test]

fn test_runtime_type() {
    let context = Context::create();
    let rt_type_singleton = RuntimeType::get_instance(&context);
    let rt_struct = rt_type_singleton.get_type();

    assert!(RuntimeType::is_runtime_type(&rt_struct));

    // Test that we have a pointer to RuntimeData

    let rt_struct_ptr = rt_type_singleton.get_ptr_type();
    assert!(rt_struct_ptr.get_element_type().is_struct_type());
    assert!(RuntimeType::is_runtime_type (rt_struct_ptr.get_element_type().as_struct_type()));
}
