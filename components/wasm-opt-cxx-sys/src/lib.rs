pub use cxx;

pub mod ffi {
    #[cxx::bridge(namespace = "Colors")]
    pub mod colors {
        unsafe extern "C++" {
            include!("shims.h");

            fn setEnabled(enabled: bool);
        }
    }

    #[cxx::bridge(namespace = "wasm_shims")]
    pub mod wasm {
        unsafe extern "C++" {
            include!("shims.h");

            type Module;

            fn newModule() -> UniquePtr<Module>;
        }

        unsafe extern "C++" {
            include!("shims.h");

            type ModuleReader;

            fn newModuleReader() -> UniquePtr<ModuleReader>;

            fn ModuleReader_readText(
                reader: Pin<&mut ModuleReader>,
                filename: &CxxString,
                wasm: Pin<&mut Module>,
            ) -> Result<()>;

            fn ModuleReader_readBinary(
                reader: Pin<&mut ModuleReader>,
                filename: &CxxString,
                wasm: Pin<&mut Module>,
                sourceMapFilename: &CxxString,
            ) -> Result<()>;

            fn ModuleReader_read(
                reader: Pin<&mut ModuleReader>,
                filename: &CxxString,
                wasm: Pin<&mut Module>,
                sourceMapFilename: &CxxString,
            ) -> Result<()>;
        }

        unsafe extern "C++" {
            include!("shims.h");

            type ModuleWriter;

            fn newModuleWriter() -> UniquePtr<ModuleWriter>;

            fn ModuleWriter_setDebugInfo(writer: Pin<&mut ModuleWriter>, debug: bool);
            
            fn ModuleWriter_writeText(
                writer: Pin<&mut ModuleWriter>,
                wasm: Pin<&mut Module>,
                filename: &CxxString,
            ) -> Result<()>;

            fn ModuleWriter_writeBinary(
                writer: Pin<&mut ModuleWriter>,
                wasm: Pin<&mut Module>,
                filename: &CxxString,
            ) -> Result<()>;
        }

        unsafe extern "C++" {
            include!("shims.h");

            type InliningOptions;

            fn newInliningOptions() -> UniquePtr<InliningOptions>;

            fn setAlwaysInlineMaxSize(self: Pin<&mut Self>, size: u32);

            fn setOneCallerInlineMaxSize(self: Pin<&mut Self>, size: u32);

            fn setFlexibleInlineMaxSize(self: Pin<&mut Self>, size: u32);

            fn setAllowFunctionsWithLoops(self: Pin<&mut Self>, allow: bool);

            fn setPartialInliningIfs(self: Pin<&mut Self>, number: u32);
        }

        unsafe extern "C++" {
            include!("shims.h");

            type PassOptions;

            fn newPassOptions() -> UniquePtr<PassOptions>;

            fn setDebug(self: Pin<&mut Self>, debug: bool);

            fn setValidate(self: Pin<&mut Self>, validate: bool);

            fn setValidateGlobally(self: Pin<&mut Self>, validate: bool);

            fn setOptimizeLevel(self: Pin<&mut Self>, level: i32);

            fn setShrinkLevel(self: Pin<&mut Self>, level: i32);

            fn setInliningOptions(self: Pin<&mut Self>, inlining: UniquePtr<InliningOptions>);

            fn setTrapsNeverHappen(self: Pin<&mut Self>, ignoreTraps: bool);

            fn setLowMemoryUnused(self: Pin<&mut Self>, memoryUnused: bool);

            fn setFastMath(self: Pin<&mut Self>, fastMath: bool);

            fn setZeroFilledMemory(self: Pin<&mut Self>, zeroFilledMemory: bool);

            fn setDebugInfo(self: Pin<&mut Self>, debugInfo: bool);
        }

        unsafe extern "C++" {
            include!("shims.h");

            type PassRunner<'wasm>;

            fn newPassRunner<'wasm>(wasm: Pin<&'wasm mut Module>) -> UniquePtr<PassRunner<'wasm>>;

            fn newPassRunnerWithOptions<'wasm>(
                wasm: Pin<&'wasm mut Module>,
                options: UniquePtr<PassOptions>,
            ) -> UniquePtr<PassRunner<'wasm>>;

            fn addDefaultOptimizationPasses(self: Pin<&mut Self>);

            fn run(self: Pin<&mut Self>);
        }
    }
}

/// Hack to establish linage to wasm-opt-sys.
///
/// See docs for wasm_opt_sys::init.
///
/// FIXME: reevaluate this later
#[doc(hidden)]
pub fn init() -> anyhow::Result<()> {
    wasm_opt_sys::init();

    Ok(())
}
