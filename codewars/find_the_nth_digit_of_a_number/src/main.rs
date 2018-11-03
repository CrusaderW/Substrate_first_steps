fn x(n: usize) -> Vec<usize> {
    fn x_inner(n: usize, xs: &mut Vec<usize>) {
            if n >= 10 {
                        x_inner(n / 10, xs);
                    }
            xs.push(n % 10);
        }
    let mut xs = Vec::new();
    x_inner(n, &mut xs);
    xs
}



fn main() {
    let num: isize = -5;
    let pos_num: usize =
        if num < 0 {
            let inverse = num * -1;
            inverse as usize
        } else {
            num as usize
        };

    //let inverse: u32 = num * -1;
    println!("{:?}", num);
    let digits = x(5);
    println!("{:?}", digits);
}
