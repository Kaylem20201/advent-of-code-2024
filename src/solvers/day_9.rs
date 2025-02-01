struct ParsedInput {
    discmap: Vec<usize>,
}

pub fn solve(input: String) -> (String, String) {
    let parsed_input = parse_input(&input);

    println!("Parsed input.");
    println!("{:?}", parsed_input.discmap);

    let part1_answer = part_1(&parsed_input);
    let part2_answer = part_2(&parsed_input);

    return (part1_answer, part2_answer);
}

fn part_1(input: &ParsedInput) -> String {
    let mut discmap = input.discmap.clone();
    let mut i = 0;
    let mut j = discmap.len() - 1;
    while i <= j {
        let [start, end] = [discmap[i], discmap[j]];
        if start != usize::MAX {
            i += 1;
            continue;
        }
        if end == usize::MAX {
            j -= 1;
            continue;
        }
        discmap[i] = end;
        discmap[j] = usize::MAX;
        i += 1;
        j -= 1;
    }

    discmap
        .into_iter()
        .take_while(|ele| *ele != usize::MAX)
        .enumerate()
        .fold(0, |sum, (i, ele)| sum + (ele * i))
        .to_string()
}

fn part_2(input: &ParsedInput) -> String {
    let mut discmap = input.discmap.clone();
    let mut j = discmap.len() - 1;
    while j != usize::MAX {
        let end = discmap[j];
        if end == usize::MAX {
            if j == 0 {
                break;
            }
            j -= 1;
            continue;
        }
        //j is at block
        let [block_start, block_end] = [find_start_of_block(&discmap, j), j + 1];
        let block_size = block_end - block_start;
        let mut i = 0;
        while i < block_start {
            if discmap[i] != usize::MAX {
                i += 1;
                continue;
            }
            let [free_start, free_end] = [i, find_end_of_block(&discmap, i)];
            println!(
                "free block: [{:?}..{:?}], candidate block: [{:?}..{:?}], candidate size: {:?}",
                free_start, free_end, block_start, block_end, block_size
            );
            if block_size > (free_end - free_start) {
                println!(
                    "Not enough free space for {:?} at {:?}",
                    discmap[block_start], free_start
                );
                i = free_end;
                continue;
            }
            break;
        }
        if i >= block_start {
            println!("Could not move block {:?}", discmap[block_start]);
            if block_start == 0 {
                break;
            }
            j = block_start - 1;
            continue;
        }
        println!("Swapping blocks at {:?} and {:?}", i, j);
        let check = discmap[j];
        discmap[i] = check;
        discmap[j] = usize::MAX;
        while j > block_start {
            j -= 1;
            i += 1;
            discmap[i] = check;
            discmap[j] = usize::MAX;
        }
    }

    println!("{:?}", discmap);

    discmap
        .into_iter()
        .enumerate()
        .fold(0, |sum, (i, ele)| {
            if ele == usize::MAX {
                return sum;
            }
            sum + (ele * i)
        })
        .to_string()
}

fn find_end_of_block(discmap: &Vec<usize>, start_index: usize) -> usize {
    //returns index after end of block
    if start_index >= discmap.len() - 1 {
        return discmap.len();
    }
    let check = discmap.get(start_index).unwrap();
    let mut i = start_index;
    loop {
        if discmap[i] == *check {
            i += 1;
            if i >= discmap.len() {
                return discmap.len();
            }
            continue;
        }
        return i;
    }
}

fn find_start_of_block(discmap: &Vec<usize>, start_index: usize) -> usize {
    //returns index AT BEGINNING (INCLUSIVE) of block
    if start_index == 0 {
        return 0;
    }
    let check = discmap.get(start_index).unwrap();
    let mut i = start_index;
    loop {
        if discmap[i] == *check {
            i -= 1;
            if i == 0 {
                return 0;
            }
            continue;
        }
        return i + 1;
    }
}

fn parse_input(input: &str) -> ParsedInput {
    let chars = input.chars().collect::<Vec<char>>();
    let files = chars.chunks_exact(2);

    let mut discmap: Vec<usize> = Vec::new();
    files.enumerate().for_each(|(index, file)| {
        println!("{:?}, {:?}", index, file);
        let [filesize, free_space]: [u8; 2] = [
            char::to_digit(file[0], 10).unwrap().try_into().unwrap(),
            char::to_digit(file[1], 10)
                .unwrap_or_else(|| 0)
                .try_into()
                .unwrap(),
        ];
        for _ in 0..filesize {
            discmap.push(index);
        }
        for _ in 0..free_space {
            discmap.push(usize::MAX);
        }
    });

    ParsedInput { discmap }
}
