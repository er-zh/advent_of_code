fn main() {
    let run_program = |init_value| {
        let mut output = vec![];

        let mut a = init_value;
        let mut b = 0;
        let mut c = 0;

        loop {
            b = a % 8;
            b = b ^ 5;
            c = a / (1 << b);
            b = b ^ 6;
            b = b ^ c;

            output.push(b % 8);
            a = a / 8;

            if a == 0 {
                break;
            }
        }

        output
    };

    // part 1
    //println!("{:?}", run_program(44374556u32));

    let instructions = vec![2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 1, 5, 5, 3, 0];
    let end = instructions.len();
    let mut quine = 0u64;

    for i in (0..end).rev() {
        let subset = &instructions[i..end];

        loop {
            let temp = run_program(quine);

            if temp == subset {
                println!("{}", quine);
                break;
            }

            quine += 1;
        }

        quine *= 8;
    }
}
