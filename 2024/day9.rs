use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

fn read_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn main() {
    let disk_map = env::args().nth(1).as_ref()
                        .map(|filename| read_to_string(filename))
                        .expect("need a file as input")
                        .expect("input can't be read")
                        .chars()
                        .filter_map(|num| num.to_digit(10))
                        .map(|num| num as u64)
                        .collect::<Vec<_>>();

    let mlen = disk_map.len();
    // indices into disk_map
    let mut front_idx = 0;
    let mut ffile = true;
    let mut blocks_filled = 0;

    let mut back_idx = mlen - 1;
    let mut bfile = mlen % 2 == 1;
    let mut blocks_taken = 0;

    // file ids
    let mut forward_fid = 0;
    let mut backward_fid = (back_idx as u64) / 2;

    // index into the "filesystem"
    let mut fs_idx = 0;

    let block_checksum = |id, start, end| {
        let sum = if end == 0 { 0 } else { end * (end - 1) / 2 };
        let sub_sum = if start == 0 { 0 } else { start * (start - 1) / 2 };
        id * (sum - sub_sum)
    };

    let mut checksum = 0;
    loop {
        if ffile {
            let end = fs_idx + disk_map[front_idx];
            checksum += block_checksum(forward_fid, fs_idx, end);
            fs_idx = end;

            forward_fid += 1;
            front_idx += 1;
            ffile = false;
        }
        else {
            if !bfile {
                back_idx -= 1;
                bfile = true;
            }

            let free_block_size = disk_map[front_idx] - blocks_filled;
            let back_block_size = disk_map[back_idx] - blocks_taken;
            let region_size;

            if free_block_size < back_block_size {
                blocks_taken += free_block_size;

                region_size = free_block_size;
            }
            else {
                blocks_filled += back_block_size;

                region_size = back_block_size;
            }

            let end = fs_idx + region_size;
            checksum += block_checksum(backward_fid, fs_idx, end);
            fs_idx = end;

            if free_block_size <= back_block_size {
                blocks_filled = 0;
                front_idx += 1;
                ffile = true;
            }

            if back_block_size <= free_block_size {
                blocks_taken = 0;
                backward_fid -= 1;
                back_idx -= 1;
                bfile = false;
            }
        }

        if front_idx >= back_idx { break; }
    }

    if ffile {
        let end = fs_idx + disk_map[front_idx] - blocks_taken;
        checksum += block_checksum(forward_fid, fs_idx, end);
    }

    println!("{}", checksum);
}
