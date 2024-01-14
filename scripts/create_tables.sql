CREATE TABLE melcloud (
    time TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
    device_id INTEGER NOT NULL,
    device_type SMALLINT NOT NULL,
    power BOOLEAN NOT NULL,
    offline BOOLEAN NOT NULL,
    room_temperature REAL NOT NULL,
    set_temperature REAL NOT NULL,
    last_communication TEXT NOT NULL,
    actual_fan_speed SMALLINT NOT NULL,
    fan_speed SMALLINT NOT NULL,
    automatic_fan_speed BOOLEAN,
    vane_vertical_direction SMALLINT NOT NULL,
    vane_vertical_swing BOOLEAN,
    vane_horizontal_direction SMALLINT NOT NULL,
    vane_horizontal_swing BOOLEAN,
    operation_mode SMALLINT NOT NULL,
    in_standby_mode BOOLEAN NOT NULL,
    heating_energy_consumed_rate1 REAL,
    heating_energy_consumed_rate2 REAL,
    cooling_energy_consumed_rate1 REAL,
    cooling_energy_consumed_rate2 REAL,
    auto_energy_consumed_rate1 REAL,
    auto_energy_consumed_rate2 REAL,
    dry_energy_consumed_rate1 REAL,
    dry_energy_consumed_rate2 REAL,
    fan_energy_consumed_rate1 REAL,
    fan_energy_consumed_rate2 REAL,
    other_energy_consumed_rate1 REAL,
    other_energy_consumed_rate2 REAL,
    current_energy_consumed REAL,
    current_energy_mode SMALLINT,
    energy_correction_model REAL,
    energy_correction_active BOOLEAN,
    wifi_signal_strength REAL,
    wifi_adapter_status TEXT,
    has_error BOOLEAN,
    UNIQUE (time, device_id)
);

SELECT CREATE_HYPERTABLE('melcloud', BY_RANGE('time'));
