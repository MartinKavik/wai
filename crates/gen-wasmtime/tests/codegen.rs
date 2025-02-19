#![allow(dead_code, type_alias_bounds)]

fn main() {
    println!("compiled successfully!")
}

#[rustfmt::skip]
mod exports {
    test_helpers::codegen_wasmtime_export!(
        "*.wai"

        // TODO: implement async support
        "!async-functions.wai"

        // If you want to exclude a specific test you can include it here with
        // gitignore glob syntax:
        //
        // "!wasm.wai"
        // "!host.wai"
        //
        //
        // Similarly you can also just remove the `*.wai` glob and list tests
        // individually if you're debugging.
    );
}

mod imports {
    test_helpers::codegen_wasmtime_import!(
        "*.wai"

        // TODO: implement async support
        "!async-functions.wai"

        // TODO: these use push/pull buffer which isn't implemented in the test
        // generator just yet
        "!wasi-next.wai"
        "!host.wai"
    );
}

mod async_tests {
    mod not_async {
        wai_bindgen_wasmtime::export!({
            src["x"]: "foo: func()",
            async: ["bar"],
        });

        struct Me;

        impl x::X for Me {
            fn foo(&mut self) {}
        }
    }
    mod one_async {
        wai_bindgen_wasmtime::export!({
            src["x"]: "
                foo: func() -> list<u8>
                bar: func()
            ",
            async: ["bar"],
        });

        struct Me;

        #[wai_bindgen_wasmtime::async_trait]
        impl x::X for Me {
            fn foo(&mut self) -> Vec<u8> {
                Vec::new()
            }

            async fn bar(&mut self) {}
        }
    }
    mod one_async_export {
        wai_bindgen_wasmtime::import!({
            src["x"]: "
                foo: func(x: list<string>)
                bar: func()
            ",
            async: ["bar"],
        });
    }
    mod resource_with_none_async {
        wai_bindgen_wasmtime::export!({
            src["x"]: "
                resource y {
                    z: func() -> string
                }
            ",
            async: [],
        });
    }
}

mod custom_errors {
    wai_bindgen_wasmtime::export!({
        src["x"]: "
            foo: func()
            bar: func() -> expected<unit, u32>
            enum errno {
                bad1,
                bad2,
            }
            baz: func() -> expected<u32, errno>
        ",
        custom_error: true,
    });
}
