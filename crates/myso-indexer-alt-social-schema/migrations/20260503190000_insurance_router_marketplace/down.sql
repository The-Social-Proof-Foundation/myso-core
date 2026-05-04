DROP TABLE IF EXISTS insurance_route_fills;
DROP TABLE IF EXISTS insurance_coverage_routes;

DROP INDEX IF EXISTS idx_insurance_policies_route_id;

ALTER TABLE insurance_policies
    DROP COLUMN IF EXISTS backstop_sweep_amount,
    DROP COLUMN IF EXISTS route_leg_index,
    DROP COLUMN IF EXISTS route_id;

ALTER TABLE insurance_vaults
    DROP COLUMN IF EXISTS paused,
    DROP COLUMN IF EXISTS enabled,
    DROP COLUMN IF EXISTS max_exposure_per_option;
