use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static MONTH: Lazy<HashMap<u32, &str>> = Lazy::new(|| {HashMap::from({
    [
        (1,"yanvarya"),
        (2,"fevralya"),
        (3,"marta"),
        (4,"aprelya"),
        (5,"maya"),
        (6, "iyunya"),
        (7, "iyulya"),
        (8, "avgust"),
        (9,  "sentyabrya"),
        (10, "oktyabrya"),
        (11, "noyabrya"),
        (12, "dekabrya")
    ]
})});

pub static ZODIAC: Lazy<HashMap<&str, &str>> = Lazy::new(|| {HashMap::from({
    [
        ("овен", "oven"),
        ("телец","telec"),
        ("близнецы","bliznecy"),
        ("дева","deva"),
        ("весы","vesy"),
        ("скорпион", "skorpion"),
        ("стрелец", "strelec"),
        ("козерог", "kozerog"),
        ("водолей", "vodolej"),
        ("рыбы","ryby"),
        ("лев", "lev"),
        ("рак", "rak"),
    ]
})});