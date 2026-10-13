// 1. Build `battery` with the `export` feature and copy `battery_ffi.h` from the `OUT_DIR`
// (it is probably somewhere at `target/*/build/battery-*/out/`)
// next to this file.
//
// 2. Run `gcc ffi.c /path/to/libbattery.so`
//
// 3. Run `./a.out`

#include <string.h>
#include <stdio.h>
#include <float.h>
#include <limits.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdlib.h>

#include "battery_ffi.h"

static const char *const state_names[] = {
    "unknown", "charging", "discharging", "empty", "full"
};

static const char *const technology_names[] = {
    "unknown", "lithium-ion", "lead-acid", "lithium-polymer",
    "nickel-metal-hydride", "nickel-cadmium", "nickel-zinc",
    "lithium-iron-phosphate", "rechargeable-alkaline-manganese"
};

static void print_string(BatteryString value) {
    if (value == NULL) {
        printf("N/A\n");
    } else {
        printf("%s\n", (const char *)value);
        battery_str_free(value);
    }
}

void pretty_print(Battery *battery, uint32_t *idx) {
    printf("Device:\t\t\t%u\n", *idx);

    printf("vendor:\t\t\t");
    print_string(battery_get_vendor(battery));

    printf("model:\t\t\t");
    print_string(battery_get_model(battery));

    printf("S/N:\t\t\t");
    print_string(battery_get_serial_number(battery));

    printf("battery\n");
    printf("  state:\t\t");
    State state = battery_get_state(battery);
    printf("%s\n", state < sizeof(state_names) / sizeof(state_names[0])
        ? state_names[state] : "unknown");
    printf("  energy:\t\t%.2f joule\n", battery_get_energy(battery));
    printf("  energy-full:\t\t%.2f joule\n", battery_get_energy_full(battery));
    printf("  energy-full-design:\t%.2f joule\n", battery_get_energy_full_design(battery));
    printf("  energy-rate:\t\t%.2f W\n", battery_get_energy_rate(battery));
    printf("  voltage:\t\t%.2f V\n", battery_get_voltage(battery));

    printf("  technology:\t\t");
    Technology technology = battery_get_technology(battery);
    printf("%s\n", technology < sizeof(technology_names) / sizeof(technology_names[0])
        ? technology_names[technology] : "unknown");

    float time_to_full = battery_get_time_to_full(battery);
    if ((state == STATE_CHARGING) && (time_to_full > 0)) {
        printf("  time-to-full:\t\t%.2f sec.\n", time_to_full);
    }

    float time_to_empty = battery_get_time_to_empty(battery);
    if ((state == STATE_DISCHARGING) && (time_to_empty > 0)) {
        printf("  time-to-empty:\t\t%.2f sec.\n", time_to_empty);
    }

    printf("  state of charge:\t%.2f %%\n", battery_get_state_of_charge(battery));
    float temp = battery_get_temperature(battery);
    printf("  temperature:\t\t");
    if (temp < FLT_MAX) {
        printf("%.2f K\n", temp);
    } else {
        printf("N/A\n");
    }

    printf("  state of health:\t%.2f %%\n", battery_get_state_of_health(battery));
    uint32_t cycle_count = battery_get_cycle_count(battery);
    printf("  cycle-count:\t\t");
    if (cycle_count < UINT_MAX) {
        printf("%u\n", cycle_count);
    } else {
        printf("N/A\n");
    }
}

void print_error() {
    int32_t length = battery_last_error_length();
    if (length <= 0) {
        return;
    }

    uint8_t *message = malloc((size_t)length);
    if (message == NULL) {
        return;
    }
    CSliceMutu8 buffer = {message, (size_t)length};
    int32_t written = battery_last_error_message(buffer);
    if (written > 0) {
        fwrite(message, 1, (size_t)written, stdout);
    }
    free(message);
}

int main(void) {
    Manager *manager = battery_manager_new();
    if (manager == NULL) {
        print_error();
        return 1;
    }

    Batteries *iterator = battery_manager_iter(manager);
    if (iterator == NULL) {
        print_error();
        battery_manager_free(manager);
        return 1;
    }

    uint32_t idx = 0;
    while (true) {
        Battery *battery = battery_iterator_next(iterator);
        if (battery == NULL) {
            if (battery_have_last_error() == 1) {
                print_error();
            }
            break;
        }

        pretty_print(battery, &idx);

        battery_free(battery);
        idx++;
    }

    battery_iterator_free(iterator);
    battery_manager_free(manager);
    return 0;
}
