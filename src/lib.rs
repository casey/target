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
    #[cfg(target_arch = "hexagon")]
    {
        "hexagon"
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
    #[cfg(target_arch = "powerpc")]
    {
        "powerpc"
    }
    #[cfg(target_arch = "powerpc64")]
    {
        "powerpc64"
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
    #[cfg(target_os = "haiku")]
    {
        "haiku"
    }
    #[cfg(target_os = "ios")]
    {
        "ios"
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
    #[cfg(target_os = "openbsd")]
    {
        "openbsd"
    }
    #[cfg(target_os = "solaris")]
    {
        "solaris"
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
    #[cfg(target_env = "msvc")]
    {
        "msvc"
    }
    #[cfg(target_env = "musl")]
    {
        "musl"
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
    #[cfg(target_pointer_width = "32")]
    {
        "32"
    }
    #[cfg(target_pointer_width = "64")]
    {
        "64"
    }
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
pub fn vendor() -> &'static str {
  #[cfg(target_vendor = "apple"  )] { "apple"   }
  #[cfg(target_vendor = "pc"     )] { "pc"      }
  #[cfg(target_vendor = "unknown")] { "unknown" }
}

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
