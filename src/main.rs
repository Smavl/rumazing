use std::{fmt};
use rand::{Rng};
use rand::prelude::SliceRandom;

#[derive(Clone, Copy, PartialEq)]
#[derive(Debug)]
enum CellType {
    Wall,
    Tile,
    Door,
}
#[derive(Clone, PartialEq, Debug)]
struct Cell {
    cell_type: CellType,
    visited: bool,
}

impl Cell {
    fn new(cell_type: CellType)-> Cell{
       Cell{
           cell_type,
           visited: false,
       }
    }

    fn get_celltype(&self) -> &CellType{
        &self.cell_type
    }
    fn is_visited(&self)-> bool{
        self.visited
    }
    fn mark_visited(&mut self){
        match self.cell_type {
            CellType::Wall => {
                self.convert_to(CellType::Tile);
                self.visited = true
            },
            _ => self.visited = true,
        }
    }
    fn convert_to(&mut self, ct: CellType) {
       self.cell_type = ct;
    }
    fn to_string(&self) -> &'static str {
        match (self.cell_type, self.visited) {
            (CellType::Wall, _ )=> "█",
            (CellType::Tile, false)=> "_",
            (CellType::Tile, true )=> "x",
            (CellType::Door, _ )=> "^",
        }
    }
}

// Maze struct
struct Maze {
    height: usize,
    width: usize,
    cells: Vec<Vec<Cell>>,
}
struct GeneratedMaze {
    height: usize,
    width: usize,
    cells: Vec<Vec<Cell>>,
}

impl Maze {
    fn grid_fill(width: usize, height: usize) -> Maze {
        let mut cells = vec![vec![Cell::new(CellType::Wall); width]; height];
        for i in 0..height{
            for j in 0..width{
                if (i % 2 != 1) && (j % 2 != 1) { 
                //if (i % 2 == 1) && (j % 2 == 1) {
                    cells[i][j].convert_to(CellType::Tile)
                }
            }
        }

        Maze {height, width, cells}
    }

    pub fn new(width: usize, height: usize, mut cs:Box<dyn CarvingStrategy>) -> Maze {
        let mut m= Self::grid_fill(width,height);
        cs.carve(&mut m);
        //m.add_doors();
        m
    }

    fn get_cell_ref(&mut self, width: usize, height: usize) -> &mut Cell {
        &mut self.cells[height][width]
    }

    fn get_dim(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn remove_wall_between_cells(&mut self, cell : (usize, usize), ncell : (usize, usize)) {
        let (cx, cy) = cell;
        let (nx, ny) = ncell;

        // The wall is located between the current cell and the neighbor cell
        let wall_x = (cx + nx) / 2;
        let wall_y = (cy + ny) / 2;
        self.get_cell_ref(wall_x, wall_y).mark_visited();
    }

    fn remove_wall(&mut self, current_cell: &mut Cell) {
        match current_cell.get_celltype() {
            CellType::Wall => {
               current_cell.convert_to(CellType::Tile);
            }
            CellType::Tile => {
                panic!("\"Wall\" is actually a Tile")
            }
            CellType::Door => {
                panic!("\"Wall\" is actually a Door")
            }
        }
    }


    fn carve_maze(&mut self, mut alg: Box<dyn CarvingStrategy>) {
        let maze = self;
        alg.carve(maze);
    }

    // unused
    fn add_doors(&mut self) {
        let (_w, _h) = (self.width-1, self.height-1);
        // create openings
        //self.modify_cell(0, 1, Door); // row
        //self.modify_cell(h, w-1, Door); // cols

    }
    fn add_random_doors(&mut self) {
        let (w, h) = (self.width-1, self.height-1);
        let mut rng = rand::thread_rng();
        let _rr =rng.gen_range(1..w);
        let _rc =rng.gen_range(1..h);
        // create openings
        //self.modify_cell(0, rr, AlgorithmCell::Door); // row
        //self.modify_cell(rc, 0, AlgorithmCell::Door); // cols

    }
}

trait CarvingStrategy {
    fn get_neighbors(&self,x:usize, y:usize, maze: &mut Maze) -> Vec<(usize, usize)>;
    fn carve(&mut self, maze: &mut Maze);

}

struct DFSCarvingAlgorithm;
impl CarvingStrategy for DFSCarvingAlgorithm{
    fn get_neighbors(&self, x: usize, y: usize, maze: &mut Maze) -> Vec<(usize,usize)>{
        let dir: Vec<(isize, isize)> = vec![(0,-2), (0,2), (2,0), (-2,0)];
        let (w,h) = maze.get_dim();
        let mut neighbors: Vec<(usize,usize)> = Vec::new();
        for (dx, dy) in dir {
            let x_new = (x as isize + dx) as usize;
            let y_new = (y as isize + dy) as usize;
            if (x_new < w ) && (y_new < h ) {
                //println!("{:?}, {:?}, {:?}", x, y, maze.get_dim());
                neighbors.push((x_new, y_new))
            }
        }
        neighbors
    }

    // stack approach
    fn carve(self: &mut DFSCarvingAlgorithm, maze: &mut Maze) {
        // create stack
        let mut stack = Vec::new();
        // push starting point (1,1)?
        maze.get_cell_ref(0, 0).mark_visited();
        stack.push((0,0));

        // while stack is not empty
        while let Some((x,y)) = stack.last().cloned() {
            //println!("current_cell: ({x},{y}) stack: {stack:?}");

            // get neighbors (tuples)
            let mut neighbors = self.get_neighbors(x, y, maze);
            let mut rng = rand::thread_rng();
            neighbors.shuffle(&mut rng);
            // print neighbors
            //println!("neighbors: {:?}",neighbors);
            
           let mut found_unvisited = false; 
            
            for (nx,ny) in neighbors {
                if !maze.get_cell_ref(nx,ny).is_visited(){
                    // remove wall between new and old cell
                    maze.remove_wall_between_cells((x, y), (nx , ny ));
                    // mark neighbor as visited
                    maze.get_cell_ref(nx,ny).mark_visited();
                    stack.push((nx,ny));
                    found_unvisited = true;
                    break;
                }
            }
            
            if !found_unvisited {
                stack.pop();
            }
        }

    }

}
struct HomemadeCarvingAlgorithm;
impl CarvingStrategy for HomemadeCarvingAlgorithm {
    fn get_neighbors(&self,_x: usize, _y: usize, _maze: &mut Maze) -> Vec<(usize, usize)> {
        todo!()
    }

    fn carve(self: &mut HomemadeCarvingAlgorithm, _maze: &mut Maze) {
        todo!()
    }
}


impl fmt::Display for Maze{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let width = self.cells[0].len();
        // Print column numbers
        write!(f, "  ")?;
        for j in 0..width {
            write!(f, "r{} ", j % 10)?;
        }
        writeln!(f)?;

        for (r, row) in self.cells.iter().enumerate() {
            write!(f, "c{} ", r % 10)?;
            for cell in row {
                let symbol = cell.to_string();
                write!(f, " {} ", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    // print maze
    let m = Maze::new(41, 29, Box::new(DFSCarvingAlgorithm));
    println!("{}", m);


    //stdout.flush();
}
