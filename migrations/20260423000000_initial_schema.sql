CREATE TABLE players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nickname VARCHAR(50) UNIQUE NOT NULL,
    coins BIGINT DEFAULT 100,
    crypto_cores INT DEFAULT 0,
    energy INT DEFAULT 20,
    is_shadowbanned BOOLEAN DEFAULT FALSE,
    is_deleted BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE inventory (
    id UUID PRIMARY KEY,
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    item_type VARCHAR(30) NOT NULL,
    rarity INT DEFAULT 0,
    level INT DEFAULT 1,
    is_equipped BOOLEAN DEFAULT FALSE,
    acquired_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE analytics_events (
    id BIGSERIAL PRIMARY KEY,
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    event_type VARCHAR(50) NOT NULL,
    event_data JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);