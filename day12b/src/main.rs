use num::integer::lcm;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
struct Moon {
    pos: [i64; 3],
    vel: [i64; 3],
}

fn step(moons: &mut [Moon; 4]) {
    // gravity
    for a in 0..moons.len() {
        for b in (a + 1)..moons.len() {
            for n in 0..3 {
                if moons[a].pos[n] < moons[b].pos[n] {
                    moons[a].vel[n] += 1;
                    moons[b].vel[n] += -1;
                } else if moons[b].pos[n] < moons[a].pos[n] {
                    moons[a].vel[n] += -1;
                    moons[b].vel[n] += 1;
                }
            }
        }
    }

    // position
    for mut moon in moons {
        for n in 0..3 {
            moon.pos[n] += moon.vel[n];
        }
    }
}

fn main() {
    /*
    <x=1, y=2, z=-9>
    <x=-1, y=-9, z=-4>
    <x=17, y=6, z=8>
    <x=12, y=4, z=2>
    */
    let mut moons = [
        Moon {
            pos: [1, 2, -9],
            vel: [0, 0, 0],
        },
        Moon {
            pos: [-1, -9, -4],
            vel: [0, 0, 0],
        },
        Moon {
            pos: [17, 6, 8],
            vel: [0, 0, 0],
        },
        Moon {
            pos: [12, 4, 2],
            vel: [0, 0, 0],
        },
    ];

    let mut first_seen: [HashMap<[i64; 8], i64>; 3] =
        [HashMap::new(), HashMap::new(), HashMap::new()];
    let mut cycles: Vec<Option<(i64, i64)>> = vec![None, None, None];
    let mut i = 0;
    loop {
        for n in 0..3 {
            let signature = [
                moons[0].pos[n],
                moons[0].vel[n],
                moons[1].pos[n],
                moons[1].vel[n],
                moons[2].pos[n],
                moons[2].vel[n],
                moons[3].pos[n],
                moons[3].vel[n],
            ];
            match cycles[n] {
                Some(_) => (),
                None => {
                    if first_seen[n].contains_key(&signature) {
                        cycles[n] = Some((first_seen[n][&signature], i));
                    } else {
                        first_seen[n].insert(signature, i);
                    }
                }
            }
        }

        if cycles.iter().all(|c| c.is_some()) {
            println!("{:?}", cycles);

            // From observation, we know that the first-seen position for all
            // dimensions is the original position, meaning that each dimension
            // is on a cycle. So the LCM of all 3 numbers should be sufficient.
            let cycle_length: Vec<i64> =
                cycles.iter().map(|c| c.unwrap().1 - c.unwrap().0).collect();
            println!("lcm {}", cycle_length.iter().fold(1, |acc, v| lcm(acc, *v)));
            return;
        }

        step(&mut moons);
        i += 1;
    }
}
