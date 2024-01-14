/*
The general definition of the energy level verification function is the following:

    f(r₀, r₁, r₇) = f(n, x, y) = fₙ(x, y) =
        f₀(x, y) = x + 1
        fₙ(0, y) = fₙ₋₁(y, y)
        fₙ(x, y) = fₙ₋₁(fₙ(x - 1, y), y)

We can expand this definition for the first 3 values of n:

    f₁(x, y) =
        f₁(0, y) = f₀(y, y) = y + 1
        f₁(1, y) = f₀(f₁(0, y), y) = f₀(y + 1, y) = y + 2
        f₁(2, y) = f₀(f₁(1, y), y) = f₀(y + 2, y) = y + 3
        ... (by induction)
        f₁(x, y) = x + y + 1

    f₂(x, y) =
        f₂(0, y) = f₁(y, y) = 2 * y + 1
        f₂(1, y) = f₁(f₂(0, y), y) = f₁(2 * y + 1, y) = 3 * y + 2
        f₂(2, y) = f₁(f₂(1, y), y) = f₁(3 * y + 2, y) = 4 * y + 3
        ... (by induction)
        f₂(x, y) = (x + 2) * y + x + 1 = x * (y + 1) + 2 * y + 1

    f₃(x, y) =
        f₃(0, y) = f₂(y, y) = y² + 3 * y + 1
        f₃(x, y) = f₂(f₃(x - 1, y), y) = f₃(x - 1, y) * (y + 1) * 2 * y + 1
                                         \..cached../

This function can be implemented using dynamic programming, which allows to quickly evaluate it for a given y.
We can then iterate for each value of y to find the one that satisfies the verification condition
of f₄(1, y) == 6 using only f₃:

    f₄(1, y) = f₃(f₄(0, y), y) = f₃(f₃(y, y), y)
*/

const MAX_SIZE: u32 = 32768;

fn check_energy_level(y: u32) -> bool {
    let mut f3 = [0; MAX_SIZE as usize];

    f3[0] = (y * y + 3 * y + 1) % MAX_SIZE;

    for i in 1 .. f3.len() {
        f3[i] = (f3[i - 1] * (y + 1) + 2 * y + 1) % MAX_SIZE;
    }

    f3[f3[y as usize] as usize] == 6
}

fn main() {
    for r7 in 0 .. MAX_SIZE {
        if check_energy_level(r7) {
            println!("r7 = {}", r7);
            break;
        }
    }
}
