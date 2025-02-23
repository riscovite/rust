//! Raw system calls on RISCovite.
#![allow(dead_code)]

use core::arch::asm;

pub(super) mod defs;

/// The raw type of system call arguments.
pub(super) type V = u64;

/// A newtype for error codes.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Error(pub V);

impl Error {
    #[inline]
    pub fn is(self, err_def: defs::Error) -> bool {
        self.0 == (err_def as u64)
    }
}

/// Raw system call result type.
#[repr(C)]
pub(super) struct Result {
    pub value: V,
    pub error: Error,
}

impl Result {
    #[inline]
    pub fn is_error(&self) -> bool {
        self.error.0 != 0
    }

    #[inline]
    pub fn as_io<T: From<V>>(self) -> crate::io::Result<T> {
        self.as_io_map(|v| v.into())
    }

    pub fn as_io_map<T>(self, m: impl FnOnce(V) -> T) -> crate::io::Result<T> {
        if self.is_error() {
            crate::io::Result::Err(crate::io::Error::from_raw_os_error(self.error.0))
        } else {
            crate::io::Result::Ok(m(self.value))
        }
    }
}

impl core::ops::Try for Result {
    type Output = V;
    type Residual = Error;

    #[inline(always)]
    fn from_output(output: Self::Output) -> Self {
        Self {
            value: output,
            error: Error(0),
        }
    }

    #[inline]
    fn branch(self) -> core::ops::ControlFlow<Self::Residual, Self::Output> {
        if self.error.0 == 0 {
            core::ops::ControlFlow::Continue(self.value)
        } else {
            core::ops::ControlFlow::Break(self.error)
        }
    }
}

impl core::ops::FromResidual<Error> for Result {
    #[inline(always)]
    fn from_residual(residual: Error) -> Self {
        Self {
            value: 0,
            error: residual,
        }
    }
}

/// Call into a system function with no arguments.
#[inline(always)]
pub(super) unsafe fn syscall0(n: V) -> Result {
    let value: V;
    let error: V;
    unsafe {
        asm!(
            "ecall",
            in("a7") n,
            out("a0") value,
            out("a1") error,
        )
    };
    Result {
        value,
        error: Error(error),
    }
}

/// Call into a system function with one argument.
#[inline(always)]
pub(super) unsafe fn syscall1(n: V, a0: V) -> Result {
    let value: V;
    let error: V;
    unsafe {
        asm!(
            "ecall",
            in("a7") n,
            inout("a0") a0 => value,
            out("a1") error,
        )
    };
    Result {
        value,
        error: Error(error),
    }
}

/// Call into a system function with two arguments.
#[inline(always)]
pub(super) unsafe fn syscall2(n: V, a0: V, a1: V) -> Result {
    let value: V;
    let error: V;
    unsafe {
        asm!(
            "ecall",
            in("a7") n,
            inout("a0") a0 => value,
            inout("a1") a1 => error,
        )
    };
    Result {
        value,
        error: Error(error),
    }
}

/// Call into a system function with three arguments.
#[inline(always)]
pub(super) unsafe fn syscall3(n: V, a0: V, a1: V, a2: V) -> Result {
    let value: V;
    let error: V;
    unsafe {
        asm!(
            "ecall",
            in("a7") n,
            inout("a0") a0 => value,
            inout("a1") a1 => error,
            in("a2") a2,
        )
    };
    Result {
        value,
        error: Error(error),
    }
}

/// Call into a system function with four arguments.
#[inline(always)]
pub(super) unsafe fn syscall4(n: V, a0: V, a1: V, a2: V, a3: V) -> Result {
    let value: V;
    let error: V;
    unsafe {
        asm!(
            "ecall",
            in("a7") n,
            inout("a0") a0 => value,
            inout("a1") a1 => error,
            in("a2") a2,
            in("a3") a3,
        )
    };
    Result {
        value,
        error: Error(error),
    }
}

/// Call into a system function with five arguments.
#[inline(always)]
pub(super) unsafe fn syscall5(n: V, a0: V, a1: V, a2: V, a3: V, a4: V) -> Result {
    let value: V;
    let error: V;
    unsafe {
        asm!(
            "ecall",
            in("a7") n,
            inout("a0") a0 => value,
            inout("a1") a1 => error,
            in("a2") a2,
            in("a3") a3,
            in("a4") a4,
        )
    };
    Result {
        value,
        error: Error(error),
    }
}

/// Call into a system function with six arguments.
#[inline(always)]
pub(super) unsafe fn syscall6(n: V, a0: V, a1: V, a2: V, a3: V, a4: V, a5: V) -> Result {
    let value: V;
    let error: V;
    unsafe {
        asm!(
            "ecall",
            in("a7") n,
            inout("a0") a0 => value,
            inout("a1") a1 => error,
            in("a2") a2,
            in("a3") a3,
            in("a4") a4,
            in("a5") a5,
        )
    };
    Result {
        value,
        error: Error(error),
    }
}

/// Call into a system function with seven arguments.
#[inline(always)]
pub(super) unsafe fn syscall7(n: V, a0: V, a1: V, a2: V, a3: V, a4: V, a5: V, a6: V) -> Result {
    let value: V;
    let error: V;
    unsafe {
        asm!(
            "ecall",
            in("a7") n,
            inout("a0") a0 => value,
            inout("a1") a1 => error,
            in("a2") a2,
            in("a3") a3,
            in("a4") a4,
            in("a5") a5,
            in("a6") a6,
        )
    };
    Result {
        value,
        error: Error(error),
    }
}

