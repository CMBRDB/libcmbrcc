// use super::tree::Tree;
use litemap::LiteMap;
use pgn_lexer::parser::Token;
use std::collections::VecDeque;

// TODO(#16): Implement tests for ast generating
// TODO(#17): Oh shit, currently this program uses 12x the memory of the input file :sob: maybe reduce that ??

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum PgnToken<'a> {
    Token(Token<'a>),
    VariationPointer(u16),
    #[default]
    None,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PgnVariation<'a>(pub Vec<PgnToken<'a>>);

#[derive(Debug, Clone, Default, PartialEq, Eq)]
// (`Tokens specific to the game, such as headers, results, etc.`, A map of variations)
pub struct PgnGame<'a>(pub (Vec<Token<'a>>, LiteMap<u16, PgnVariation<'a>>));

pub fn build_pgn_ast<'a>(tokens: &mut VecDeque<Token<'a>>) -> Vec<PgnGame<'a>> {
    let mut tree: Vec<PgnGame<'a>> = Vec::new();
    let mut game_number = 0;
    let mut amount_of_encountered_variations = 1;

    tree.push(PgnGame::default());
    // SAFE: Safe
    unsafe {
        let value = &mut tree.get_unchecked_mut(0).0;

        value.0 = Vec::new();
        value.1.insert(0, PgnVariation::default());
    }

    while tokens.len() != 0 {
        next_token(
            tokens,
            &mut tree,
            &mut game_number,
            0,
            &mut amount_of_encountered_variations,
        );
    }

    tree.pop();

    tree
}

macro_rules! push_token {
    ($tree:expr, $game_number:expr, $variation_depth:expr, $token:expr) => {
        $tree
            .get_mut($game_number as usize)
            .unwrap()
            .0
             .1
            .get_mut($variation_depth)
            .unwrap()
            .0
            .push($token)
    };
}

fn next_token<'a>(
    tokens: &mut VecDeque<Token<'a>>,
    tree: &mut Vec<PgnGame<'a>>,
    game_number: &mut u32,
    variation_depth: u16,
    amount_of_encountered_variations: &mut u16,
) {
    // NOTE: I don't know if this is slow. (Like this whole approach) I'm just gonna pretend it isn't until it causes problems
    // NOTE: This should be safe when called correctly. this function is only used locally, so it
    // should be
    // SAFE: Should be safe
    let token = unsafe { tokens.pop_front().unwrap_unchecked() };

    match token {
        Token::Move(_)
        | Token::Commentary(_)
        | Token::NAG(_)
        | Token::MoveAnnotation(_)
        | Token::MoveNumber(_, _) => {
            push_token!(tree, *game_number, &variation_depth, PgnToken::Token(token))
        }
        Token::TagSymbol(_) | Token::TagString(_) => tree
            .get_mut(*game_number as usize)
            .unwrap()
            .0
             .0
            .push(token),
        Token::NullMove(_) => {}
        Token::EscapeComment(_) => { /* NOTE: IDK what to do with this */ }
        Token::Result(_) => {
            tree.get_mut(*game_number as usize)
                .unwrap()
                .0
                 .0
                .push(token);

            *game_number += 1;
            *amount_of_encountered_variations = 1;

            tree.push(PgnGame::default());
            // SAFE: safe
            unsafe {
                let value = &mut tree.get_unchecked_mut(*game_number as usize).0;

                value.0 = Vec::new();
                value.1.insert(variation_depth, PgnVariation::default());
            }
        }
        Token::StartVariation(_) => {
            let new_variation_depth = *amount_of_encountered_variations * (variation_depth + 1);

            *amount_of_encountered_variations += 1;

            push_token!(
                tree,
                *game_number,
                &variation_depth,
                PgnToken::VariationPointer(new_variation_depth)
            );

            // SAFE: Safe
            unsafe {
                let value = &mut tree.get_unchecked_mut(*game_number as usize).0;
                value.1.insert(new_variation_depth, PgnVariation::default());
            }

            next_token(
                tokens,
                tree,
                game_number,
                new_variation_depth,
                amount_of_encountered_variations,
            );
        }
        Token::EndVariation(_) => {
            return;
        }
    }

    if tokens.len() != 0 {
        next_token(
            tokens,
            tree,
            game_number,
            variation_depth,
            amount_of_encountered_variations,
        );
    }
}
