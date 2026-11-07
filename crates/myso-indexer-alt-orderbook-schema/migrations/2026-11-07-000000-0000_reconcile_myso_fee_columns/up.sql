-- Databases created from the upstream DeepBook schema use `*_deep` names; the indexer writes `*_myso`.
-- Rename when the old column exists, otherwise add the column. Safe to re-run on any state.

CREATE OR REPLACE FUNCTION pg_temp.reconcile_column(
    tbl TEXT, old_col TEXT, new_col TEXT, col_type TEXT, col_default TEXT
) RETURNS VOID AS $$
BEGIN
    IF to_regclass(tbl) IS NULL THEN
        RETURN;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns
               WHERE table_schema = current_schema() AND table_name = tbl AND column_name = new_col) THEN
        RETURN;
    END IF;
    IF EXISTS (SELECT 1 FROM information_schema.columns
               WHERE table_schema = current_schema() AND table_name = tbl AND column_name = old_col) THEN
        EXECUTE format('ALTER TABLE %I RENAME COLUMN %I TO %I', tbl, old_col, new_col);
    ELSE
        EXECUTE format('ALTER TABLE %I ADD COLUMN %I %s NOT NULL DEFAULT %s', tbl, new_col, col_type, col_default);
    END IF;
END;
$$ LANGUAGE plpgsql;

SELECT pg_temp.reconcile_column('order_fills', 'taker_fee_is_deep', 'taker_fee_is_myso', 'BOOLEAN', 'FALSE');
SELECT pg_temp.reconcile_column('order_fills', 'maker_fee_is_deep', 'maker_fee_is_myso', 'BOOLEAN', 'FALSE');
SELECT pg_temp.reconcile_column('conditional_order_events', 'pay_with_deep', 'pay_with_myso', 'BOOLEAN', 'FALSE');
SELECT pg_temp.reconcile_column('referral_fee_events', 'deep_fee', 'myso_fee', 'BIGINT', '0');

DO $$
BEGIN
    IF to_regclass('deep_burned') IS NOT NULL AND to_regclass('myso_burned') IS NULL THEN
        ALTER TABLE deep_burned RENAME TO myso_burned;
    END IF;
END;
$$;
