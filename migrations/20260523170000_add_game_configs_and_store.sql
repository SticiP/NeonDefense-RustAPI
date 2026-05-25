CREATE TABLE item_catalog (
    item_code VARCHAR(30) PRIMARY KEY, -- ex: 'RAM_SCRIPT', 'CPU_OVERCLOCK'
    display_name VARCHAR(100) NOT NULL,
    description TEXT,
    base_drop_weight INT DEFAULT 10, -- Utile pentru algoritmul de drop (șansa de a pica)
    created_at TIMESTAMPTZ DEFAULT NOW()
);

INSERT INTO item_catalog (item_code, display_name) VALUES
    ('CPU_OVERCLOCK', 'CPU Overclock'),
    ('DATA_ARCHIVE', 'Data Archive'),
    ('FIREWALL_NODE', 'Firewall Node'),
    ('PROXY_NETWORK', 'Proxy Network'),
    ('RAM_SCRIPT', 'RAM Script'),
    ('ROOTKIT_INJECTOR', 'Rootkit Injector'),
    ('SYSTEM_OPTIMIZATION', 'System Optimization'),
    ('TARGET_ALGORITHM', 'Target Algorithm');

ALTER TABLE inventory 
    ADD CONSTRAINT fk_inventory_item_type 
    FOREIGN KEY (item_type) REFERENCES item_catalog(item_code) ON DELETE RESTRICT;

CREATE TABLE game_configurations (
    id SERIAL PRIMARY KEY,
    version VARCHAR(50) UNIQUE NOT NULL, -- ex: "4.1.0-stable"
    environment VARCHAR(50) DEFAULT 'PRODUCTION',
    config_payload JSONB NOT NULL, -- Aici stocăm direct JSON-ul tău cu "engine", "security", "enemy_stats"
    is_active BOOLEAN DEFAULT FALSE,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE marketplace_items (
    id SERIAL PRIMARY KEY,
    item_sku VARCHAR(50) UNIQUE NOT NULL, -- ex: 'PACK_1000_CC'
    display_name VARCHAR(100) NOT NULL,   -- ex: '1,000 Crypto-Cores'
    description TEXT,
    rarity VARCHAR(20) DEFAULT 'COMMON',  -- LEGENDARY, RARE, EPIC, COMMON
    price DECIMAL(10, 2) NOT NULL,        -- Prețul de achiziție (ex: 9.99 sau 750)
    currency VARCHAR(20) DEFAULT 'USD',   -- USD, CREDITS, DATA_FRAGMENTS
    reward_type VARCHAR(50) NOT NULL,     -- Ce primește player-ul (ex: 'CRYPTO_CORES', 'LOOT_BOX')
    reward_amount INT NOT NULL,
    stock INT DEFAULT -1,                 -- -1 reprezintă infinit (simbolul ∞ din UI-ul tău)
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);