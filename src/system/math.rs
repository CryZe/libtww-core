use core::num::Float;

#[no_mangle]
pub unsafe extern "C" fn sqrtf(x: f32) -> f32 {
    extern "C" {
        #[link_name = "std::sqrtf(f32)"]
        fn sqrtf(x: f32) -> f32;
    }
    sqrtf(x)
}

#[no_mangle]
pub unsafe extern "C" fn ceilf(mut var0: f32) -> f32 {
    let mut var2: i32;
    let mut var3: i32;
    let var4: i32;
    'label0: loop {
        var2 = var0.to_bits() as i32;
        var3 = (var2 as u32).wrapping_shr(23i32 as u32) as i32 & 255i32;
        if (var3 as u32 > 149i32 as u32) as i32 != 0 {
            break 'label0;
        }
        'label1: loop {
            'label2: loop {
                if ((var3 as u32) < 127i32 as u32) as i32 != 0 {
                    break 'label2;
                }
                var3 = var3.wrapping_add(-127i32);
                var4 = (8388607i32 as u32).wrapping_shr(var3 as u32) as i32;
                if (var4 & var2 == 0) as i32 != 0 {
                    break 'label0;
                }
                (var0 + f32::from_bits(0x7B800000)).to_bits();
                var2 = ({
                    let a = var4;
                    let b = 0i32;
                    if (var2 > -1i32) as i32 != 0 {
                        a
                    } else {
                        b
                    }
                }).wrapping_add(var2)
                    & (-8388608i32).wrapping_shr(var3 as u32);
                break 'label1;
            }
            (var0 + f32::from_bits(0x7B800000)).to_bits();
            'label3: loop {
                if (var2 < 0i32) as i32 != 0 {
                    break 'label3;
                }
                var2 = {
                    let a = 1065353216i32;
                    let b = var2;
                    if var2 & 2147483647i32 != 0 {
                        a
                    } else {
                        b
                    }
                };
                break 'label1;
            }
            var2 = -2147483648i32;
            break;
        }
        var0 = f32::from_bits(var2 as u32);
        break;
    }
    var0
}

#[no_mangle]
pub unsafe extern "C" fn floorf(mut var0: f32) -> f32 {
    let var2: i32;
    let mut var3: i32;
    let var4: i32;
    'label0: loop {
        var2 = var0.to_bits() as i32;
        var3 = (var2 as u32).wrapping_shr(23i32 as u32) as i32 & 255i32;
        if (var3 as u32 > 149i32 as u32) as i32 != 0 {
            break 'label0;
        }
        'label1: loop {
            'label2: loop {
                if ((var3 as u32) < 127i32 as u32) as i32 != 0 {
                    break 'label2;
                }
                var3 = var3.wrapping_add(-127i32);
                var4 = (8388607i32 as u32).wrapping_shr(var3 as u32) as i32;
                if (var4 & var2 == 0) as i32 != 0 {
                    break 'label0;
                }
                (var0 + f32::from_bits(0x7B800000)).to_bits();
                var3 = (var2.wrapping_shr(31i32 as u32) & var4).wrapping_add(var2) & (-8388608i32).wrapping_shr(var3 as u32);
                break 'label1;
            }
            (var0 + f32::from_bits(0x7B800000)).to_bits();
            var3 = 0i32;
            if (var2 > -1i32) as i32 != 0 {
                break 'label1;
            }
            var3 = { let a = -1082130432i32; let b = var2; if var2 & 2147483647i32 != 0 { a } else { b } };
            break;
        }
        var0 = f32::from_bits(var3 as u32);
        break;
    }
    var0
}
