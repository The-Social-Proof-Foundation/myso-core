# Vesting Curve Verification: Move vs SQL

This document compares the `calculate_claimable` logic in the Move contract (profile.move) with the SQL function `calculate_vesting_claimable` in the vesting migration.

## Summary

Both implementations use the same conceptual model:
- **progress**: elapsed_time / duration (0 to 1, or 0 to 1000 in Move)
- **curved_progress**: progress transformed by curve_factor (linear, exponential, or logarithmic)
- **claimable**: total_amount * curved_progress - claimed_amount, clamped to remaining_balance

## Linear (curve_factor = 1000)

| Implementation | Formula |
|----------------|---------|
| Move | curved_progress = progress (progress in [0, 1000]) |
| SQL | curved_progress = progress_ratio (progress_ratio in [0, 1]) |

**Alignment**: Equivalent. Both use progress directly.

## Exponential (curve_factor > 1000)

More tokens toward the end of the vesting period.

| Implementation | Formula |
|----------------|---------|
| Move | quadratic = progress²/1000, blend: (linear*1000 + quadratic*steepness)/1000 where steepness = curve_factor - 1000 |
| SQL | curved_progress = progress_ratio², blend_factor = LEAST((curve_factor/1000 - 1)*2, 1), result = progress_ratio*(1-blend) + curved_progress*blend |

**Alignment**: Both use quadratic (progress²) for the exponential component. The blend factors differ:
- Move: steepness scales the quadratic component directly (0 to 9000 for curve_factor 10000)
- SQL: blend_factor caps at 1.0 and uses (curve_factor/1000 - 1)*2

For curve_factor=2000: Move steepness=1000, SQL blend=2.0 capped to 1.0. The SQL may produce different results at high curve factors.

## Logarithmic (curve_factor < 1000)

More tokens toward the start of the vesting period.

| Implementation | Formula |
|----------------|---------|
| Move | sqrt_approx = sqrt(progress*1000) via Newton's method, blend: (sqrt_approx*steepness + linear*1000)/1000 where steepness = 1000 - curve_factor |
| SQL | curved_progress = SQRT(progress_ratio), blend_factor = LEAST((1 - curve_factor/1000)*2, 1), result = progress_ratio*(1-blend) + curved_progress*blend |

**Alignment**: Both use square root. Move uses integer sqrt on progress*1000; SQL uses floating-point SQRT on progress_ratio. Results should be close but may differ due to precision.

## Edge Cases

| Case | Move | SQL |
|------|------|-----|
| Before start | return 0 | RETURN 0 |
| After end | return remaining_balance | RETURN total_amount - claimed_amount |
| Zero duration | N/A (assert duration > 0) | elapsed >= duration, returns remainder |
| curve_factor = 0 | Treated as linear (CURVE_PRECISION) | Treated as linear (OR curve_factor = 0) |

## Test Vectors (Manual Verification)

For curve_factor=1000 (linear), progress=0.5:
- Move: progress=500, curved_progress=500, total_claimable = total_amount * 500 / 1000 = 50%
- SQL: progress_ratio=0.5, curved_progress=0.5, total_vested = total_amount * 0.5 = 50%

For curve_factor=100 (logarithmic), progress=0.25:
- Move: steepness=900, sqrt_approx(250)≈15.8, curved_progress blends toward sqrt (more early)
- SQL: SQRT(0.25)=0.5, blend_factor=1.8→1.0, curved_progress blends toward 0.5

For curve_factor=10000 (exponential), progress=0.5:
- Move: steepness=9000, quadratic=250, curved_progress blends heavily toward quadratic
- SQL: progress²=0.25, blend_factor=18→1.0, curved_progress=0.5*0 + 0.25*1 = 0.25

## Recommendations

1. **Linear**: Fully aligned. No changes needed.
2. **Exponential/Logarithmic**: Minor differences possible at extreme curve factors. The SQL blend factor caps at 1.0; Move does not cap steepness. For typical curve_factor values (100–10000), results should be within a few percent.
3. **Production**: If exact parity is required, consider porting the Move formulas to SQL with matching blend logic. Otherwise, the current SQL implementation is acceptable for API/UI display of claimable amounts.
