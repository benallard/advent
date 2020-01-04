pub fn process(program:&mut Vec<i32>){
    let mut idx=0;
    loop {
        let res;
        match program[idx] {
            1 => res = program[program[idx+1] as usize] + program[program[idx+2] as usize],
            2 => res = program[program[idx+1] as usize] * program[program[idx+2] as usize],
            99 => return,
            _ => panic!(program[idx]),
        }
        let residx = program[idx + 3] as usize;
        program[residx] = res;
        idx += 4;
    }
}

#[test]
fn examples(){
    let mut pgm = vec![1,0,0,0,99];
    process(&mut pgm);
    println!("{:?}", pgm);
    assert_eq!(pgm[0], 2);

    pgm = vec![2,3,0,3,99];
    process(&mut pgm);
    assert_eq!(pgm[3], 6);

    pgm = vec![2,4,4,5,99,0];
    process(&mut pgm);
    assert_eq!(pgm[5], 9801);

    pgm = vec![1,1,1,4,99,5,6,0,99];
    process(&mut pgm);
    assert_eq!(pgm[0], 30);
    assert_eq!(pgm[4], 2);
}
