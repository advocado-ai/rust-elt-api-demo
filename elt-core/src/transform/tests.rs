use super::parse::{parse_date, parse_optional_u32, parse_round, parse_score, parse_surface};
use crate::types::{MatchOutcome, Round, Surface};
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
    let (sets, outcome) = parse_score("7-6(4) 6-3").unwrap();
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
fn test_parse_date_length_gap() {
    assert!(parse_date("2024").is_err());
}
