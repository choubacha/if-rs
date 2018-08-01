macro_rules! my_if {
    (if ($cond:expr) $true:block else $false:block) => {
        match $cond {
            true => $true,
            false => $false,
        }
    };

    (if ($cond:expr) $true:block) => {
        match $cond {
            true => Some($true),
            false => None,
        }
    };
}

fn main() {
    my_if!(
        if (true) { println!("true!") }
    );

    my_if!(
        if (false) {
            println!("true!")
        } else {
            println!("false!")
        }
    );

    let result = my_if!(
        if (true == true) {
            println!("true!");
            "yay"

        } else {
            println!("false!");
            "nope"
        }
    );
    println!("{:?}", result);
}
