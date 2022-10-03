#![feature(rustc_private)]

extern crate rustc_codegen_llvm;
extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_target;

use rustc_codegen_llvm::context::CodegenCx;
use rustc_codegen_llvm::llvm::*;
use rustc_codegen_llvm::ModuleLlvm;
use rustc_codegen_ssa::traits::*;
use rustc_data_structures::small_c_str::SmallCStr;
use rustc_hir::def_id::{DefId, LOCAL_CRATE};
use rustc_middle::ty::{Instance, TyCtxt};

pub fn eval_func<'tcx>(tcx: TyCtxt<'_>, func_id: DefId) {
    rustc_codegen_llvm::llvm_util::init(tcx.sess);

    let module_name = tcx.crate_name(LOCAL_CRATE);

    let llvm_module = ModuleLlvm::new(tcx, module_name.as_str());

    let cx = CodegenCx::new(tcx, None, &llvm_module);

    let instance = Instance::mono(tcx, func_id);
    cx.get_fn(instance);

    unsafe {
        LLVMDumpModule(cx.llmod);

        LLVMLinkInMCJIT();

        let ee = LLVMRustCreateExecutionEngineForModule(cx.llmod);

        let addr =
            LLVMGetFunctionAddress(ee, SmallCStr::new(tcx.symbol_name(instance).name).as_ptr());
        let f: extern "C" fn() -> i32 = std::mem::transmute(addr);
        dbg!(f());
    }
}
