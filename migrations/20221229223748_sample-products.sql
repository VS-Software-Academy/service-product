ALTER TABLE "product" ALTER COLUMN "description" TYPE VARCHAR(100)
;
INSERT INTO "product" ("description", "category", "price")
VALUES (
    'Smart TV LED 32" LG 32LQ620BPSB',
    (SELECT "id" FROM "product_category" WHERE "description" = 'Default category'),
    300.99
)
;
INSERT INTO "product" ("description", "category", "price")
VALUES (
    'Smart TV LED 4K UHD 50" Samsung UN50AU7700GXZD',
    (SELECT "id" FROM "product_category" WHERE "description" = 'Default category'),
    650.00
)
;
INSERT INTO "product" ("description", "category", "price")
VALUES (
    'Smart TV Android LED Full HD 43" Philips 43PFG6917/78',
    (SELECT "id" FROM "product_category" WHERE "description" = 'Default category'),
    650.00
)
