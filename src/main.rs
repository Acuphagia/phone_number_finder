use rand::Rng;
use std::time::Instant;

fn main()
{

    // Starts the timer
    let start = Instant
        ::now();

    // The AI Structure
    struct AI
    {
        x: i32,
        y: i32,
        skibidis: i32,
    }

    impl AI
    {
        // Doing boring math to see how far away it is from the target
        fn distance_from_target(&self, target_x: i32, target_y: i32) -> i32
        {
            (self
                .x
                - target_x)
                .abs()
                + (self
                    .y
                    - target_y)
                    .abs()
        }

        // Check if the AI got closer or further from target, & then tell it to move based on that data
        fn move_toward_target(&mut self, target_x: i32, target_y: i32)
        {
            let current_distance = self
                .distance_from_target(
                    target_x,
                    target_y
                );

            // Minimize the distance as much as possible
            if self
                .x
                < target_x
            {
                self
                    .x
                    += 1;
            } else if self
                .x
                > target_x
            {
                self
                    .x
                    -= 1;
            }

            if self
                .y
                < target_y
            {
                self
                    .y
                    += 1;
            } else if self
                .y
                > target_y
            {
                self
                    .y
                    -= 1;
            }

            let new_distance = self
                .distance_from_target(
                    target_x,
                    target_y
                );

            // Reward system logic based on distance change
            if new_distance
                < current_distance
            {
                self
                    .skibidis
                    += 10; // Got closer, +10 skibidis!!
            } else if new_distance
                > current_distance
            {
                self
                    .skibidis
                    -= 5; // Got further, decrease skibidis
            }
        }
    }

    // Grid dimensions (simulate a LOT of phone numbers)
    let grid_width = 10000;
    let grid_height = 10000;

    // Generate a random grid of phone numbers
    let mut rng = rand
        ::thread_rng();
    let mut phone_numbers = Vec
        ::new();

    for _ in 0..grid_width * grid_height
    {
        let random_number = format!(
            "{}-{}-{}",
            rng
                .gen_range(100..999),
            rng
                .gen_range(100..999),
            rng
                .gen_range(1000..9999)
        );
        phone_numbers
            .push(
            random_number
        );
    }

    // Define the target phone number and its grid position
    let target_x = rng
        .gen_range(0..grid_width);
    let target_y = rng
        .gen_range(0..grid_height);
    // Replace this phone number with your phone number
    let target_phone_number = "111-111-1111";
    phone_numbers[(
        target_y
            * grid_width
        + target_x
    ) as usize] = target_phone_number
        .to_string();

    println!(
        "Target phone number {} located at position: ({}, {})",
        target_phone_number, target_x, target_y
    );

    // Skibidi usage
    let mut ai = AI
    {
        x: rng
            .gen_range(0..10000),
        y: rng
            .gen_range(0..10000),
        skibidis: 0,
    };

    // This code is used, removing it will cause an error message on line 166.
    let mut moves_count: i32 = 0;

    // Simulate movements
    loop
    {
        ai.move_toward_target(
            target_x,
            target_y
        );

        moves_count += 1;

        if ai
            .x
            == target_x && ai
                .y
                == target_y
        {
            println!(
                "The AI has found the target phone number: {} in {:?}!!",
                target_phone_number,
                start
                    .elapsed()
            );
            break;
        }

    }
}
