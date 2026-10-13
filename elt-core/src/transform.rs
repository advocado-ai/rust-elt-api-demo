use crate::error::TransformError;
use crate::types::{
    MatchOutcome, MatchRecord, RawMatch, Round, Set, Surface, TourneyLevel, ImportStatus,
};

// ── Public entry point ────────────────────────────────────────────────────────
pub fn transform(raw: RawMatch) -> Result<MatchRecord, TransformError>{
    //1. {tourney_id}_{match_num}"
    let tourney_id = &raw.tourney_id;
    let match_num = &raw.match_num;
    let id = format!("{tourney_id}_{match_num}");

    /* 
    let surface = match parse_surface(&raw.surface){
        Ok(s) => s,
        Err(e) => 
    }
    */
    let surface = parse_surface(&raw.surface)?;

    let tourneylevel = parse_tourney_level(&raw.tourney_level)?;

    //parse_round()

    //parse_date()

    //parse_score()
    let score = parse_score(&raw.score)?;

    let (sets, match_outcome) = score;
    
    let round = parse_round(&raw.round)?;





    //parse_optional_u32()
    let m_record = MatchRecord{
        id: id,
        tourney_id: tourney_id.to_string(),
        tourney_name: raw.tourney_name.to_string(),
        surface: surface,
        tourney_level: tourneylevel,
        winner_name: raw.winner_name,
        loser_name: raw.loser_name,
        score_raw: raw.score,
        sets: sets,
        outcome: match_outcome,
        round: round,




    };


    //2. for each field in MatchRecord, if field exists in RawMatch, extract.  Note any fields in MatchRecord that don't have corresponding field in RawMatch.  

    //2a. use the parsing helper functions to parse the RawMatch into formats we want in MatchRecord

    //3. If successful, wrap the matchRecord in Result and return else return Err(TransformError)


    
}

// ── Parsers ───────────────────────────────────────────────────────────────────
fn parse_surface(s: &str) -> Result<Surface, TransformError> {
    let surface = match s{
        "Hard" => Surface::Hard,
        "Clay" => Surface::Clay,
        "Grass" => Surface::Grass,
        "Carpet" => Surface::Carpet,
        _ => return Err(TransformError::UnknownSurface(format!("{s} is an invalid surface"))),
    };

    Ok(surface)
    
}

fn parse_tourney_level(s: &str) -> Result<TourneyLevel, TransformError> {
    let tourneylevel = match s{
        "G" => TourneyLevel::Grandslam,
        "M" => TourneyLevel::Masters,
        "A" => TourneyLevel::Tour,
        "F" => TourneyLevel::Finals,
        "O" => TourneyLevel::Olympics,
        "D" => TourneyLevel::DavisCup,
        _ => return Err(TransformError::UnknownTourneyLevel(format!("{s} is an unknown tourney level"))), 
    };

    Ok(tourneylevel)
}

fn parse_round(s: &str) -> Result<Round, TransformError> {
    let round = match s{
        "R128" => Round::R128,
        "R64" => Round::R64,
        "R32" => Round::R32,
        "R16" => Round::R16,
        "QF" => Round::Quarterfinal,
        "SF" => Round::Semifinal,
        "F" => Round::Final,
        "RR" => Round::RoundRobin,
        "BR" => Round::Bronze,
        _ => return Err(TransformError::UnknownRound(format!("{s} is an unknown round in the tournament, ex. R128, R16, Quarterfinal."))),
    };
    
    Ok(round)
}

/// "20240101" → "2024-01-01"
/// Returns TransformError::InvalidDate if the string isn't 8 ASCII digits.
fn parse_date(s: &str) -> Result<String, TransformError> {
    /*
    Why the check matters: you'll build "2024-01-01" by slicing byte offsets (&s[0..4], &s[4..6], &s[6..8]). Slicing a str at a non-character boundary panics. If you validate "length 8 and all ASCII digits" first, every byte offset is a character boundary, so the slices can't panic. Note that s.len() counts bytes, not chars. The two agree only because you've proven the string is all ASCII digits. 
    */

    /*
    for c in s.chars(){
        if !c.is_ascii_digit(){
            return Err(TransformError::InvalidDate(format!("Invalid date parsing error: {s}")))
        }
    }
    */

    if !s.chars().all(|c| c.is_ascii_digit()){
        return Err(TransformError::InvalidDate(format!("Invalid date parsing error: {s}")));
    }

    //build the date
    let year = &s[0..4];
    let month = &s[4..6];
    let day = &s[6..8];

    let date = format!("{year}-{month}-{day}");

    Ok(date)
    
}


