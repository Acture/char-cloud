mod support;

use char_cloud::{AlgorithmKind, generate};
use std::time::Instant;
use support::build_request;

#[test]
#[ignore = "performance checks are noisy in shared environments; run manually in CI perf job"]
fn fast_grid_should_not_be_slower_than_random_baseline() {
	let fast_request = build_request(AlgorithmKind::FastGrid);
	let baseline_request = build_request(AlgorithmKind::RandomBaseline);

	let start_fast = Instant::now();
	let _ = generate(fast_request).expect("fast-grid generation should succeed");
	let fast_elapsed = start_fast.elapsed();

	let start_baseline = Instant::now();
	let _ = generate(baseline_request).expect("baseline generation should succeed");
	let baseline_elapsed = start_baseline.elapsed();

	assert!(
		fast_elapsed <= baseline_elapsed,
		"fast-grid regression: fast-grid={:?}, baseline={:?}",
		fast_elapsed,
		baseline_elapsed
	);
}
