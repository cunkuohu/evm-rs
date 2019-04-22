use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;

use evmjit::compiler::attributes::LLVMAttributeFactory;
use evmjit::compiler::evmconstants::EvmConstants;
use evmjit::compiler::evmtypes::EvmTypes;
use evmjit::compiler::callback::CallbackTypes;
use evmjit::compiler::memory::mem_representation::MemoryRepresentationType;
use evmjit::compiler::runtime::{
    env::EnvDataType, rt_data_type::RuntimeDataType, rt_type::RuntimeType
};

/// The context for the JIT. Includes data that is bound to the LLVM context for a given execution.
#[derive(Debug)]
pub struct JITContext {
    /// The LLVM context.
    m_llvm_ctx: Context,
    /// The LLVM builder.
    m_builder: Builder,
    /// The LLVM module.
    m_module: Module,
    /// The EVM-specific type provider.
    m_evm_types: EvmTypes,
    /// The EVM constant provider.
    m_evm_constants: EvmConstants,
    /// The LLVM attribute provider.
    m_attrs: LLVMAttributeFactory,
    /// The runtime data type provider.
    m_rt_data: RuntimeDataType,
    /// The environment type provider.
    m_env: EnvDataType,
    /// The callback signature provider.
    m_callbacks: CallbackTypes,
    /// The memory representation type provider.
    m_memrep: MemoryRepresentationType,
    /// The runtime type provider.
    m_rt: RuntimeType,
}

impl JITContext {
    pub fn new() -> Self {
        let ctx = Context::create();
        let builder = ctx.create_builder();
        let module = ctx.create_module("evm");
        let types = EvmTypes::new(&ctx);
        let constants = EvmConstants::new(&ctx);
        let attr_factory = LLVMAttributeFactory::new(&ctx);
        let rt_data = RuntimeDataType::new(&ctx);
        let env = EnvDataType::new(&ctx);
        let callbacks = CallbackTypes::new(&ctx, &types, &env);
        let memrep = MemoryRepresentationType::new(&ctx);
        let rt = RuntimeType::new(&ctx, &rt_data, &env, &memrep);

        JITContext {
            m_llvm_ctx: ctx,
            m_builder: builder,
            m_module: module,
            m_evm_types: types,
            m_evm_constants: constants,
            m_attrs: attr_factory,
            m_rt_data: rt_data,
            m_env: env,
            m_callbacks: callbacks,
            m_memrep: memrep,
            m_rt: rt,
        }
    }

    // TODO: explore alternate ways of exposing these interfaces.
    pub fn llvm_context(&self) -> &Context {
        &self.m_llvm_ctx
    }

    pub fn builder(&self) -> &Builder {
        &self.m_builder
    }

    pub fn module(&self) -> &Module {
        &self.m_module
    }

    pub fn evm_types(&self) -> &EvmTypes {
        &self.m_evm_types
    }

    pub fn evm_constants(&self) -> &EvmConstants {
        &self.m_evm_constants
    }

    pub fn attributes(&self) -> &LLVMAttributeFactory {
        &self.m_attrs
    }

    pub fn rt_data(&self) -> &RuntimeDataType {
        &self.m_rt_data
    }

    pub fn env(&self) -> &EnvDataType {
        &self.m_env
    }

    pub fn callback_types(&self) -> &CallbackTypes {
        &self.m_callbacks
    }

    pub fn memrep(&self) -> &MemoryRepresentationType {
        &self.m_memrep
    }

    pub fn rt(&self) -> &RuntimeType {
        &self.m_rt
    }
}
