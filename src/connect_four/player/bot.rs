use std::{collections::HashMap, fmt::Display};

use crate::connect_four::{board::Board, player::Player, square::Square};

use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng};

pub struct Bot {
    color: Square,
}

impl Player for Bot {
    type MoveData = usize;

    fn is_human(&self) -> bool {
        false
    }

    fn get_move(&mut self, current_board: &Board) -> Result<Self::MoveData> {
        if current_board.is_empty() {
            return Ok(self.get_random_move(current_board));
        }

        let mut tree = GameTree::new();

        let opponent_color = self.color.flip_into();
        tree.init_tree(current_board, opponent_color);
        tree.build_tree(current_board, opponent_color, 0);

        println!("{}", (0..20).map(|_| "-").collect::<String>());
        println!("{tree}");
        println!("{}", (0..20).map(|_| "-").collect::<String>());

        // SAFETY: A user won't be asked to pick a move if there is a draw.
        let moves = current_board.list_valid_moves();
        Ok(moves[0])
    }
}

impl Bot {
    pub fn new(color: Square) -> Self {
        Self { color }
    }
    fn get_random_move(&self, current_board: &Board) -> usize {
        let mut rng = thread_rng();
        let moves = current_board.list_valid_moves();

        // SAFETY: A user won't be asked to pick a move if there is a draw.
        *moves.choose(&mut rng).unwrap()
    }
}

#[derive(Debug, Clone)]
struct GameNode {
    board: Board,
    color: Square,
    depth: usize,
    player_move: usize,
    evaluation: isize,
    children: Vec<GameNode>,
}

impl Display for GameNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Move: {},", self.player_move)?;
        write!(f, "Color: {},", self.color)?;
        write!(f, "Depth: {},", self.depth)?;
        write!(f, "Evaluation: {},", self.evaluation)?;
        write!(f, "Children: {}", self.children.len())?;
        writeln!(f)?;
        Ok(())
    }
}

struct GameTree {
    tree: HashMap<Board, GameNode>,
}

impl Display for GameTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (board, node) in &self.tree {
            writeln!(f, "Board:\n{board}")?;
            writeln!(f, "{node}")?;
        }
        Ok(())
    }
}

impl GameTree {
    // Should be an odd number
    const MAX_DEPTH: usize = 1;
    fn new() -> Self {
        Self {
            tree: HashMap::new(),
        }
    }

    fn init_tree(&mut self, starting_board: &Board, starting_color: Square) {
        self.tree.insert(
            starting_board.clone(),
            GameNode {
                board: starting_board.clone(),
                color: starting_color,
                depth: 0,
                player_move: 0, // null here
                evaluation: starting_board.eval(),
                children: vec![],
            },
        );
    }

    fn build_tree(&mut self, board: &Board, color: Square, tree_depth: usize) {
        if tree_depth >= Self::MAX_DEPTH {
            return;
        }

        self.expand_node(board, color, tree_depth);

        let node_children = {
            let node = self.tree.get_mut(board).unwrap();
            std::mem::take(&mut node.children)
        };

        for child in &node_children {
            self.build_tree(&child.board, child.color, tree_depth + 1);
        }

        let node = self.tree.get_mut(board).unwrap();
        node.children = node_children;
    }

    fn expand_node(&mut self, board: &Board, color: Square, current_tree_depth: usize) {
        let moves = board.list_valid_moves();
        let mut children = Vec::new();

        for m in moves {
            // BUG: transpositions can be messed up with color
            let mut board = board.clone();
            let color = color.flip_into();
            let tree_depth = current_tree_depth + 1;

            board.apply_move(m, color).unwrap();

            let node = GameNode {
                board: board.clone(),
                color,
                depth: tree_depth,
                player_move: m,
                evaluation: board.eval(),
                children: vec![],
            };

            self.tree.insert(board, node.clone());

            children.push(node);
        }

        let parent_node = self.tree.get_mut(board).unwrap();
        parent_node.children = children;
    }
}
