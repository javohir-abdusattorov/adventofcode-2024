
pub fn part_1(input: String) -> i128 {
    #[derive(Debug)]
    enum Operators { Add, Multiply }

    impl Operators {
        fn eval(&self, a: i128, b: i128) -> i128 {
            match self {
                Operators::Add => a + b,
                Operators::Multiply => a * b,
            }
        }
    }

    fn cmb(prev: i128, nums: &Vec<i128>, i: usize, target: i128) -> bool {
        if i >= nums.len() {
            return prev == target;
        }

        [Operators::Add, Operators::Multiply]
            .into_iter()
            .any(|operator| {
                let ev = operator.eval(prev, nums[i]);
                cmb(ev, nums, i + 1, target)
            })
    }

    input
        .split("\n")
        .map(|line| {
            let mut parts = line.split(": ");
            let eq = parts.next().unwrap().parse::<i128>().unwrap();
            let nums = parts.next().unwrap().split_whitespace().filter_map(|n| n.parse::<i128>().ok()).collect::<Vec<i128>>();

            (eq, nums)
        })
        .filter(|(target, nums)| cmb(nums[0], nums, 1, *target))
        .fold(0, |acc, (target, _)| acc + target)
}

pub fn part_2(input: String) -> i128 {
    #[derive(Debug)]
    enum Operators { Add, Multiply, Concatenate }

    impl Operators {
        fn eval(&self, a: i128, b: i128) -> i128 {
            match self {
                Operators::Add => a + b,
                Operators::Multiply => a * b,
                Operators::Concatenate => format!("{}{}", a.to_string(), b.to_string()).parse::<i128>().unwrap()
            }
        }
    }

    fn cmb(prev: i128, nums: &Vec<i128>, i: usize, target: i128) -> bool {
        if i >= nums.len() {
            return prev == target;
        }

        [Operators::Add, Operators::Multiply, Operators::Concatenate]
            .into_iter()
            .any(|operator| {
                let ev = operator.eval(prev, nums[i]);
                cmb(ev, nums, i + 1, target)
            })
    }

    input
        .split("\n")
        .map(|line| {
            let mut parts = line.split(": ");
            let eq = parts.next().unwrap().parse::<i128>().unwrap();
            let nums = parts.next().unwrap().split_whitespace().filter_map(|n| n.parse::<i128>().ok()).collect::<Vec<i128>>();

            (eq, nums)
        })
        .filter(|(target, nums)| cmb(nums[0], nums, 1, *target))
        .fold(0, |acc, (target, _)| acc + target)
}