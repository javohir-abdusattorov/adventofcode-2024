use itertools::Itertools;


pub fn part_1(input: String) -> i128 {
    let mut r = 0;
    let mut v = input
        .split("")
        .filter_map(|str| str.parse::<usize>().ok())
        .enumerate()
        .fold((vec![], 0), |(mut acc, p), (index, n)| {
            if index % 2 != 0 {
                acc.append(&mut vec![-1; n]);
                (acc, p)
            } else {
                acc.append(&mut vec![p; n]);
                r = acc.len() - 1;
                (acc, p + 1)
            }
        })
        .0;

    for i in 0..v.len() {
        if v[i] != -1 { continue; }
        if r <= i { break; }

        v.swap(i, r);
        while v[r] == -1 { r -= 1 }
    }

    v.into_iter()
        .enumerate()
        .filter(|(_, n)| *n != -1)
        .fold(0, |acc: i128, (i, n)| acc + (n as i128 * i as i128))
}

pub fn part_2(input: String) -> i128 {
    let (mut mem, file_slots, mut open_slots, _) = input
        .split("")
        .filter_map(|str| str.parse::<usize>().ok())
        .enumerate()
        .fold((vec![], vec![], vec![], 0), |(mut acc, mut file_slots, mut open_slots, p), (index, n)| {
            if index % 2 != 0 {
                open_slots.push((acc.len(), n));
                acc.append(&mut vec![-1; n]);
                (acc, file_slots, open_slots, p)
            }
            else {
                file_slots.push((acc.len(), n));
                acc.append(&mut vec![p; n]);
                (acc, file_slots, open_slots, p + 1)
            }
        });

    for (i, size) in file_slots.into_iter().skip(1).rev() {
        if let Some((pos, open_slot)) = open_slots.iter().filter(|slot| slot.0 < i).find_position(|slot| slot.1 >= size) {
            (0..size).for_each(|k| mem.swap(k + open_slot.0, k + i));

            open_slots[pos] = (open_slots[pos].0 + size, open_slots[pos].1 - size);
            if open_slots[pos].1 <= 0 {
                open_slots.remove(pos);
            }
        }
    }

    mem.into_iter()
        .enumerate()
        .filter(|(_, n)| *n != -1)
        .fold(0, |acc: i128, (i, n)| acc + (n as i128 * i as i128))
}