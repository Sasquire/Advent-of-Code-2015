use std::collections::HashMap;

// TODO this whole process should be cleaner and solved for more generic "wire" problems
// Perhaps it's best to model it as a directed graph? Maybe even make its own library

#[derive(Debug, PartialEq)]
enum Action {
	Assignment,
	Not,
	And,
	Or,
	Rshift,
	Lshift
}

#[derive(Debug)]
struct Rule {
	from_a: Option<String>,
	from_b: Option<String>,
	to: String,
	action: Action
}

fn parse_input (input: &str) -> Vec<Rule> {
	return input.lines()
		.map(|line| line.split(" ").map(|x| String::from(x)).collect::<Vec<_>>())
		.map(|line| (
			if line.len() == 3 { // Direct assignment
				return Rule {
					from_a: Some(line[0].clone()),
					from_b: None,
					to: line[2].clone(),
					action: Action::Assignment
				};
			} else if line.len() == 4 { // Not
				return Rule {
					from_a: Some(line[1].clone()),
					from_b: None,
					to: line[3].clone(),
					action: Action::Not
				};
			} else {
				return Rule {
					from_a: Some(line[0].clone()),
					from_b: Some(line[2].clone()),
					to: line[4].clone(),
					action: match line[1].as_str() {
						"AND" => Action::And,
						"OR" => Action::Or,
						"RSHIFT" => Action::Rshift,
						"LSHIFT" => Action::Lshift,
						_ => panic!()
					}
				};
			}

		))
		.collect::<Vec<_>>();
}

fn parse_or_lookup (map: &HashMap<String, u16>, key: String) -> Option<u16> {
	match u16::from_str_radix(key.as_str(), 10) {
		Ok(x) => Some(x),
		Err(_) => map.get(&key).copied()
	}
}

fn apply_rules (machine_state: &mut HashMap<String, u16>, rules: &Vec<Rule>) {
	let mut applied_rules = vec![false; rules.len()];

	loop {
		let mut some_rule_changed = false;

		for i in 0..applied_rules.len() {
			if applied_rules[i] == true { continue; }
			let rule = &rules[i];

			let from_a = {
				let from_a = parse_or_lookup(&machine_state, rule.from_a.clone().unwrap());
				if from_a.is_none() { continue; } // from_a is always required
				from_a.unwrap()
			};

			let from_b = if rule.action == Action::Assignment || rule.action == Action::Not {
				None
			} else {
				let from_b = parse_or_lookup(&machine_state, rule.from_b.clone().unwrap());
				if from_b.is_none() { continue; } // from_b is required for double rules
				from_b
			};
			
			let to = parse_or_lookup(&machine_state, rule.to.clone());
			if to.is_some() { continue; } // Prevent accidental overwrites
			
			let result = match rule.action {
				Action::Assignment => from_a,
				Action::Not => !from_a,
				Action::And => from_a & from_b.unwrap(),
				Action::Or => from_a | from_b.unwrap(),
				Action::Rshift => from_a >> from_b.unwrap(),
				Action::Lshift => from_a << from_b.unwrap()
			};

			machine_state.insert(rule.to.clone(), result);
			applied_rules[i] = true;
			some_rule_changed = true;
		}

		if some_rule_changed == false { break; }
	}
}

pub fn day_07_1 (input: &str) -> String {
	let rules = parse_input(input);
	let mut machine_state = HashMap::new();
	apply_rules(&mut machine_state, &rules);
	return machine_state.get("a").unwrap().to_string();
}

pub fn day_07_2 (input: &str) -> String {
	let rules = parse_input(input);
	
	// Run the first part
	let mut machine_state = HashMap::new();
	apply_rules(&mut machine_state, &rules);
	let first_a = *machine_state.get("a").unwrap();

	// Run the second part
	let mut machine_state = HashMap::new();
	machine_state.insert(String::from("b"), first_a);
	apply_rules(&mut machine_state, &rules);
	
	return machine_state.get("a").unwrap().to_string();
}