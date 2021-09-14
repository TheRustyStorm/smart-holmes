pub struct TransitionMatrix{
    matrix: Vec<usize>,
    rows: usize,
    columns: usize,
}

impl TransitionMatrix{
    pub fn new(size: usize) -> TransitionMatrix{
        let matrix = vec![0;size*size];
        TransitionMatrix{ matrix: matrix, rows: size, columns: size}
    }

    pub fn increase(&mut self, from: usize, to: usize){
        self.matrix[from*self.rows + to] += 1;
    }

    pub fn print(&self) {
        for row in 0..self.rows{
            for column in 0..self.columns{
                print!("{}\t", self.matrix[row*self.columns + column]);
            }
            println!("");
        }
    }

    pub fn get_at(&self, row: usize, column: usize) -> usize{
        self.matrix[row*self.columns + column]
    }

    pub fn sum_at_row(&self, row: usize) -> usize{
        let mut sum = 0;
        for i in self.columns*row..self.columns*(row+1){
            sum += self.matrix[i];
        }
        sum
    }
}