macro_rules! syscall {
    ($n:expr $(,)?) => {
        $crate::sys::pal::riscovite::syscall::syscall0($n)
    };
    ($n:expr, $a0:expr $(,)?) => {
        $crate::sys::pal::riscovite::syscall::syscall1($n, $a0)
    };
    ($n:expr, $a0:expr, $a1:expr $(,)?) => {
        $crate::sys::pal::riscovite::syscall::syscall2($n, $a0, $a1)
    };
    ($n:expr, $a0:expr, $a1:expr, $a2:expr $(,)?) => {
        $crate::sys::pal::riscovite::syscall::syscall3($n, $a0, $a1, $a2)
    };
    ($n:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr $(,)?) => {
        $crate::sys::pal::riscovite::syscall::syscall4($n, $a0, $a1, $a2, $a3)
    };
    ($n:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr $(,)?) => {
        $crate::sys::pal::riscovite::syscall::syscall5($n, $a0, $a1, $a2, $a3, $a4)
    };
    ($n:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr $(,)?) => {
        $crate::sys::pal::riscovite::syscall::syscall6($n, $a0, $a1, $a2, $a3, $a4, $a5)
    };
    ($n:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr $(,)?) => {
        $crate::sys::pal::riscovite::syscall::syscall7($n, $a0, $a1, $a2, $a3, $a4, $a5, $a6)
    };
}
pub(super) use syscall;

/// Calculate a system function number for an interface-specific function.
///
/// Interface-specific function numbers are built from two parts: a slot number
/// and a local function number. Each handle has 16 slots numbered 0-15 which
/// can each be assigned to a different interface. Each interface in turn defines
/// its own 32-bit local function numbers whose meaning is interface-specific.
///
/// The slot number and the function number are packed together to produce
/// a number that can be used as a function number when making a system call.
/// Whenever a result from this function is used as a function number, the
/// first call argument (`a0`) must be the handle that the function is being
/// called on.
#[inline(always)]
pub(super) const fn interface_func_num(slot: InterfaceSlot, n: u32) -> V {
    // The numbering scheme uses the lower four bits to capture the slot
    // number and then the next 32 bits to capture the function number.
    // That whole result is then bitwise-negated so that the top
    // 28 bits are always 1 and the overall resulting number can
    // potentially be in the range reachable by a 12-bit signed
    // immediate in e.g. an ADDI instruction, if `n` is low enough.
    let raw: u64 = (slot.number() as u64) | ((n as u64) << 4);
    !raw
}

/// Represents an interface-specific function slot number.
///
/// Each handle has 16 interface slots numbered 0-15, where each can be
/// bound to a different interface. This newtype is a `u8` that is
/// guaranteed to be in the required range.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct InterfaceSlot(u8);

/// Convenient predefined constants for the 16 valid interface slot numbers.
///
/// Use this to concisely construct interface-specific function numbers in
/// const contexts:
///
/// ```rust
/// IFACE_SLOT[0].func_num(2)
/// ```
///
/// Interface-specific function slots are primarily intended for situations
/// where a wrapping type arranges for specific interfaces to be in specific
/// slots decided at compile time, making constant values convenient and
/// most appropriate.
///
/// Although it is technically possible to construct interface-specific
/// function numbers dynamically at runtime, that is not a usage pattern that
/// RISCovite's design was optimized for. If multiple different Rust objects
/// need to use different interfaces on the same device then it's better to
/// produce a separate handle for each one, e.g. using `dup`, and then
/// configure each handle with different interfaces in each slot so that
/// each wrapping object can have its own const-selected function numbers.
pub(super) const IFACE_SLOT: [InterfaceSlot; 16] = [
    InterfaceSlot::new_const::<0>(),
    InterfaceSlot::new_const::<1>(),
    InterfaceSlot::new_const::<2>(),
    InterfaceSlot::new_const::<3>(),
    InterfaceSlot::new_const::<4>(),
    InterfaceSlot::new_const::<5>(),
    InterfaceSlot::new_const::<6>(),
    InterfaceSlot::new_const::<7>(),
    InterfaceSlot::new_const::<8>(),
    InterfaceSlot::new_const::<9>(),
    InterfaceSlot::new_const::<10>(),
    InterfaceSlot::new_const::<11>(),
    InterfaceSlot::new_const::<12>(),
    InterfaceSlot::new_const::<13>(),
    InterfaceSlot::new_const::<14>(),
    InterfaceSlot::new_const::<15>(),
];

impl InterfaceSlot {
    /// Build an  [`InterfaceSlot`] from a const generic argument.
    #[inline(always)]
    pub const fn new_const<const RAW: u8>() -> Self {
        if RAW > 0xf {
            panic!("const generic RAW not in range 0-15")
        }
        Self(RAW)
    }

    /// Wrap the given value in [`InterfaceSlot`] without checking whether
    /// it is in the correct range.
    ///
    /// # Safety
    ///
    /// Callers must ensure that the value is less than 16.
    #[inline(always)]
    pub const unsafe fn new_unchecked(raw: u8) -> Self {
        Self(raw)
    }

    /// Try to use the given number as an interface slot, returning `Err(())`
    /// if the value is not in the expected range.
    #[inline(always)]
    pub const fn numbered(num: u8) -> core::result::Result<Self, ()> {
        if num <= 0xf {
            Ok(Self(num))
        } else {
            Err(())
        }
    }

    /// Get the raw slot number as a `u8` whose value is guaranteed to be
    /// less than 16.
    #[inline(always)]
    pub const fn number(self) -> u8 {
        self.0
    }

    #[inline(always)]
    pub const fn func_num(self, num: u32) -> u64 {
        interface_func_num(self, num)
    }
}
