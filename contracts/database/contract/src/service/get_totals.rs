use crate::{ db::state::TotalState, types::TResult };

pub fn get_totals() -> TResult<(u64, u64)> {
	let totals = TotalState::instance().get_totals();
	Ok(totals)
}
