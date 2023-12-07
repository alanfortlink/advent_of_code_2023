fn q1(l: &String) -> i32 {
    let mut items = l.split(":");
    let game_id = items.next().unwrap();
    let id = game_id.split(" ").skip(1).next().unwrap().parse::<i32>();

    let reveals = items.next().unwrap().trim().split(";");
    for reveal in reveals.into_iter() {
        for ball in reveal.trim().split(",").into_iter() {
            let mut values = ball.trim().split(" ");

            let amount = values.next().unwrap().parse::<i32>().unwrap();
            let color = values.next().unwrap();

            let limit = match color {
                "red" => 12,
                "green" => 13,
                "blue" => 14,
                _ => 0,
            };

            if amount > limit {
                return 0;
            }
        }
    }

    id.unwrap()
}
