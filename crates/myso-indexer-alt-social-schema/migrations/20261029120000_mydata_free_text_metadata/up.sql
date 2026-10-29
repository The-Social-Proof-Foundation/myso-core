-- `mydata::validate_optional_metadata` only bounds the length of `update_frequency` and
-- `data_quality`, so any string is valid on chain. Enum checks here made the sequential `mydata`
-- pipeline retry the same batch forever on values like "real-time" or "Daily".
ALTER TABLE mydata_data DROP CONSTRAINT IF EXISTS mydata_data_update_frequency_check;
ALTER TABLE mydata_data DROP CONSTRAINT IF EXISTS mydata_data_data_quality_check;
