-- NOT VALID: rows indexed while the constraints were absent may not satisfy them.
ALTER TABLE mydata_data ADD CONSTRAINT mydata_data_data_quality_check
    CHECK (data_quality IN ('high', 'medium', 'low')) NOT VALID;
ALTER TABLE mydata_data ADD CONSTRAINT mydata_data_update_frequency_check
    CHECK (update_frequency IN ('hourly', 'daily', 'weekly', 'monthly', 'yearly')) NOT VALID;
