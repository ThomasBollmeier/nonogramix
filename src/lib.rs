use std::{collections::HashMap, cell::Cell};

#[derive(Clone, Copy, Debug)]
struct Placement {
    offset: u8, 
    size: u8,
}

enum CellState {
    Unknown,
    Filled,
    Empty,
}

fn find_placements(
    blocks: &Vec<u8>, 
    size: u8, 
    givens: &HashMap<u8, CellState>) -> Vec<Vec<Placement>> {

    find_placements_at(blocks, size, 0, givens)

}

fn find_placements_at(
    blocks: &Vec<u8>, 
    size: u8, 
    offset: u8, 
    givens: &HashMap<u8, CellState>) -> Vec<Vec<Placement>> {


    if blocks.is_empty() {
        return vec![vec![]];
    }

    let block = blocks[0];
    let remaining = if blocks.len() > 0 {
        blocks[1..].to_vec()
    } else {
        vec![]
    };
    let remaining_size: u8 = remaining.iter().sum::<u8>() + remaining.len() as u8;

    if size - offset < block + remaining_size {
        return vec![];
    } 

    let mut ret = vec![];
    let max_offset = size - block - remaining_size;

    for offs in offset..=max_offset {
        
        let p = Placement{
            offset: offs,
            size: block,
        };

        for ps in find_placements_at(&remaining, size, offs + block + 1, givens) {
            let mut new_ps = vec![p];
            new_ps.extend(ps);
            if placements_valid(&new_ps, givens) {
                ret.push(new_ps);
            }
        }
    }
    
    ret
}

fn placements_valid(placements: &Vec<Placement>, givens: &HashMap<u8, CellState>) -> bool {

    let mut cnt_filled = givens
        .values()
        .fold(0u8, |acc, v| {
            match v {
                CellState::Filled => acc + 1,
                _ => acc,
            }
        });

    for placement in placements {
        
        let start = placement.offset;
        let end = start + placement.size;

        for offset in start..end {

            match givens.get(&offset) {
                Some(state) => match state {
                    CellState::Empty => return false,
                    CellState::Filled => { cnt_filled -= 1; },
                    _ => (),
                },
                None => (),
            }

        }
    }

    cnt_filled == 0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_placements_works() {

        let placements = find_placements(&vec![1, 5, 1], 10, &HashMap::new());

        assert_eq!(placements.len(), 4);

        for ps in placements {
            println!("{:?}", ps);
        }

    }

    #[test]
    fn find_placements_works_with_givens() {

        let mut givens = HashMap::new();
        givens.insert(1, CellState::Empty);
        givens.insert(9, CellState::Empty);

        let placements = find_placements(&vec![1, 5, 1], 10, &givens);

        assert_eq!(placements.len(), 1);

        for ps in placements {
            println!("{:?}", ps);
        }

    }

}
