fn setup_secp() {

    pub trait TellCargo {
        fn tell_cargo(&self);
    }

    ///--------------------------------
    pub enum WideMulFormat {
        Int128,
        Int64,
    }

    impl TellCargo for WideMulFormat {
        fn tell_cargo(&self) {
            match self {
                WideMulFormat::Int64  => println!("cargo:rustc-cfg=SECP256K1_WIDEMUL_INT64"),
                WideMulFormat::Int128 => println!("cargo:rustc-cfg=SECP256K1_WIDEMUL_INT128"),
            }
        }
    }

    ///--------------------------------
    pub struct BuildConfig {
        exhaustive_test_order: bool,
        use_basic_config:      bool,
        widemul_format:        WideMulFormat,
    }

    impl Default for BuildConfig {
        fn default() -> Self {
            Self {
                exhaustive_test_order: false,
                use_basic_config:      true,
                widemul_format:        WideMulFormat::Int64,
            }
        }
    }

    impl TellCargo for BuildConfig {
        fn tell_cargo(&self) {

            if self.exhaustive_test_order {
                println!("cargo:rustc-cfg=EXHAUSTIVE_TEST_ORDER")
            }

            if self.use_basic_config {
                println!("cargo:rustc-cfg=USE_BASIC_CONFIG")
            }

            self.widemul_format.tell_cargo();
        }
    }

    let cfg = BuildConfig::default();
    cfg.tell_cargo();
}

fn cfg_aliases() {
    use cfg_aliases::cfg_aliases;

    // Setup cfg aliases
    cfg_aliases! {
        // Platforms
        wasm:        { target_arch = "wasm32" },
        android:     { target_os = "android" },
        macos:       { target_os = "macos" },
        linux:       { target_os = "linux" },
        not_windows: { not(target_family = "windows") },
    }

    cfg_aliases! {
        i386: { target_arch = "i386" },
        i386_but_not_windows: { all(not(target_family = "windows"),target_arch = "i386") },
        x86_64_or_amd64: {
            any( 
                target_arch = "x64_64", 
                target_arch = "amd64" 
            )
        },
        x86_64_or_amd64_or_i386: {
            any( 
                target_arch = "x64_64", 
                target_arch = "amd64",
                target_arch = "i386" 
            )
        }
    }

    cfg_aliases! {
        have_getcpuid: {
            any(
                target_arch = "x86_64",
                target_arch = "amd64",
                target_arch = "i386"
            )
        }
    }
}

pub fn setup() {
    cfg_aliases();
    setup_secp();
}
