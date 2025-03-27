use crate::ffi;

/// Fill rule dictates how intersection total is interpreted during rasterization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
pub enum FillRule {
    Negative = ffi::msdfgen_FillRule_FILL_NEGATIVE,
    NonZero = ffi::msdfgen_FillRule_FILL_NONZERO,
    Odd = ffi::msdfgen_FillRule_FILL_ODD,
    Positive = ffi::msdfgen_FillRule_FILL_POSITIVE,
}

impl Default for FillRule {
    fn default() -> Self {
        FillRule::NonZero
    }
}

impl FillRule {
    #[cfg(target_os = "windows")]
    pub(crate) fn into_raw(self) -> i32 {
        self as i32
    }

    #[cfg(not(target_os = "windows"))]
    pub(crate) fn into_raw(self) -> u32 {
        self as u32
    }

    /*pub(crate) fn from_raw(raw: u32) -> Self {
        unsafe { core::mem::transmute(raw) }
    }*/
}

/// Scanline object
pub struct Scanline {
    raw: ffi::msdfgen_Scanline,
}

impl Default for Scanline {
    fn default() -> Self {
        let raw = unsafe { ffi::msdfgen_Scanline::new() };
        Self { raw }
    }
}

impl Drop for Scanline {
    fn drop(&mut self) {
        unsafe { ffi::msdfgen_Scanline_destructor(&mut self.raw) }
    }
}

impl Scanline {
    pub(crate) fn as_raw(&self) -> &ffi::msdfgen_Scanline {
        &self.raw
    }

    pub(crate) fn as_raw_mut(&mut self) -> &mut ffi::msdfgen_Scanline {
        &mut self.raw
    }

    pub fn overlap(a: &Scanline, b: &Scanline, x_from: f64, x_to: f64, fill_rule: FillRule) -> f64 {
        unsafe {
            ffi::msdfgen_Scanline::overlap(
                a.as_raw(),
                b.as_raw(),
                x_from,
                x_to,
                fill_rule.into_raw(),
            )
        }
    }

    /// Returns the number of intersections left of x
    pub fn count_intersections(&self, x: f64) -> i32 {
        unsafe { self.raw.countIntersections(x) }
    }

    /// Returns the total sign of intersections left of x
    pub fn sum_intersections(&self, x: f64) -> i32 {
        unsafe { self.raw.sumIntersections(x) }
    }

    /// Decides whether the scanline is filled at x based on fill rule
    pub fn filled(&self, x: f64, fill_rule: FillRule) -> bool {
        unsafe { self.raw.filled(x, fill_rule.into_raw()) }
    }
}
