#![no_std]
use num_complex::Complex32;

/// Don't panic!  If you do, this will hang the page.
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

const WIDTH: usize = 360;
const HEIGHT: usize = 270;

/// The buffer is used with a "pointer cast" in javascript, so it must
/// match the ImageData structure there.
/// Each pixel is 0xaa_bb_gg_rr
#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

static mut C_VALUE: Complex32 = Complex32 { re: -0.5, im: 0.5 };
static mut ROTOR: Complex32 = Complex32 { re: 1., im: 0.002 };

#[no_mangle]
pub unsafe extern "C" fn go() {
    // This is called from JavaScript, and should *only* be
    // called from JavaScript. If you maintain that condition,
    // then we know that the &mut we're about to produce is
    // unique, and therefore safe.
    C_VALUE *= ROTOR;
    if C_VALUE.norm_sqr() > 2. || C_VALUE.norm_sqr() < 0.3 {
        ROTOR *= 1. / ROTOR.norm_sqr();
    }
    render_frame_safe(&mut BUFFER, C_VALUE)
}

// We split this out so that we can escape 'unsafe' as quickly
// as possible.
fn render_frame_safe(buffer: &mut [u32; WIDTH * HEIGHT], c: Complex32) {
    const S: f32 = 3.2 / (HEIGHT as f32);
    const X0: f32 = ((WIDTH / 2) as f32) * S;
    const Y0: f32 = ((HEIGHT / 2) as f32) * S;
    let max_iter = if has_inside(c) { 52 } else { 255 };
    for y in 0..HEIGHT / 2 {
        let zy = f32::from(y as i16) * S - Y0;
        for x in 0..WIDTH {
            let z = Complex32 {
                re: f32::from(x as i16) * S - X0,
                im: zy,
            };
            let val = pixel(julia(z, c, max_iter));
            buffer[y * WIDTH + x] = val;
            buffer[(HEIGHT - y) * WIDTH - x - 1] = val;
        }
    }
    if max_iter == 52 {
        for x in 2..5 {
            for y in 2..5 {
                buffer[y * WIDTH + x] = 0xff_ff_ff_ff;
            }
        }
    }
}

/// Sample some points which will probably be inside if there is a large
/// "black" area.
fn has_inside(c: Complex32) -> bool {
    julia(Complex32::new(0., 0.), c, 255) == 0
        && julia(Complex32::new(0.01, 0.), c, 255) == 0
}

fn pixel(i: u8) -> u32 {
    u32::from_be_bytes([0xff, i * 6, i + i / 2, i.saturating_mul(8)])
}
fn julia(z: Complex32, c: Complex32, max_i: u8) -> u8 {
    let mut z = z;
    for i in 1..max_i {
        if z.norm_sqr() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    if z.norm_sqr() > 4.0 {
        max_i
    } else {
        0
    }
}
