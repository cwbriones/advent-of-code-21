use crate::prelude::*;
use crate::search::SearchQueue;

//    1 2 3 4
//  01 2 3 4 56
// #############
// #01.3.5.7.9.#
// ###B#C#B#D###
//   #A#D#C#A#
//   #########
//
// 0 - 10
// rooms: 2 4 6 8
//
// ('A', 2)
// enum Pos {
//     Hall(usize, usize),
//     Room(usize, usize),
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Frog(usize);

impl Frog {
    fn new(c: char) -> Self {
        match c {
            'A' | 'B' | 'C' | 'D' => {}
            _ => panic!("bad char {}", c),
        }
        let i = (c as usize) - ('A' as usize);
        Frog(i)
    }

    fn cost(&self) -> usize {
        10usize.pow(self.0 as u32)
    }

    fn as_char(&self) -> char {
        *['A', 'B', 'C', 'D'].get(self.0).unwrap()
    }
}

fn parse(input: &str) -> [Room; 4] {
    let mut lines = input.lines().skip(2);
    let top = lines.next().unwrap().chars().filter(|c| c.is_alphabetic());
    let bottom = lines.next().unwrap().chars().filter(|c| c.is_alphabetic());

    let mut rooms = [
        Room {
            occupants: Vec::new(),
        },
        Room {
            occupants: Vec::new(),
        },
        Room {
            occupants: Vec::new(),
        },
        Room {
            occupants: Vec::new(),
        },
    ];
    for (i, (b, t)) in bottom.zip(top).enumerate() {
        rooms[i].occupants.push(Frog::new(b));
        rooms[i].occupants.push(Frog::new(t));
    }
    rooms
}

#[derive(Debug, Clone)]
struct Room {
    occupants: Vec<Frog>,
}

#[derive(Clone)]
struct SearchState {
    rooms: [Room; 4],
    hall: [Option<Frog>; 11],
}

fn part_one(rooms: [Room; 4]) -> usize {
    // println!("{:?}", rooms);
    let mut fringe = SearchQueue::new();
    // trying dfs since we should get *some* result
    fringe.push(
        0,
        SearchState {
            rooms,
            hall: [None; 11],
        },
    );
    let room_height = 2;
    while let Some((cost, state)) = fringe.pop() {
        if state
            .rooms
            .iter()
            .enumerate()
            .all(|(i, r)| r.occupants.len() == 2 && r.occupants.iter().all(|f| f.0 == i))
        {
            return cost;
        }
        // - move a frog from their origin room to the hall
        for (i, room) in state.rooms.iter().enumerate() {
            if room.occupants.iter().all(|f| f.0 == i) {
                // Don't need to move anything out of this room
                continue;
            }
            let room_x = (i + 1) * 2;
            // try moving the top-most frog in this room into the hall
            let mut x_start = room_x;
            while x_start >= 1 && state.hall[x_start - 1].is_none() {
                x_start -= 1;
            }
            let mut x_end = room_x;
            while x_end < 10 && state.hall[x_end + 1].is_none() {
                x_end += 1;
            }
            let mut new_rooms = state.rooms.clone();
            let frog = new_rooms[i].occupants.pop().unwrap();
            let y_cost = room_height - new_rooms[i].occupants.len();

            let x_iter = (x_start..room_x)
                .chain(room_x + 1..x_end) // may not need this because of filter
                .filter(|&x| x != 2 && x != 4 && x != 6 && x != 8);
            for x in x_iter {
                let x_cost = if x > room_x { x - room_x } else { room_x - x };
                let mut new_hall = state.hall;
                new_hall[x] = Some(frog);
                fringe.push(
                    cost + (y_cost + x_cost) * frog.cost(),
                    SearchState {
                        rooms: new_rooms.clone(),
                        hall: new_hall,
                    },
                );
            }
        }
        // - move a frog from the hall to their final position
        for (x_pos, frog) in state.hall.iter().enumerate() {
            let frog = match frog {
                Some(f) => f,
                None => continue,
            };
            if state.rooms[frog.0].occupants.iter().any(|of| of != frog) {
                // The destination room is unavailable
                continue;
            }
            let room_x = 2 * (frog.0 + 1);
            let (mut x_range, x_cost) = if room_x < x_pos {
                (room_x..x_pos, x_pos - room_x)
            } else {
                (x_pos + 1..room_x, room_x - x_pos)
            };
            if x_range.any(|x| state.hall[x].is_some()) {
                // Can move into destination but there is something in the way
                continue;
            }
            let mut new_hall = state.hall;
            new_hall[x_pos] = None;
            let mut new_rooms = state.rooms.clone();
            let y_cost = room_height - new_rooms[frog.0].occupants.len();
            new_rooms[frog.0].occupants.push(*frog);
            fringe.push(
                cost + (y_cost + x_cost) * frog.cost(),
                SearchState {
                    rooms: new_rooms,
                    hall: new_hall,
                },
            );
        }
    }
    panic!("no path found");
}

fn part_two(rooms: [Room; 4]) -> usize {
    0
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
