pub mod mall;
use std::ops::Sub;

pub use floor::store::employee::*;
pub use mall::floor::*;
pub use mall::*;
use mall::{floor::store::employee, guard::Guard};

pub fn biggest_store(mall: Mall) -> store::Store {
    let u = mall
        .floors
        .iter()
        .max_by(|x, y| {
            let a = x
                .stores
                .iter()
                .max_by(|x1, y1| x1.square_meters.cmp(&y1.square_meters))
                .unwrap();
            let b = y
                .stores
                .iter()
                .max_by(|x1, y1| x1.square_meters.cmp(&y1.square_meters))
                .unwrap();

            a.square_meters.cmp(&b.square_meters)
        })
        .unwrap();

    u.stores
        .iter()
        .max_by_key(|store| store.square_meters)
        .unwrap()
        .clone()
}

pub fn highest_paid_employee(mall: Mall) -> Employee{
    let mut h_e: Option<Employee> = None;
    mall.floors.iter().for_each(|f| {
        f.stores.iter().for_each(|s| {
            let m = s
                .employees
                .iter()
                .max_by(|e1, e2| e1.salary.total_cmp(&e2.salary))
                .unwrap();

                if h_e.is_none() {
                    h_e = Some(m.clone());
                } else  {
                    let e = h_e.clone().unwrap();
                    if e.salary < m.salary {
                        h_e = Some(m.clone());
                    }
                }
        });
       
    });

    h_e.unwrap()
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
