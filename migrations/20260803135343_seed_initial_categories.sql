-- 1. Insert Main Categories (Parents)
INSERT INTO public.categories (name_en, name_ar, parent_id, notes) VALUES 
('Notebooks', 'الدفاتر', NULL, NULL),
('Writing & Correction', 'أدوات الكتابة والتصحيح', NULL, NULL),
('Office Supplies', 'مستلزمات مكتبية', NULL, NULL),
('Filing & Organization', 'حفظ وتنظيم الملفات', NULL, NULL),
('Technology & Electronics', 'تكنولوجيا وإلكترونيات', NULL, NULL),
('Drawing and measuring tools', 'أدوات الرسم والقياس', NULL, NULL),
('School & College Supplies', 'الأدوات المدرسية والجامعية', NULL, NULL),
('Toys, Games, Gifts & Party Supplies', 'الألعاب والهدايا ومستلزمات الحفلات', NULL, 'Main section for children toys, educational games, modeling dough, crafts, party decorations, and gift wrapping supplies')
ON CONFLICT DO NOTHING;


-- 2. Insert Sub-Categories (Children) linked dynamically to their Parents
INSERT INTO public.categories (name_en, name_ar, parent_id, notes) VALUES 
('Notebooks, Notepads & Journals', 'الكشاكيل والدفاتر والنوت بوك', 
    (SELECT id FROM public.categories WHERE name_en = 'Notebooks' AND parent_id IS NULL), 
    'Includes wirebound notebooks, pocket journals, composition books, notepad sets, and loose foolscap/exam paper'),

('Pencils & Lead Refills', 'أقلام رصاص وسنون ', 
    (SELECT id FROM public.categories WHERE name_en = 'Writing & Correction' AND parent_id IS NULL), 
    'Includes wooden pencils, mechanical pencils, pencil sharpeners, and graphite lead refills'),

('Erasers & Correction', 'أدوات المحو والتصحيح', 
    (SELECT id FROM public.categories WHERE name_en = 'Writing & Correction' AND parent_id IS NULL), 
    'Includes erasers, correction tapes, and correction fluids'),

('Markers & Highlighters', 'أقلام ماركر وتحديد', 
    (SELECT id FROM public.categories WHERE name_en = 'Writing & Correction' AND parent_id IS NULL), 
    'Includes permanent markers, whiteboard markers, and text highlighters'),

('Pens & Refills', 'أقلام جاف وجيل وحبر', 
    (SELECT id FROM public.categories WHERE name_en = 'Writing & Correction' AND parent_id IS NULL), 
    'Includes ballpoint pens, gel pens, rollerball pens, technical drawing pens, and ink refills'),

('Adhesives & Glues', 'مواد وأدوات اللصق', 
    (SELECT id FROM public.categories WHERE name_en = 'Office Supplies' AND parent_id IS NULL), 
    'Includes glue sticks, hot melt glue, liquid glue, and tapes'),

('Stamps & Inks', 'أختام وأحبار', 
    (SELECT id FROM public.categories WHERE name_en = 'Office Supplies' AND parent_id IS NULL), 
    'Includes stamp pads, refill inks, and custom stamps'),

('Pricing, Invoicing & Thermal Rolls', 'أدوات التسعير ودفاتر الفواتير والبكر الحراري', 
    (SELECT id FROM public.categories WHERE name_en = 'Office Supplies' AND parent_id IS NULL), 
    'Includes pricing guns, labels, thermal rolls, invoice books, receipt vouchers, and bill books'),

('Staplers, Clips, Rubber Bands & Desk Accessories', 'دباسات ومشابك وأساتك ومستلزمات مكتبية', 
    (SELECT id FROM public.categories WHERE name_en = 'Office Supplies' AND parent_id IS NULL), 
    'Includes staplers, staples, bulldog clips, paper clips, rubber bands of various sizes, punchers, clipboards, and sticky notes'),

('Legal Contracts & Ready Forms', 'العقود القانونية والنماذج الجاهزة', 
    (SELECT id FROM public.categories WHERE name_en = 'Office Supplies' AND parent_id IS NULL), 
    'Includes ready-made legal contracts, lease agreements, sales contracts, and generic business forms'),

('Files, Folders & Envelopes', 'ملفات ودوسيهات وأظرف ورقية', 
    (SELECT id FROM public.categories WHERE name_en = 'Filing & Organization' AND parent_id IS NULL), 
    'Includes display books, clear sleeves, report covers, flat folders, and all types of mailing and shipping envelopes'),

('Calculators, Storage & Tech Accessories', 'الآلات الحاسبة ووسائط التخزين وإكسسوارات التكنولوجيا', 
    (SELECT id FROM public.categories WHERE name_en = 'Technology & Electronics' AND parent_id IS NULL), 
    'Includes scientific and desktop calculators, USB flash drives, memory cards, and tech accessories'),

('Gift Wrapping Supplies', 'مستلزمات وتغليف الهدايا', 
    (SELECT id FROM public.categories WHERE name_en = 'Toys, Games, Gifts & Party Supplies' AND parent_id IS NULL), 
    'Includes gift ribbons, cellophane sheets, wrapping accessories, and party-related gift packaging'),

('Geometry & Measuring Tools', 'أدوات القياس والهندسة', 
    (SELECT id FROM public.categories WHERE name_en = 'Drawing and measuring tools' AND parent_id IS NULL), 
    'Includes rulers, protractors, compasses, set squares, and complete geometry boxes'),

('Art Supplies, Sketchbooks & Colors', 'أدوات الرسم والاسكتشات والألوان', 
    (SELECT id FROM public.categories WHERE name_en = 'Drawing and measuring tools' AND parent_id IS NULL), 
    'Includes sketchbooks, art papers, colored pencils, crayons, felt-tip markers, watercolors, and drawing accessories'),

('Pencil Cases & School Accessories', 'المقالم والمستلزمات المدرسية', 
    (SELECT id FROM public.categories WHERE name_en = 'School & College Supplies' AND parent_id IS NULL), 
    'Includes fabric pencil cases, hardtop organizers, multi-layer pouches, and basic student accessories'),

('Educational Books & Study Guides', 'الكتب الخارجية والمذكرات التعليمية', 
    (SELECT id FROM public.categories WHERE name_en = 'School & College Supplies' AND parent_id IS NULL), 
    'Includes primary, preparatory, and high school educational textbooks, revision guides, and teacher editions'),

('Modeling Dough & Crafts', 'الصلصال والأنشطة اليدوية للاطفال', 
    (SELECT id FROM public.categories WHERE name_en = 'Toys, Games, Gifts & Party Supplies' AND parent_id IS NULL), 
    'Includes Foam clay, slime, and playdough sets'),

('Party & Birthday Supplies', 'مستلزمات الحفلات وأعياد الميلاد', 
    (SELECT id FROM public.categories WHERE name_en = 'Toys, Games, Gifts & Party Supplies' AND parent_id IS NULL), 
    'Includes balloons, banners, party poppers, candles, and birthday decorations')
ON CONFLICT DO NOTHING;