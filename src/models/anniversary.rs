use super::milestone::Milestone;
use crate::utils::now_local_date;
use time::Date;

#[derive(Debug, Clone)]
pub struct Anniversary {
    pub milestone: Milestone,
    pub next_anniversary: Date,
    pub years_passed: i32,
    pub days_until: i64,
}

pub fn get_upcoming_anniversaries(milestones: &[Milestone], days_ahead: i64) -> Vec<Anniversary> {
    let today = now_local_date();
    let mut anniversaries = Vec::<Anniversary>::new();

    for milestone in milestones.iter().filter(|m| m.is_recurring) {
        if let Some(anniversary) = calculate_next_anniversary(milestone, today, days_ahead) {
            anniversaries.push(anniversary);
        }
    }

    anniversaries.sort_by(|a, b| a.next_anniversary.cmp(&b.next_anniversary));
    anniversaries
}

fn calculate_next_anniversary(
    milestone: &Milestone,
    today: Date,
    days_ahead: i64,
) -> Option<Anniversary> {
    let current_year = today.year();
    let milestone_year = milestone.date.year();

    if current_year < milestone_year {
        return None;
    }

    let this_year_anniversary = milestone.date.replace_year(current_year).ok()?;

    let (next_anniversary, years_passed) = if this_year_anniversary >= today {
        (this_year_anniversary, current_year - milestone_year)
    } else {
        let next_year_anniversary = milestone.date.replace_year(current_year + 1).ok()?;
        (next_year_anniversary, current_year + 1 - milestone_year)
    };

    let days_until = (next_anniversary - today).whole_days();

    if days_until <= days_ahead {
        Some(Anniversary {
            milestone: milestone.clone(),
            next_anniversary,
            years_passed,
            days_until,
        })
    } else {
        None
    }
}
