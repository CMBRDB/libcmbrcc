use litemap::LiteMap;
use pgn_lexer::parser::Token;
use std::collections::VecDeque;

use crate::utils::nth_prime_number;

pub type VariationPointerT = u32;

/// An enumeration representing different types of PGN tokens.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum PgnToken<'a> {
    /// Represents a token specific to the game, such as a move, header, or result.
    Token(Token<'a>),
    /// Represents a pointer to a variation.
    VariationPointer(VariationPointerT),
    /// Represents no token. This is the default variant.
    #[default]
    None,
}

/// A structure representing a PGN variation, which is a series of PGN tokens.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PgnVariation<'a>(pub Vec<PgnToken<'a>>);

/// A structure representing a PGN game.
///
/// This consists of:
/// - A vector of tokens specific to the game, such as headers and results.
/// - A map of variations, indexed by their respective pointers.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PgnGame<'a> {
    pub global_tokens: Vec<Token<'a>>,
    pub variations: LiteMap<VariationPointerT, PgnVariation<'a>>,
}

/// Builds an ast (represented as `a Vec<PgnGame>`) from the inputted Token list
pub fn build_pgn_ast<'a>(tokens: &mut VecDeque<Token<'a>>) -> Vec<PgnGame<'a>> {
    let mut tree: Vec<PgnGame<'a>> = Vec::new();
    let mut game_number = 0;
    let mut amount_of_encountered_variations = 1;

    tree.push(PgnGame::default());
    // SAFE: Safe
    unsafe {
        let value = tree.get_unchecked_mut(0);

        value.global_tokens = Vec::new();
        value.variations.insert(0, PgnVariation::default());
    }

    while !tokens.is_empty() {
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
            .variations
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
    variation_depth: VariationPointerT,
    amount_of_encountered_variations: &mut u16,
) {
    // NOTE: I don't know if this is slow. (Like this whole approach) I'm just gonna pretend it isn't until it causes problems
    // NOTE: This should be safe when called correctly. this function is only used locally, so it
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
            .global_tokens
            .push(token),
        Token::NullMove(_) => {}
        Token::EscapeComment(_) => { /* NOTE: IDK what to do with this */ }
        Token::Result(_) => {
            tree.get_mut(*game_number as usize)
                .unwrap()
                .global_tokens
                .push(token);

            *game_number += 1;
            *amount_of_encountered_variations = 1;

            tree.push(PgnGame::default());
            // SAFE: safe
            unsafe {
                let value = &mut tree.get_unchecked_mut(*game_number as usize);

                value.global_tokens = Vec::new();
                value
                    .variations
                    .insert(variation_depth, PgnVariation::default());
            }
        }
        Token::StartVariation(_) => {
            let new_variation_depth = nth_prime_number::<u32>(*amount_of_encountered_variations as u32) * (variation_depth + 1);

            *amount_of_encountered_variations += 1;

            push_token!(
                tree,
                *game_number,
                &variation_depth,
                PgnToken::VariationPointer(new_variation_depth)
            );

            // SAFE: Safe
            unsafe {
                let value = &mut tree.get_unchecked_mut(*game_number as usize);
                value
                    .variations
                    .insert(new_variation_depth, PgnVariation::default());
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

    if !tokens.is_empty() {
        next_token(
            tokens,
            tree,
            game_number,
            variation_depth,
            amount_of_encountered_variations,
        );
    }
}
