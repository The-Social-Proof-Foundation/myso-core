-- Unify posts-related time triggers to expect epoch milliseconds
-- All timestamp fields from blockchain events are in milliseconds (epoch timestamp in ms),
-- but PostgreSQL's to_timestamp() expects seconds, so we need to divide by 1000

-- 1. Fix posts time trigger
CREATE OR REPLACE FUNCTION update_post_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 2. Fix comments time trigger
CREATE OR REPLACE FUNCTION update_comment_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 3. Fix reactions time trigger
CREATE OR REPLACE FUNCTION update_reaction_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 4. Fix reposts time trigger
CREATE OR REPLACE FUNCTION update_repost_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 5. Fix tips time trigger
CREATE OR REPLACE FUNCTION update_tip_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 6. Fix posts_reports time trigger
CREATE OR REPLACE FUNCTION update_report_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.reported_at / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 7. Fix posts_transfers time trigger
CREATE OR REPLACE FUNCTION update_transfer_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.transferred_at / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 8. Fix posts_moderation_events time trigger
CREATE OR REPLACE FUNCTION update_moderation_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.moderated_at / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 9. Fix posts_deletion_events time trigger
CREATE OR REPLACE FUNCTION update_deletion_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.deleted_at / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
