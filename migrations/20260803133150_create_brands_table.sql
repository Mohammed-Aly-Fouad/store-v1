-- ========================================================
-- 0. Helper Functions
-- ========================================================

-- Create or replace automatic updated_at timestamp maintenance function
CREATE OR REPLACE FUNCTION public.update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ========================================================
-- 1. Table: public.brands
-- ========================================================

-- DROP TABLE IF EXISTS public.brands CASCADE;

CREATE TABLE IF NOT EXISTS public.brands
(
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY,
    name character varying(255) COLLATE pg_catalog."default" NOT NULL,
    name_ar character varying(255) COLLATE pg_catalog."default" NOT NULL,
    notes text COLLATE pg_catalog."default",
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    updated_at timestamp with time zone NOT NULL DEFAULT now(),

    CONSTRAINT brands_pkey PRIMARY KEY (id)
)
TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.brands OWNER to mohammed;

-- ========================================================
-- 2. Indexes for brands
-- ========================================================

CREATE UNIQUE INDEX IF NOT EXISTS idx_brands_unique_name_lower
    ON public.brands USING btree
    (lower(name::text) COLLATE pg_catalog."default" ASC NULLS LAST)
    WITH (fillfactor=100, deduplicate_items=True)
    TABLESPACE pg_default;

CREATE UNIQUE INDEX IF NOT EXISTS idx_brands_unique_name_ar_lower
    ON public.brands USING btree
    (lower(name_ar::text) COLLATE pg_catalog."default" ASC NULLS LAST)
    WITH (fillfactor=100, deduplicate_items=True)
    TABLESPACE pg_default;

-- ========================================================
-- 3. Trigger for brands
-- ========================================================

DROP TRIGGER IF EXISTS update_brands_modtime ON public.brands;
CREATE TRIGGER update_brands_modtime
    BEFORE UPDATE ON public.brands
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();