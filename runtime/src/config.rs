pub enum Platform {
  Linux,
  Windows,
  Solaris,
  VM
}

impl Platform {
  pub fn form_str(s: &str) -> Option<Platform> {
    match s {
      "linux" => Some(Platform::Linux),
      "windows" => Some(Platform::Windows),
      "solaris" => Some(Platform::Solaris),
      "vm" => Some(Platform::VM),
      _ => None
    }
  }

  pub fn as_str(&self) -> &'static str {
    match self {
      Platform::Linux => "linux",
      Platform::Windows => "windows",
      Platform::Solaris => "solaris",
      Platform::VM => "vm"
    }
  }
}

pub enum Arch {
  ArchX86,
  ArchX8664,
  ArchX32,
  ArchARM,
  ArchAARCH64,
  ArchMIPS,
  ArchMIPS64,
  ArchMIPS64N32,
  ArchMIPSEL,
  ArchMIPSEL64,
  ArchMIPSEL64N32,
  ArchPPC,
  ArchPPC64,
  ArchPPC64LE,
  ArchS390,
  ArchS390X,
  ArchPARISC,
  ArchPARISC64
}

impl Arch {
  pub fn form_str(s: &str) -> Option<Arch> {
    match s {
      "SCMP_ARCH_X86" => Some(Arch::ArchX86),
      "SCMP_ARCH_X86_64" => Some(Arch::ArchX8664),
      "SCMP_ARCH_X32" => Some(Arch::ArchX32),
      "SCMP_ARCH_ARM" => Some(Arch::ArchAARCH64),
      "SCMP_ARCH_AARCH64" => Some(Arch::ArchAARCH64),
      "SCMP_ARCH_MIPS" => Some(Arch::ArchMIPS),
      "SCMP_ARCH_MIPS64" => Some(Arch::ArchMIPS64),
      "SCMP_ARCH_MIPS64N32" => Some(Arch::ArchMIPS64N32),
      "SCMP_ARCH_MIPSEL" => Some(Arch::ArchMIPSEL),
      "SCMP_ARCH_MIPSEL64" => Some(Arch::ArchMIPSEL64),
      "SCMP_ARCH_MIPSEL64N32" => Some(Arch::ArchMIPSEL64N32),
      "SCMP_ARCH_PPC" => Some(Arch::ArchPPC),
      "SCMP_ARCH_PPC64" => Some(Arch::ArchPPC64),
      "SCMP_ARCH_PPC64LE" => Some(Arch::ArchPPC64LE),
      "SCMP_ARCH_S390" => Some(Arch::ArchS390),
      "SCMP_ARCH_S390X" => Some(Arch::ArchS390X),
      "SCMP_ARCH_PARISC" => Some(Arch::ArchPARISC),
      "SCMP_ARCH_PARISC64" => Some(Arch::ArchPARISC64),
      _ => None
    }
  }

  pub fn as_str(&self) -> &'static str {
    match self {
      Arch::ArchX86 => "SCMP_ARCH_X86",
      Arch::ArchX8664 => "SCMP_ARCH_X86_64",
      Arch::ArchX32 => "SCMP_ARCH_X32",
      Arch::ArchARM => "SCMP_ARCH_ARM",
      Arch::ArchAARCH64 => "SCMP_ARCH_AARCH64",
      Arch::ArchMIPS => "SCMP_ARCH_MIPS",
      Arch::ArchMIPS64 => "SCMP_ARCH_MIPS64",
      Arch::ArchMIPS64N32 => "SCMP_ARCH_MIPS64N32",
      Arch::ArchMIPSEL => "SCMP_ARCH_MIPSEL",
      Arch::ArchMIPSEL64 => "SCMP_ARCH_MIPSEL64",
      Arch::ArchMIPSEL64N32 => "SCMP_ARCH_MIPSEL64N32",
      Arch::ArchPPC => "SCMP_ARCH_PPC",
      Arch::ArchPPC64 => "SCMP_ARCH_PPC64",
      Arch::ArchPPC64LE => "SCMP_ARCH_PPC64LE",
      Arch::ArchS390 => "SCMP_ARCH_S390",
      Arch::ArchS390X => "SCMP_ARCH_S390X",
      Arch::ArchPARISC => "SCMP_ARCH_PARISC",
      Arch::ArchPARISC64 => "SCMP_ARCH_PARISC64"
    }
  }
}