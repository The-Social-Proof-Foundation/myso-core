-- Add reasoning and evidence_urls fields to spot_resolutions table
ALTER TABLE spot_resolutions 
ADD COLUMN IF NOT EXISTS reasoning TEXT NOT NULL DEFAULT '',
ADD COLUMN IF NOT EXISTS evidence_urls JSONB NOT NULL DEFAULT '[]'::jsonb;

-- Remove defaults after adding columns (for future inserts) - only if defaults exist
DO $$
BEGIN
    -- Check if reasoning column has a default before dropping it
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'spot_resolutions' 
        AND column_name = 'reasoning' 
        AND column_default IS NOT NULL
    ) THEN
        ALTER TABLE spot_resolutions ALTER COLUMN reasoning DROP DEFAULT;
    END IF;
    
    -- Check if evidence_urls column has a default before dropping it
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'spot_resolutions' 
        AND column_name = 'evidence_urls' 
        AND column_default IS NOT NULL
    ) THEN
        ALTER TABLE spot_resolutions ALTER COLUMN evidence_urls DROP DEFAULT;
    END IF;
END $$;

-- Add reasoning and evidence_urls fields to poc_analysis_results table
ALTER TABLE poc_analysis_results 
ADD COLUMN IF NOT EXISTS reasoning TEXT NULL,
ADD COLUMN IF NOT EXISTS evidence_urls JSONB NULL;

-- Add reasoning field to platform_events table
ALTER TABLE platform_events 
ADD COLUMN IF NOT EXISTS reasoning TEXT NULL;

-- Create indexes for querying
CREATE INDEX IF NOT EXISTS idx_spot_resolutions_reasoning ON spot_resolutions USING gin(to_tsvector('english', reasoning)) WHERE reasoning != '';
CREATE INDEX IF NOT EXISTS idx_poc_analysis_reasoning ON poc_analysis_results USING gin(to_tsvector('english', reasoning)) WHERE reasoning IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_platform_events_reasoning ON platform_events USING gin(to_tsvector('english', reasoning)) WHERE reasoning IS NOT NULL;

-- Backfill poc_analysis_results.similarity_detected using score-threshold semantics (GraphQL-facing).
-- Uses latest poc_config thresholds only; skips when no config row exists (runtime indexer applies DEFAULT 95).
UPDATE poc_analysis_results par
SET similarity_detected = par.highest_similarity_score >= (
    CASE par.media_type
        WHEN 1 THEN cfg.image_threshold
        WHEN 2 THEN cfg.video_threshold
        WHEN 3 THEN cfg.audio_threshold
        ELSE 100
    END
)
FROM (
    SELECT image_threshold, video_threshold, audio_threshold
    FROM poc_config
    ORDER BY time DESC
    LIMIT 1
) cfg
WHERE EXISTS (SELECT 1 FROM poc_config LIMIT 1);

UPDATE poc_analysis_results par
SET original_creator = prr.original_post_id
FROM poc_revenue_redirections prr
WHERE par.post_id = prr.accused_post_id
  AND par.transaction_id = prr.transaction_id
  AND par.original_creator IS NULL;

-- poc_revenue_redirections references accused_post_id, not post_id.
CREATE OR REPLACE FUNCTION validate_poc_accused_post_reference()
RETURNS TRIGGER AS $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM posts WHERE post_id = NEW.accused_post_id) THEN
        RAISE EXCEPTION 'Referenced accused_post_id does not exist: %', NEW.accused_post_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS check_poc_redirection_accused_post_reference ON poc_revenue_redirections;
CREATE TRIGGER check_poc_redirection_accused_post_reference
BEFORE INSERT OR UPDATE ON poc_revenue_redirections
FOR EACH ROW
EXECUTE FUNCTION validate_poc_accused_post_reference();

