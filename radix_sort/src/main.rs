fn main() {
    let mut input = [23,46,7,885,4367,21,3546,655];
    radix_sort(&mut input);
    assert_eq!(input, [7, 21, 23, 46, 655, 885, 3546, 4367]);
    println!("{:#?}", input);
}

pub fn radix_sort<const LEN: usize>(input: &mut [usize; LEN]) {
    let mut output = [0; LEN];
    let mut sum = [0_usize; 256];
    let mut mask = 255;
    const ITERATIONS: usize = std::mem::size_of::<usize>();

    for i in 0..ITERATIONS {
        for num in input.iter() {
            sum[(num & mask) >> (i * 8)] += 1;
        }

        let mut last_sum = 0;
        for index in 0..256 {
            sum[index] = sum[index] + last_sum;
            last_sum = sum[index];
        }
        drop(last_sum);

        for val in input.iter().rev() {
            sum[(val & mask) >> (i * 8)] -= 1;
            let index = sum[(val & mask) >> (i * 8)];
            output[index] = *val;
        }

        sum.fill(0);
        std::mem::swap(&mut output, input);
        mask <<= 8;
    }
}