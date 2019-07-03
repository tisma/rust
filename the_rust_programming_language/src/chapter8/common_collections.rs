use std::collections::HashMap;

fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v1 = vec![1, 2, 3, 4, 5, 6];

    {
        let vv = vec![1, 2, 3];
    }

    let third: &i32 = &v1[2];
    println!("{}", third);
    let fourth: Option<&i32> = v1.get(3);
    match fourth {
        None => println!("None"),
        Some(x) => println!("{}", x),
    }

    println!("fourth = {}", fourth.unwrap());

    // let does_not_exist = &v1[222];
    let does_not_exist = v1.get(222);
    // println!("{}", does_not_exist.unwrap());
    match does_not_exist {
        None => (),
        Some(x) => println!("{}", x),
    }

    let mut va = vec![1, 2, 3, 4, 5];
    let first = &va[0];
    va.push(42);

    for i in &va {
        println!("{}", i);
    }
    for i in &mut va {
        *i += 100;
    }

    for i in &va {
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(123),
        SpreadSheetCell::Float(43.34),
        SpreadSheetCell::Text(String::from("text field")),
    ];
    for i in &row {
        println!("{:?}", i);
    }

    {
        // let mut s = String::new();
        let data = "initial contents";
        let s = data.to_string();
        println!("{}", s);
        let hello = String::from("Ћирилични фонт");
        println!("{}", hello);

        let mut s1 = String::from("foo");
        let s2 = " bar";
        s1.push_str(s2);
        println!("s1 is {}, s2 is {}", s1, s2);

        let mut ss = String::from("lo");
        ss.push('l');
        println!("{}", ss);
    }
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;
        println!("s3 is {}", s3);
    }
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{}", s);
    }
    {
        let len = String::from("hello").len();
        let len2 = String::from("Ћао људи").len();
        println!("{}, {}", len, len2);
        let t = "test";
        let idx1 = &t[0..2];
        println!("{}", idx1);

        for c in "ово је ћирилични литерал".chars() {
            println!("{}", c);
        }
        for cb in "ово је ћирилични литерал".bytes() {
            println!("{}", cb);
        }
    }
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("red"), 54);

        println!("score of blue = {}", scores.get("blue").unwrap());
        let teams = vec![String::from("x1"), String::from("x2")];
        let initial_scores = vec![10, 450];
        let team_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

        for (key, value) in &team_scores {
            println!("{}: {}", key, value);
        }

        scores.insert("blue".to_string(), 254);
        println!("{:?}", scores);
    }
    {
        let mut scores = HashMap::new();
        scores.insert("blue".to_string(), 10);
        scores.entry("yellow".to_string()).or_insert(40);
        scores.entry("blue".to_string()).or_insert(40);
        println!("{:?}", scores);

        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }
}