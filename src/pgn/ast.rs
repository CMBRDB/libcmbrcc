use super::{tokens::PgnToken, tree::Tree};
use std::{collections::VecDeque, error::Error};

pub fn pgn_tokens_to_ast(
    tokens: &mut VecDeque<PgnToken>,
) -> Result<Tree<PgnToken>, Box<dyn Error>> {
    let mut tree = Tree::new();

    while tokens.len() != 0 {
        next_token(tokens, &mut tree, u16::MAX)
    }

    Ok(tree)
}

fn next_token(
    tokens: &mut VecDeque<PgnToken>,
    tree: &mut Tree<PgnToken>,
    mut half_move_number: u16,
) {
    use PgnToken::*;

    let token = tokens.pop_front().unwrap();

    match token {
        Header(_, _) | Comment(_) | Result(_) | VariationEnd => tree.insert(token),

        HalfMoveNumber(v) => {
            half_move_number = v;
        }

        PgnMove(_) => {
            
        }

        VariationStart => next_token(tokens, tree, half_move_number),

        NGA(_) => {
            unreachable!()
        }

        None => {
            unreachable!()
        }
    }
}
