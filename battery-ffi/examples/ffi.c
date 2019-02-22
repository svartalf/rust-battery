// 1. Build `battery-ffi` crate and copy `battery_ffi.h` from the `OUT_DIR`
// (it is probably somewhere at `target/*/build/battery-ffi-*/out/`)
// next to this file.
//
// 2. Run `gcc ffi.c /path/to/libbattery_ffi.so`
//
// 3. Run `./a.out`

#include <stdio.h>
#include <float.h>
#include <limits.h>

#include "battery_ffi.h"

float from_millis(uint32_t value) {
    return (float) value / 1000;
}

void pretty_print(Battery *battery, uint32_t *idx) {
    printf("Device:\t\t\t%d\n", *idx);

    printf("vendor:\t\t\t");
    char *vendor = battery_get_vendor(battery);
    if (vendor == NULL) {
        printf("N/A\n");
    } else {
        printf("%s\n", vendor);
        battery_str_free(vendor);
    }

    printf("model:\t\t\t");
    char *model = battery_get_model(battery);
    if (model == NULL) {
        printf("N/A\n");
    } else {
        printf("%s\n", model);
        battery_str_free(model);
    }

    printf("S/N:\t\t\t");
    char *sn = battery_get_serial_number(battery);
    if (sn == NULL) {
        printf("N/A\n");
    } else {
        printf("%s\n", sn);
        battery_str_free(sn);
    }

    printf("battery\n");
    printf("  state:\t\t");
    uint8_t state = battery_get_state(battery);
    switch (state) {
        case StateUnknown:
            printf("unknown\n");
            break;
        case StateCharging:
            printf("charging\n");
            break;
        case StateDischarging:
            printf("discharging\n");
            break;
        case StateEmpty:
            printf("empty\n");
            break;
        case StateFull:
            printf("full\n");
            break;
    }
    printf("  energy:\t\t%.2f Wh\n", from_millis(battery_get_energy(battery)));
    printf("  energy-full:\t\t%.2f Wh\n", from_millis(battery_get_energy_full(battery)));
    printf("  energy-full-design:\t%.2f Wh\n", from_millis(battery_get_energy_full_design(battery)));
    printf("  energy-rate:\t\t%.2f W\n", from_millis(battery_get_energy_rate(battery)));
    printf("  voltage:\t\t%.2f V\n", from_millis(battery_get_voltage(battery)));

    printf("  technology:\t\t");
    switch (battery_get_technology(battery)) {
        case TechnologyUnknown:
            printf("unknown\n");
            break;
        case TechnologyLithiumIon:
            printf("lithium-ion\n");
            break;
        case TechnologyLeadAcid:
            printf("lead-acid\n");
            break;
        case TechnologyLithiumPolymer:
            printf("lithium-polymer\n");
            break;
        case TechnologyNickelMetalHydride:
            printf("nickel-metal-hydride\n");
            break;
        case TechnologyNickelCadmium:
            printf("nickel-cadmium\n");
            break;
        case TechnologyNickelZinc:
            printf("nickel-zinc\n");
            break;
        case TechnologyLithiumIronPhosphate:
            printf("lithium-iron-phosphate\n");
            break;
        case TechnologyRechargeableAlkalineManganese:
            printf("rechargeable-alkaline-manganese\n");
            break;
    }

    uint64_t time_to_full = battery_get_time_to_full(battery);
    if ((state == StateCharging) && (time_to_full > 0)) {
        printf("  time-to-full:\t\t%d sec.\n", time_to_full);
    }

    uint64_t time_to_empty = battery_get_time_to_empty(battery);
    if ((state == StateDischarging) && (time_to_empty > 0)) {
        printf("  time-to-empty:\t\t%d sec.\n", time_to_empty);
    }

    printf("  percentage:\t\t%.2f %%\n", battery_get_percentage(battery));
    float temp = battery_get_temperature(battery);
    printf("  temperature:\t\t");
    if (temp < FLT_MAX) {
        printf("%.2f C\n", temp);
    } else {
        printf("N/A\n");
    }

    printf("  capacity:\t\t%.2f %%\n", battery_get_capacity(battery));
    uint32_t cycle_count = battery_get_cycle_count(battery);
    printf("  cycle-count:\t\t");
    if (cycle_count < UINT_MAX) {
        printf("%d\n", cycle_count);
    } else {
        printf("N/A\n");
    }
}

void main() {
    Manager *manager = battery_manager_new();
    Batteries *iterator = battery_manager_iter(manager);
    uint32_t idx = 0;
    while (true) {
        Battery *battery = battery_iterator_next(iterator);
        if (battery == NULL) {
            break;
        }

        pretty_print(battery, &idx);

        battery_free(battery);
        idx++;
    }

    battery_iterator_free(iterator);
    battery_manager_free(manager);
}
