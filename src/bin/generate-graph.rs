use std::collections::{HashMap, HashSet};
use synacor_vm::VM;

const ADVENTURE_LOOP_START: u16 = 2756;
const ITEMS_POSITION: usize = 27395;
const START_LOCATION: usize = 2339;
const END_LOCATION: usize = 2665;
const FORCED_MAZE_LOCATION_FROM: usize = 2434;
const FORCED_MAZE_LOCATION_TO: usize = 2439;
const TELEPORT_LOCATION: usize = 2485;
const SYNACOR_LOCATION: usize = 2510;
const BEACH_LOCATION: usize = 2520;

struct Item {
    name: String,
    _description: String,
    _location: usize,
    _use_fn: u16
}

struct Location {
    id: usize,
    name: String,
    _description: String,
    connections: Vec<(String, usize)>,
    enter_fn: u16
}

fn get_string(memory: &[u16], position: u16) -> String {
    let position = position as usize;
    let size = memory[position] as usize;

    memory[position + 1 ..= position + size]
        .iter()
        .map(|v| *v as u8 as char)
        .collect()
}

fn get_items_by_location(memory: &[u16]) -> HashMap<usize, Vec<Item>> {
    let mut items_by_location: HashMap<usize, Vec<Item>> = HashMap::new();
    let items_size = memory[ITEMS_POSITION] as usize;

    for item_ptr in &memory[ITEMS_POSITION + 1 ..= ITEMS_POSITION + items_size] {
        let item_ptr = *item_ptr as usize;
        let location = memory[item_ptr + 2] as usize;

        items_by_location
            .entry(location)
            .or_default()
            .push(Item {
                name: get_string(memory, memory[item_ptr]),
                _description: get_string(memory, memory[item_ptr + 1]),
                _location: location,
                _use_fn: memory[item_ptr + 3]
            });
    }

    items_by_location
}

fn get_locations(memory: &[u16]) -> Vec<Location> {
    let mut pending = vec![BEACH_LOCATION, SYNACOR_LOCATION, FORCED_MAZE_LOCATION_TO, START_LOCATION];
    let mut visited: HashSet<usize> = pending.clone().into_iter().collect();
    let mut locations = vec![];

    while !pending.is_empty() {
        let location_id = pending.pop().unwrap();
        let mut location = Location {
            id: location_id,
            name: get_string(memory, memory[location_id]),
            _description: get_string(memory, memory[location_id + 1]),
            connections: match location_id {
                // Hardcode unnamed connections that happen through code
                FORCED_MAZE_LOCATION_FROM => vec![("always".to_string(), FORCED_MAZE_LOCATION_TO)],
                TELEPORT_LOCATION => vec![
                    ("use teleport at min energy level".to_string(), SYNACOR_LOCATION),
                    ("use teleport at specific energy level".to_string(), BEACH_LOCATION)
                ],
                _ => vec![]
            },
            enter_fn: memory[location_id + 4]
        };
        let connection_names = memory[location_id + 2] as usize;
        let connection_ids = memory[location_id + 3] as usize;

        for connection_idx in 1 ..= memory[connection_names] {
            let connection_idx = connection_idx as usize;
            let next_location_id = memory[connection_ids + connection_idx] as usize;
            let next_location_name = get_string(memory, memory[connection_names + connection_idx]);

            location.connections.push((next_location_name, next_location_id));

            if !visited.contains(&next_location_id) {
                pending.push(next_location_id);
                visited.insert(next_location_id);
            }
        }

        locations.push(location);
    }

    locations
}

fn generate_graph_dot(locations: &Vec<Location>, items_by_location: &HashMap<usize, Vec<Item>>) {
    println!("digraph G {{");

    for location in locations {
        let items_part = items_by_location
            .get(&location.id)
            .unwrap_or(&Vec::new())
            .iter()
            .map(|item| item.name.clone())
            .collect::<Vec<String>>()
            .join(", ");
        let label_part = vec![
            format!("[{}] {}", location.id, location.name),
            items_part,
            if location.enter_fn == 0 { String::new() } else { format!("enter_fn {}", location.enter_fn) }
        ]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join("\n");
        let fill_part = match location.id {
            START_LOCATION | END_LOCATION                     => r#", fillcolor="palegreen", style="filled""#,
            _ if items_by_location.contains_key(&location.id) => r#", fillcolor="paleturquoise", style="filled""#,
            _                                                 => ""
        };

        println!(r#"    {id} [label="{label}"{fill}]"#, id=location.id, label=label_part, fill=fill_part);
    }

    for location in locations {
        for (connection_name, connection_id) in &location.connections {
            println!(r#"    {} -> {} [label="{}"]"#, location.id, connection_id, connection_name);
        }
    }

    println!("}}");
}


fn main() {
    let mut vm = VM::new();

    vm.load_binary("files/challenge.bin");
    vm.dbg_add_breakpoint(ADVENTURE_LOOP_START);
    vm.dbg_set_output_enabled(false);
    vm.run();

    let memory = vm.dbg_get_memory();
    let items_by_location = get_items_by_location(memory);
    let locations = get_locations(memory);

    generate_graph_dot(&locations, &items_by_location);
}
