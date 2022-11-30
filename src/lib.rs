#![feature(maybe_uninit_uninit_array)]

#[macro_export]
macro_rules! arr {
    ($type:ty; $count:literal; $init:expr) => {
        unsafe {
            let mut array: [std::mem::MaybeUninit<$type>; $count] =
                std::mem::MaybeUninit::uninit_array();
            for element in &mut array[..] {
                *element = std::mem::MaybeUninit::new($init);
            }
            std::mem::transmute::<_, [$type; $count]>(array)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_numbers() {
        let array0 = arr![u8; 4; 1];
        assert_eq!(array0.len(), 4);
        assert_eq!(array0[0], 1);
        assert_eq!(array0[1], 1);
        assert_eq!(array0[2], 1);
        assert_eq!(array0[3], 1);
    }
}