//build date ex. "2024-01-01" by slicing byte offsets 


/// Parse score string into (sets, outcome).
/// Handles: 
/// "6-3 7-6(4) 6-4", 3 parts 
/// "W/O", 1
/// "6-4 3-0 RET", 3 
/// "6-3 RET" 2
/// "6-3 2-6 3-1 RET"
/// "6-4 6-7(6) 2-0 RET" 4
/* 
The key insight: every score string is just space-separated tokens, and each token is one of three things:

A set: "6-3"
A set with tiebreak: "7-6(4)"
A keyword: "W/O" or "RET"
So the algorithm is:

Split on whitespace → ["6-4", "6-7(6)", "2-0", "RET"]
1. Check the first token for "W/O" → early return, Walkover, empty sets
2. Check the last token for "RET" → outcome = Retirement, drop that token2. Otherwise → outcome = Completed
Parse remaining tokens as sets
Each token that isn't a keyword follows the same pattern: X-Y or X-Y(Z). You parse that with a single helper (or inline logic) — no need to know how many sets there are.

That covers every variation you listed without enumerating them. The only real branching is: W/O → RET → normal. Three cases, not combinatorial.
*/

pub fn parse_score(s: &str) -> Result<(Vec<Set>, MatchOutcome), TransformError> {
    //make a struct Set for each set and put in Vec<Set>
    //MatchOutcome: Completed, Retirement, Walkover

    let keyword_walkover = "W/O";
    let keyword_retired = "RET";

    let mut sets_vec = Vec::new();

    //default if completed
    let mut match_outcome = MatchOutcome::Completed;


    let mut tokens: Vec<&str> = s.split_whitespace().collect();

    if tokens[0] == keyword_walkover{
        return Ok((vec![], MatchOutcome::Walkover));
    }

    let last_token = tokens[tokens.len()-1];

    if last_token == keyword_retired {
        match_outcome = MatchOutcome::Retirement;
        tokens.truncate(tokens.len() -1);
    }

    
    for t in tokens{
        sets_vec.push(parse_set_token(t)?);
    }
    
    /* 
    //idiomatic Rust, no need to collect errors since we propagate errors
    let sets_vec: Vec<Result<Set, TransformError>> = tokens.iter().map(|t| parse_set_token(t)).collect()::<Result<Vec<Set>, _>>()?; 
    */

    Ok((sets_vec, match_outcome))
    

}

//now parse set scores, count num chars, no-tiebreak would have 3 chars, any tiebreak score would have "()" and 6 chars. Winner's number of games per set always leads, losers comes after the "-". in a set with tiebreak that winner lost like 6-7(3) stil the same, winner games = 6, loser games = 7 and tibreak: Option(3)
fn parse_set_token(s: &str) -> Result<Set, TransformError>{
    let mut winner_games: u8 = 0;
    let mut loser_games:u8 = 0;
    let mut tiebreak: Option<u8> = None;
    
    let char_indices: Vec<char> = s.chars().collect();
    
    //if len is 3 no tiebreak
    if char_indices.len() > 3{
        //check if tiebreak went more than std 7 pts
        if char_indices[5] == ')'{
            tiebreak = Some(char_indices[4].to_digit(10).unwrap() as u8);
        }else{
            //grab 2 digits at idx 4,5
            let mut tiebreak_score_str = "".to_string();
            tiebreak_score_str.push(char_indices[4]);
            tiebreak_score_str.push(char_indices[5]);
            tiebreak = Some(tiebreak_score_str.parse::<u8>().unwrap());
        }        
    }

    winner_games = char_indices[0].to_digit(10).unwrap() as u8;

    loser_games = char_indices[2].to_digit(10).unwrap() as u8;

    let set = Set{
        winner_games, loser_games, tiebreak,
    };

    Ok(set)

}

