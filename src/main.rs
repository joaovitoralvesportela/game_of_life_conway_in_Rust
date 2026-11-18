/*

|==============================|
|                              |
| GAME OF LIFE CONWAY'S GAME   |
|                              |
|==============================|

-> Um automato celular criado pelo matematico John Horton Conway em 1970.
-> A cellular automaton created by the mathematician John Horton Conway in 1970.

-> Esse codigo foi criado com auxilio do claude, para que eu possa estudar e aprender mais
-> sobre a linguagem rust e suas ferramentas
-> O código não usa dependências externas
*/

use std::fmt;
use std::{thread, time::Duration};
 
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Dead,
    Alive,
}
 
struct Universe {
    width: usize,
    height: usize,
    /// Grade "achatada": a célula (row, col) fica em `row * width + col`.
    cells: Vec<Cell>,
    /// Segundo buffer: guarda a próxima geração enquanto `cells` ainda está sendo lida.
    next: Vec<Cell>,
}
 
impl Universe {
    fn new(width: usize, height: usize) -> Self {
        assert!(width >= 3 && height >= 3, "a grade precisa ser de pelo menos 3x3");
        Self {
            width,
            height,
            cells: vec![Cell::Dead; width * height],
            next: vec![Cell::Dead; width * height],
        }
    }
 
    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
 
    fn set_alive(&mut self, coords: &[(usize, usize)]) {
        for &(row, col) in coords {
            let idx = self.index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
 
    /// Conta os vizinhos vivos (as 8 células ao redor) com "wrap-around":
    /// a borda de cima conecta com a de baixo, e a da esquerda com a da direita.
    fn live_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        // `height - 1` faz o papel de "-1" depois do `% height`.
        // Assim nunca subtraímos de um usize que pode ser 0 (isso daria overflow).
        for dr in [self.height - 1, 0, 1] {
            for dc in [self.width - 1, 0, 1] {
                if dr == 0 && dc == 0 {
                    continue; // a própria célula não conta
                }
                let r = (row + dr) % self.height;
                let c = (col + dc) % self.width;
                if self.cells[self.index(r, c)] == Cell::Alive {
                    count += 1;
                }
            }
        }
        count
    }
 
    /// Avança uma geração.
    fn tick(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.index(row, col);
                let neighbors = self.live_neighbors(row, col);
 
                // As regras de Conway, quase literalmente:
                self.next[idx] = match (self.cells[idx], neighbors) {
                    (Cell::Alive, 2 | 3) => Cell::Alive, // sobrevive
                    (Cell::Dead, 3) => Cell::Alive,      // nasce
                    _ => Cell::Dead,                     // morre (ou continua morta)
                };
            }
        }
        // A geração nova vira a atual. O swap só troca os ponteiros dos Vecs:
        // sem copiar elementos e sem alocar memória a cada geração.
        std::mem::swap(&mut self.cells, &mut self.next);
    }
}
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        for line in self.cells.chunks(self.width){
            for &cell in line{
                let symbol = if cell == Cell::Alive{"■ "} else {". "};
                write!(f, "{symbol}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main(){
    let mut universe = Universe::new(32,16);

    universe.set_alive(&[(0,1), (1,2), (2,0), (2,1), (2,2)]);

    universe.set_alive(&[(12,4), (12,5), (12,6)]);

    for generation in 0u32..{
        print!("\x1B[2J\x1B[H");
        println!("geração {generation} CTRL+C para sair\n");
        println!("{universe}");
        universe.tick();
        thread::sleep(Duration::from_millis(120));
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    fn universe(width: usize, height: usize, coords: &[(usize, usize)]) -> Universe{
        let mut u = Universe::new(width, height);
        u.set_alive(coords);
        u
    }
    fn alive(u: &Universe) -> Vec<(usize, usize)>{
        (0..u.height)
            .flat_map(|r| (0..u.width).map(move |c| (r, c)))
            .filter(|&(r, c)| u.cells[u.index(r, c)] == Cell::Alive)
            .collect()
    }

    #[test]
    fn celula_isolada_morre(){
        let mut u = universe(5,5,&[(2,2)]);
        u.tick();
        assert!(alive(&u).is_empty());
    }
    #[test]
    fn bloco_nao_muda(){
        let block = [(1, 1), (1, 2), (2, 1), (2, 2)];
        let mut u = universe(6,6,&block);
        u.tick();
        assert_eq!(alive(&u), block);    }
    #[test]
    fn blinker_oscila_com_periodo_2(){
        let horizontal = [(2, 1), (2, 2), (2, 3)];
        let vertical = [(1, 2), (2, 2), (3, 2)];
        let mut u = universe(6,6,&horizontal);
        u.tick();
        assert_eq!(alive(&u), vertical);
        u.tick();
        assert_eq!(alive(&u), horizontal);

    }
    #[test]
    fn glider_anda_uma_casa_na_diagonal_a_cada_4_geracoes() {
        // Nave (spaceship)
        let glider = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
        let mut u = universe(8, 8, &glider);
        for _ in 0..4 {
            u.tick();
        }
        let esperado: Vec<_> = glider.iter().map(|&(r, c)| (r + 1, c + 1)).collect();
        assert_eq!(alive(&u), esperado);
    }
 
    #[test]
    fn bordas_dao_a_volta() {
        // Um blinker na borda esquerda precisa "enxergar" a coluna da direita.
        let mut u = universe(6, 6, &[(2, 5), (2, 0), (2, 1)]);
        u.tick();
        assert_eq!(alive(&u), [(1, 0), (2, 0), (3, 0)]);
    }
}