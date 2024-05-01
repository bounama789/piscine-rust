use std::ops::AddAssign;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

struct Res {
    cals: f64,
    carbs: f64,
    proteins: f64,
    fats: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut res = Res {
        cals: 0.0,
        carbs: 0.0,
        proteins: 0.0,
        fats: 0.0,
    };


    
    foods.iter().for_each(|food| {
        let cal: f64 = food.calories[1]
            .replace("kcal", "")
            .parse()
            .expect("parse error");
        res.cals.add_assign(cal * food.nbr_of_portions);
        res.fats.add_assign(food.fats * food.nbr_of_portions);
        res.carbs.add_assign(food.carbs * food.nbr_of_portions);
        res.proteins
            .add_assign(food.proteins * food.nbr_of_portions)
    });

    res.cals = (res.cals * 100.0).round() / 100.0;
    res.carbs = (res.carbs * 100.0).round() / 100.0;
    res.proteins = (res.proteins * 100.0).round() / 100.0;
    res.fats = (res.fats * 100.0).round() / 100.0;



    json::object!{
        "cals": res.cals,
        "carbs": res.carbs,
        "proteins": res.proteins,
        "fats": res.fats,
    }
}
