-- ========================================================
-- 1. Helper Functions
-- ========================================================

-- Function for automatic updated_at timestamp maintenance
CREATE OR REPLACE FUNCTION public.update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to enforce maximum category nesting depth (e.g., max depth of 3)
CREATE OR REPLACE FUNCTION public.check_category_depth()
RETURNS TRIGGER AS $$
DECLARE
    current_depth INT := 1;
    parent_cursor BIGINT := NEW.parent_id;
BEGIN
    -- Prevent self-referencing parent
    IF NEW.id IS NOT NULL AND NEW.id = NEW.parent_id THEN
        RAISE EXCEPTION 'Category cannot be its own parent.';
    END IF;

    -- Traverse up the parent tree
    WHILE parent_cursor IS NOT NULL LOOP
        current_depth := current_depth + 1;
        
        -- Set your desired max depth limit here (e.g., 3 levels: Root -> Sub -> Sub-sub)
        IF current_depth > 3 THEN
            RAISE EXCEPTION 'Category depth limit exceeded (Maximum 3 levels allowed).';
        END IF;

        SELECT parent_id INTO parent_cursor 
        FROM public.categories 
        WHERE id = parent_cursor;
    END LOOP;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ========================================================
-- 2. Table: public.categories
-- ========================================================

CREATE TABLE IF NOT EXISTS public.categories
(
    id bigint NOT NULL GENERATED ALWAYS AS IDENTITY,
    parent_id bigint,
    name_en character varying(255) COLLATE pg_catalog."default" NOT NULL,
    name_ar character varying(255) COLLATE pg_catalog."default" NOT NULL,
    notes text COLLATE pg_catalog."default",
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    updated_at timestamp with time zone NOT NULL DEFAULT now(),

    CONSTRAINT categories_pkey PRIMARY KEY (id),
    CONSTRAINT fk_categories_parent FOREIGN KEY (parent_id)
        REFERENCES public.categories (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
)
TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.categories OWNER to mohammed;

-- ========================================================
-- 3. Indexes for categories
-- ========================================================

CREATE INDEX IF NOT EXISTS idx_categories_parent_id
    ON public.categories USING btree
    (parent_id ASC NULLS LAST)
    WITH (fillfactor=100, deduplicate_items=True)
    TABLESPACE pg_default;

CREATE UNIQUE INDEX IF NOT EXISTS idx_categories_unique_name_parent_lower
    ON public.categories USING btree
    (lower(name_en::text) COLLATE pg_catalog."default" ASC NULLS LAST, parent_id ASC NULLS LAST)
    NULLS NOT DISTINCT
    WITH (fillfactor=100, deduplicate_items=True)
    TABLESPACE pg_default;

CREATE UNIQUE INDEX IF NOT EXISTS idx_categories_unique_name_ar_parent_lower
    ON public.categories USING btree
    (lower(name_ar::text) COLLATE pg_catalog."default" ASC NULLS LAST, parent_id ASC NULLS LAST)
    NULLS NOT DISTINCT
    WITH (fillfactor=100, deduplicate_items=True)
    TABLESPACE pg_default;

-- ========================================================
-- 4. Triggers for categories
-- ========================================================

DROP TRIGGER IF EXISTS enforce_category_depth ON public.categories;
CREATE TRIGGER enforce_category_depth
    BEFORE INSERT OR UPDATE ON public.categories
    FOR EACH ROW
    EXECUTE FUNCTION public.check_category_depth();

DROP TRIGGER IF EXISTS update_categories_modtime ON public.categories;
CREATE TRIGGER update_categories_modtime
    BEFORE UPDATE ON public.categories
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();