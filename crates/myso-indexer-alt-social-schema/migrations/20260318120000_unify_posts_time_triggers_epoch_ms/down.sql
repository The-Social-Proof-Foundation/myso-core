-- Revert posts time triggers to expect epoch seconds (original behavior)
CREATE OR REPLACE FUNCTION update_post_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_comment_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_reaction_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_repost_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_tip_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_report_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.reported_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_transfer_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.transferred_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_moderation_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.moderated_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_deletion_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.deleted_at);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
