fn f(memory: &mut Vec<Vec<u64>>, x: usize, y: usize) -> u64 {
    if memory[x][y] != 0 {
        return memory[x][y];
    }

    if x == 0 && y == 0 {
        return 0;
    }
    if x == 0 || y == 0 {
        memory[x][y] = 1;
        return memory[x][y];
    }
    memory[x][y] = f(&mut *memory, x - 1, y) + f(&mut *memory, x, y - 1);
    memory[x][y]
}

pub fn start() {
    println!("0015# lattice paths");
    let grid_size = 20;
    let mut memory = vec![vec![0_u64; grid_size + 1]; grid_size + 1];

    let result = f(&mut memory, grid_size, grid_size);

    println!("Result: {:?}", result)
}
