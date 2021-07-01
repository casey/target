pub fn arch() -> &'static str {
    #[cfg(target_arch = "aarch64")]
    {
        "aarch64"
    }
    #[cfg(target_arch = "arm")]
    {
        "arm"
    }
    #[cfg(target_arch = "asmjs")]
    {
        "asmjs"
    }
    #[cfg(target_arch = "avr")]
    {
        "avr"
    }
    #[cfg(target_arch = "hexagon")]
    {
        "hexagon"
    }
    #[cfg(target_arch = "le32")]
    {
        "le32"
    }
    #[cfg(target_arch = "mips")]
    {
        "mips"
    }
    #[cfg(target_arch = "mips64")]
    {
        "mips64"
    }
    #[cfg(target_arch = "msp430")]
    {
        "msp430"
    }
    #[cfg(target_arch = "nvptx")]
    {
        "nvptx"
    }
    #[cfg(target_arch = "nvptx64")]
    {
        "nvptx64"
    }
    #[cfg(target_arch = "powerpc")]
    {
        "powerpc"
    }
    #[cfg(target_arch = "powerpc64")]
    {
        "powerpc64"
    }
    #[cfg(target_arch = "riscv32")]
    {
        "riscv32"
    }
    #[cfg(target_arch = "riscv64")]
    {
        "riscv64"
    }
    #[cfg(target_arch = "s390x")]
    {
        "s390x"
    }
    #[cfg(target_arch = "sparc")]
    {
        "sparc"
    }
    #[cfg(target_arch = "sparc64")]
    {
        "sparc64"
    }
    #[cfg(target_arch = "spriv")]
    {
        "spriv"
    }
    #[cfg(target_arch = "thumb")]
    {
        "thumb"
    }
    #[cfg(target_arch = "wasm32")]
    {
        "wasm32"
    }
    #[cfg(target_arch = "x86")]
    {
        "x86"
    }
    #[cfg(target_arch = "x86_64")]
    {
        "x86_64"
    }
    #[cfg(target_arch = "xcore")]
    {
        "xcore"
    }
}

pub fn os() -> &'static str {
    #[cfg(target_os = "android")]
    {
        "android"
    }
    #[cfg(target_os = "bitrig")]
    {
        "bitrig"
    }
    #[cfg(target_os = "cloudabi")]
    {
        "cloudabi"
    }
    #[cfg(target_os = "cuda")]
    {
        "cuda"
    }
    #[cfg(target_os = "dragonfly")]
    {
        "dragonfly"
    }
    #[cfg(target_os = "emscripten")]
    {
        "emscripten"
    }
    #[cfg(target_os = "freebsd")]
    {
        "freebsd"
    }
    #[cfg(target_os = "fuchsia")]
    {
        "fuchsia"
    }
    #[cfg(target_os = "haiku")]
    {
        "haiku"
    }
    #[cfg(target_os = "hermit")]
    {
        "hermit"
    }
    #[cfg(target_os = "illumos")]
    {
        "illumos"
    }
    #[cfg(target_os = "ios")]
    {
        "ios"
    }
    #[cfg(target_os = "l4re")]
    {
        "l4re"
    }
    #[cfg(target_os = "linux")]
    {
        "linux"
    }
    #[cfg(target_os = "macos")]
    {
        "macos"
    }
    #[cfg(target_os = "netbsd")]
    {
        "netbsd"
    }
    #[cfg(target_os = "none")]
    {
        "none"
    }
    #[cfg(target_os = "openbsd")]
    {
        "openbsd"
    }
    #[cfg(target_os = "psp")]
    {
        "psp"
    }
    #[cfg(target_os = "redox")]
    {
        "redox"
    }
    #[cfg(target_os = "sgx")]
    {
        "sgx"
    }
    #[cfg(target_os = "solaris")]
    {
        "solaris"
    }
    #[cfg(target_os = "tvos")]
    {
        "tvos"
    }
    #[cfg(target_os = "vxworks")]
    {
        "vxworks"
    }
    #[cfg(target_os = "watchos")]
    {
        "watchos"
    }
    #[cfg(target_os = "wasi")]
    {
        "wasi"
    }
    #[cfg(target_os = "windows")]
    {
        "windows"
    }
}

pub fn os_family() -> &'static str {
    #[cfg(unix)]
    {
        "unix"
    }
    #[cfg(wasm)]
    {
        "wasm"
    }
    #[cfg(windows)]
    {
        "windows"
    }
}

