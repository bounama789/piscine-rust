pub mod mall;
use std::ops::Sub;

pub use floor::store::employee::*;
pub use mall::floor::*;
use mall::guard::Guard;
pub use mall::*;

pub fn biggest_store(mall: Mall) -> store::Store {

    let empty_store = store::Store::new("", 0, [].to_vec());
    let u = mall
        .floors
        .iter()
        .max_by(|x, y| {
            let a = x
                .stores
                .iter()
                .max_by(|x1, y1| x1.square_meters.cmp(&y1.square_meters))
                .unwrap_or(&empty_store);
            let b = y
                .stores
                .iter()
                .max_by(|x1, y1| x1.square_meters.cmp(&y1.square_meters))
                .unwrap_or(&empty_store);

            a.square_meters.cmp(&b.square_meters)
        })
        .unwrap();

    u.stores
        .iter()
        .max_by_key(|store| store.square_meters)
        .unwrap()
        .clone()
}

pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    let mut highest_paid:Vec<store::employee::Employee> = Vec::new();

    for floor in mall.floors{
        for store in floor.stores{
            for employee in store.employees{
                if highest_paid.len() == 0 || employee.salary == highest_paid[0].salary{
                    highest_paid.push(employee);
                }
                else if employee.salary > highest_paid[0].salary{
                    highest_paid[0] = employee;
                }
            }
        }
    }
    return highest_paid;
}
pub fn nbr_of_employees(mall: Mall) -> usize {
    mall.floors.iter().fold(0, |acc, floor| {
        acc + floor
            .stores
            .iter()
            .fold(0, |acc1, store| acc1 + store.employees.len())
    }) + mall.guards.len()
}

pub fn check_for_securities(mall: &mut Mall, guards: Vec<Guard>) {
    let area = mall.floors.iter().fold(0, |acc, f| acc + f.size_limit);
    let expected_guard_num = area / 200;
    for g in guards {
        if mall.guards.len() < expected_guard_num as usize {
            mall.hire_guard(g)
        } else {
            break;
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    mall.floors.iter_mut().for_each(|f| {
        f.stores.iter_mut().for_each(|s| {
            s.employees.iter_mut().for_each(|e| {
                e.salary = if e.working_hours.1.sub(&e.working_hours.0) > 10 {
                    e.salary + e.salary * 0.1
                } else {
                    e.salary - e.salary * 0.1
                };
            });
        });
    })
}
