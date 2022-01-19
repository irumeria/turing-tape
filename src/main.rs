use std::fs;
use std::vec::Vec;

#[derive(Clone, Debug)]
struct Machine<'a> {
    dist: [i32; 2],
    m: [&'a str; 2],
    out: [u8; 2],
}

fn main() {
    let order: String = String::from_utf8_lossy(&fs::read("./data/ord.tape").unwrap())
        .parse()
        .unwrap(); // read the order tape
    let mut input = fs::read("./data/i.tape")
        .unwrap()
        .iter()
        .map(|x| x - 48)
        .collect::<Vec<u8>>(); // read the input tape
    let split = order.split("\n").collect::<Vec<&str>>();
    let ord_split = split
        .iter()
        .filter(|x| x.chars().nth(0).unwrap() != '#')
        .collect::<Vec<&&str>>();

    let mut tms = vec![
        Machine {
            dist: [-1, -1],
            m: ["E", "E"],
            out: [3; 2]
        };
        ord_split.len()
    ];

    let _ = ord_split
        .iter()
        .map(|x| {
            let ary = x.split("->").collect::<Vec<&str>>();
            let ary_be = ary[0].split("-").collect::<Vec<&str>>();
            let ary_af = ary[1].split("-").collect::<Vec<&str>>();
            let this_index = ary_be[0].parse::<u32>().unwrap() as usize;
            let input_index = ary_be[1].parse::<u32>().unwrap() as usize;
            tms[this_index].dist[input_index] = ary_af[0].parse::<i32>().unwrap();
            let ary_ary_af = ary_af[1].split("").collect::<Vec<&str>>();
            tms[this_index].m[input_index] = ary_ary_af[2];
            tms[this_index].out[input_index] = ary_ary_af[1].parse::<u8>().unwrap();
        })
        .collect::<Vec<()>>();

    let mut tmp = &tms[0];
    let mut position = 0 as usize;

    loop {
        let inp = input[position];
        input[position] = tmp.out[inp as usize];

        match tmp.m[inp as usize] {
            "L" => position -= 1, // move left
            "R" => position += 1, // move right
            "S" => break,         // stop
            "E" => {
                println!("E");
                panic!();
            } // illegal state order
            _ => panic!(),        // illegal move order
        }
        tmp = &tms[tmp.dist[inp as usize] as usize];
    }

    println!("{:?}", input);
}
