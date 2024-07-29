// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Debug, Default)]
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = HashMap::<&str, Team>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team1_name = split_iterator.next().unwrap();
        let team2_name = split_iterator.next().unwrap();
        let team1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        /*
        // Team 1
        scores.entry(team1_name).
            and_modify(|t: &mut Team| {
                t.goals_scored += team1_score;
                t.goals_conceded += team2_score;
            })
            .or_insert(
                Team {
                    goals_scored: team1_score,
                    goals_conceded: team2_score,
                });

        // Team 2
        scores.entry(team2_name).
            and_modify(|t: &mut Team| {
                t.goals_scored += team2_score;
                t.goals_conceded += team1_score;
            })
            .or_insert(
                Team {
                    goals_scored: team2_score,
                    goals_conceded: team1_score,
                });
        */

        // The preceeding worked, but was FAT newbie code.
        // Here's what the official solution looked like:
        let team1 = scores.entry(team1_name).or_default();

        // The crucial insight is that `team1` now contains a &mut to a
        // valid `Team` struct *inside* the mapping that we can freely modify!
        team1.goals_scored += team1_score;
        team1.goals_conceded += team2_score;

        let team2 = scores.entry(team2_name).or_default();
        team2.goals_scored += team2_score;
        team2.goals_conceded += team1_score;
    }
    scores
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