pub fn env() -> &'static str {
    #[cfg(target_env = "gnu")]
    {
        "gnu"
    }
    #[cfg(target_env = "libnx")]
    {
        "libnx"
    }
    #[cfg(target_env = "msvc")]
    {
        "msvc"
    }
    #[cfg(target_env = "musl")]
    {
        "musl"
    }
    #[cfg(target_env = "newlib")]
    {
        "newlib"
    }
    #[cfg(target_env = "uclibc")]
    {
        "uclibc"
    }
    #[cfg(target_env = "sgx")]
    {
        "sgx"
    }
    #[cfg(target_env = "")]
    {
        ""
    }
}

pub fn endian() -> &'static str {
    #[cfg(target_endian = "big")]
    {
        "big"
    }
    #[cfg(target_endian = "little")]
    {
        "little"
    }
}

pub fn pointer_width() -> &'static str {
    #[cfg(target_pointer_width = "8")]
    {
        "8"
    }
    #[cfg(target_pointer_width = "16")]
    {
        "16"
    }
    #[cfg(target_pointer_width = "32")]
    {
        "32"
    }
    #[cfg(target_pointer_width = "64")]
    {
        "64"
    }
}

pub fn vendor() -> &'static str {
    #[cfg(target_vendor = "apple")]
    {
        "apple"
    }
    #[cfg(target_vendor = "pc")]
    {
        "pc"
    }
    #[cfg(target_vendor = "sun")]
    {
        "sun"
    }
    #[cfg(target_vendor = "fortanix")]
    {
        "fortanix"
    }
    #[cfg(target_vendor = "unknown")]
    {
        "unknown"
    }
    #[cfg(target_vendor = "uwp")]
    {
        "uwp"
    }
}

pub fn features() -> &'static [&'static str] {
    &[
        #[cfg(target_feature = "aes")]
        "aes",
        #[cfg(target_feature = "avx")]
        "avx",
        #[cfg(target_feature = "avx2")]
        "avx2",
        #[cfg(target_feature = "bmi1")]
        "bmi1",
        #[cfg(target_feature = "bmi2")]
        "bmi2",
        #[cfg(target_feature = "fma")]
        "fma",
        #[cfg(target_feature = "fxsr")]
        "fxsr",
        #[cfg(target_feature = "lzcnt")]
        "lzcnt",
        #[cfg(target_feature = "pclmulqdq")]
        "pclmulqdq",
        #[cfg(target_feature = "popcnt")]
        "popcnt",
        #[cfg(target_feature = "rdrand")]
        "rdrand",
        #[cfg(target_feature = "rdseed")]
        "rdseed",
        #[cfg(target_feature = "sha")]
        "sha",
        #[cfg(target_feature = "sse")]
        "sse",
        #[cfg(target_feature = "sse2")]
        "sse2",
        #[cfg(target_feature = "sse3")]
        "sse3",
        #[cfg(target_feature = "sse4.1")]
        "sse4.1",
        #[cfg(target_feature = "sse4.2")]
        "sse4.2",
        #[cfg(target_feature = "ssse3")]
        "ssse3",
        #[cfg(target_feature = "xsave")]
        "xsave",
        #[cfg(target_feature = "xsavec")]
        "xsavec",
        #[cfg(target_feature = "xsaveopt")]
        "xsaveopt",
        #[cfg(target_feature = "xsaves")]
        "xsaves",
    ]
}

pub fn test() -> bool {
    #[cfg(test)]
    {
        true
    }
    #[cfg(not(test))]
    {
        false
    }
}

/*
pub fn has_atomic_8() -> bool {
  #[cfg(target_has_atomic     = "8")] { true  }
  #[cfg(not(target_has_atomic = "8")] { false }
}

pub fn has_atomic_16() -> bool {
  #[cfg(target_has_atomic     = "16")] { true  }
  #[cfg(not(target_has_atomic = "16")] { false }
}

pub fn has_atomic_32() -> bool {
  #[cfg(target_has_atomic     = "32")] { true  }
  #[cfg(not(target_has_atomic = "32")] { false }
}

pub fn has_atomic_64() -> bool {
  #[cfg(target_has_atomic     = "64")] { true  }
  #[cfg(not(target_has_atomic = "64")] { false }
}

pub fn has_atomic_ptr() -> bool {
  #[cfg(target_has_atomic     = "ptr")] { true  }
  #[cfg(not(target_has_atomic = "ptr")] { false }
}
*/
