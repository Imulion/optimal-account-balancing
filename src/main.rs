use std::{fmt::Display, time::Instant};

#[derive(Clone)]
struct Transaction {
    amount: f64,
    from_index: usize,
    to_index: usize,
}

impl Transaction {
    fn print(&self, persons: &[Person]) {
        println!(
            "{} --{}€--> {}",
            persons[self.from_index].name, self.amount, persons[self.to_index].name
        )
    }
}

#[derive(Clone)]
struct Person {
    name: String,
    owed: f64,
}

impl Person {
    fn new(name: &str, owed: f64) -> Self {
        Person {
            name: name.to_string(),
            owed,
        }
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{} ( Owed =  {}€)", self.name, self.owed).as_str())
    }
}

fn main() {
    let persons: Vec<Person> = vec![
        Person::new("Dave", 10.0),
        Person::new("Dieter", 35.0),
        Person::new("Thorben", 100.0),
        Person::new("Daniel", 0.0),
        Person::new("Leon", 0.0),
    ];
    if let Some(transactions) = get_minimum_transactions(persons.clone()) {
        println!(
            "Minimum Number of Transactions found: {}",
            transactions.len()
        );
        transactions.iter().for_each(|transaction| {
            transaction.print(&persons);
        });
    } else {
        println!("No Result found")
    }
}

fn adjust_for_share(persons: &mut [Person]) {
    let share = persons.iter().map(|p| p.owed).sum::<f64>() / persons.len() as f64;
    persons.iter_mut().for_each(|person| person.owed -= share);
}

fn get_minimum_transactions(mut persons: Vec<Person>) -> Option<Vec<Transaction>> {
    // println!("Initial Depts:");
    // persons.iter().for_each(|p| {
    //     println!("{p}");
    // });
    // println!();

    adjust_for_share(&mut persons);
    // println!("Adjusted for Share:");
    // persons.iter().for_each(|p| {
    //     println!("{p}");
    // });
    // println!();

    search_best_transaction(persons, Vec::new(), usize::MAX, 0)
}

fn search_best_transaction(
    persons: Vec<Person>,
    previous: Vec<Transaction>,
    current_best: usize,
    _start_index: usize,
) -> Option<Vec<Transaction>> {
    if previous.len() == current_best {
        return None;
    }
    let (p1_i, p1) = match persons.iter().enumerate().find(|(_i, p)| p.owed != 0.0) {
        Some(p) => p,
        None => return Some(previous),
    };
    let results: Vec<Vec<Transaction>> = persons
        .iter()
        .enumerate()
        .skip(p1_i)
        .filter(|(p_i, p)| {
            !(p.owed.is_sign_positive() && p1.owed.is_sign_positive())
                && p_i != &p1_i
                && p.owed != 0.0
        })
        .flat_map(|(p_i, _p)| {
            let mut new_persons = persons.clone();
            let mut new_transactions = previous.clone();
            new_persons.get_mut(p_i).unwrap().owed += p1.owed;
            new_persons.get_mut(p1_i).unwrap().owed = 0.0;

            if p1.owed.is_sign_positive() {
                new_transactions.push(Transaction {
                    amount: p1.owed,
                    from_index: p_i,
                    to_index: p1_i,
                });
            } else {
                new_transactions.push(Transaction {
                    amount: -p1.owed,
                    from_index: p1_i,
                    to_index: p_i,
                });
            }
            search_best_transaction(
                new_persons,
                new_transactions,
                current_best,
                _start_index + 1,
            )
        })
        .collect();
    match results.into_iter().min_by(|l1, l2| l1.len().cmp(&l2.len())) {
        Some(list) if list.len() < current_best => Some(list),
        _ => None,
    }
}

#[test]
fn test_min_ddtdl() {
    let persons = vec![
        Person::new("Dave", 10.0),
        Person::new("Dieter", 35.0),
        Person::new("Thorben", 100.0),
        Person::new("Daniel", 0.0),
        Person::new("Leon", 0.0),
    ];
    let timer = Instant::now();
    let result = get_minimum_transactions(persons);
    let base_time = timer.elapsed();
    println!("it took  ({base_time:.1?})");
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 4);
}

#[test]
fn test_min_daniel() {
    let persons = vec![
        Person::new("Daniel", 13.0),
        Person::new("Thorben", 7.0),
        Person::new("Leon", 3.0),
        Person::new("Patrick", 7.0),
        Person::new("Michael", 0.0),
        Person::new("Jonas", 0.0),
    ];
    let p1 = persons.clone();
    let timer = Instant::now();
    let result = get_minimum_transactions(persons);
    let base_time = timer.elapsed();

    println!("it took  ({base_time:.1?})");
    assert!(result.is_some());
    let transactions = result.unwrap();
    println!(
        "Minimum Number of Transactions found: {}",
        transactions.len()
    );
    transactions.iter().for_each(|transaction| {
        transaction.print(&p1);
    });
    assert_eq!(transactions.len(), 4);
}