/*

It's used inside `transform()` for the seven `Option<u32>` fields of `MatchRecord` ([types.rs:150-156](elt-core/src/types.rs#L150-L156)), where it converts the matching `RawMatch` string field:

| `MatchRecord` field | `RawMatch` field it comes from |
|---|---|
| `winner_seed` | `winner_seed` |
| `loser_seed` | `loser_seed` |
| `duration_minutes` | `minutes` |
| `winner_aces` | `w_ace` |
| `loser_aces` | `l_ace` |
| `winner_rank` | `winner_rank` |
| `loser_rank` | `loser_rank` |

Notice that `duration_minutes`, `winner_aces` and `loser_aces` have different names on each side.

You need a helper like this because the raw CSV keeps every field as a `String`, and an empty string means "no value". Unseeded players have `winner_seed: ""`, and some matches have no recorded duration or ace counts. The normalized record needs a real `Option<u32>`, where `None` means "not recorded".

It returns `Option` rather than `Result` on purpose. A blank seed isn't a malformed record, so it shouldn't fail the whole transform the way a bad `surface` does. In `transform()` you call it with no `?`:

```rust
winner_seed: parse_optional_u32(&raw.winner_seed),
```

For the implementation, look at `str::parse::<u32>()`, which returns a `Result`. Then look at how to turn a `Result<T, E>` into an `Option<T>` (`.ok()`). The empty-string case may already be covered, because `"".parse::<u32>()` is an `Err`.
 
*/

/// "" or non-numeric → None; otherwise parse to u32.
/// fn parse_optional_u32(s: &str) -> Result<Option<u32>, TransformError>
// ""        -> Ok(None)       blank, which is normal
// "14"      -> Ok(Some(14))
// "abc"     -> Err(...)       genuinely corrupt, so reject the record

fn parse_optional_u32(s: &str) -> Result<Option<u32>, TransformError> {

    //s is empty ""
    if s.is_empty(){
        return Ok(None)
    }

    let number = match s.parse::<u32>(){
        Ok(num) => num,
        Err(e) => return Err(TransformError::InvalidNumber(format!("Error parsing option u32: {s}, {e}"))),
    };

    Ok(Some(number))
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    // -- parse_surface --------------------------------------------------------

    #[test]
    fn surface_hard() {
        assert_eq!(parse_surface("Hard").unwrap(), Surface::Hard);
    }

    #[test]
    fn surface_unknown_errors() {
        assert!(parse_surface("Astrotruf").is_err());
    }

    // -- parse_round ----------------------------------------------------------

    #[test]
    fn round_qf() {
        assert_eq!(parse_round("QF").unwrap(), Round::Quarterfinal);
    }

    #[test]
    fn round_rr() {
        assert_eq!(parse_round("RR").unwrap(), Round::RoundRobin);
    }

    // -- parse_date -----------------------------------------------------------
    #[test]
    fn date_valid() {
        assert_eq!(parse_date("20240101").unwrap(), "2024-01-01");
    }

    #[test]
    fn date_invalid_errors() {
        assert!(parse_date("Jan 1 2024").is_err());
    }

    // -- parse_score ----------------------------------------------------------
    #[test]
    fn score_straight_sets() {
        let (sets, outcome) = parse_score("6-3 6-4").unwrap();
        assert_eq!(sets.len(), 2);
        assert_eq!(outcome, MatchOutcome::Completed);
        assert_eq!(sets[0].winner_games, 6);
        assert_eq!(sets[0].loser_games, 3);
        assert_eq!(sets[0].tiebreak, None);
    }

    #[test]
    fn score_with_tiebreak() {
        let (sets,outcome) = parse_score("7-6(4) 6-3").unwrap();
        assert_eq!(outcome, MatchOutcome::Completed);
        assert_eq!(sets[0].tiebreak, Some(4));
    }

    #[test]
    fn score_retirement() {
        let (sets, outcome) = parse_score("6-4 6-7(4) 0-0 RET").unwrap();
        assert_eq!(outcome, MatchOutcome::Retirement);
        //incomplete last set is still recorded
        assert_eq!(sets.len(), 3);
    }

    #[test]
    fn score_walkover() {
        let (_sets, outcome) = parse_score("W/O").unwrap();
        assert_eq!(outcome, MatchOutcome::Walkover);
    }

    #[test]
    fn optional_u32_empty() {
        assert_eq!(parse_optional_u32("").unwrap(), None);
    }

    #[test]
    fn test_parse_date_length_gap(){
        assert!(parse_date("2024").is_err());

    }


    

}
