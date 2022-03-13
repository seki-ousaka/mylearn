use learn::t_1::*;

/**
 * my learning entry
 */
fn main() {
    // // Sum
    // t_1_1::t_1_1_do();

    // // mod abs pow sqrt and the four basic arithmetic operations
    // t_1_2::t_1_2_do();

    // // bit operation
    // t_1_3::t_1_3_do();

    // exec t_1 dir
    t_1();

    // exec t_2 dir
    t_2();
}

fn t_1(){
    // Sum
    t_1_1::t_1_1_do();

    // mod abs pow sqrt and the four basic arithmetic operations
    t_1_2::t_1_2_do();

    // bit operation
    t_1_3::t_1_3_do();

    // constant time
    let n :i32 = t_1_4::t_1_4_do();
    println!("{}", n);
}

fn t_2() {

}