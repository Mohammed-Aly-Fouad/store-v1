-- Add migration script here
-- Create expression index for fast normalized Arabic search
CREATE INDEX IF NOT EXISTS idx_brands_normalized_name_ar 
ON brands (
    regexp_replace(TRANSLATE(name_ar, 'أإآىة', 'ااايه'), '[\u064B-\u0652]', '', 'g') varchar_pattern_ops
);