
const NUMBER: u64 = 0x1827364554637281_u64;

fn main() {
    dbg!(&dice_roller::build_array_manually(NUMBER));
    dbg!(&dice_roller::build_array_stdlib(NUMBER));
}
