#!/usr/bin/env python

"""
This is an example of FFI bindings for `battery-ffi` library.

Call it similar to this:

```
$ LD_LIBRARY_PATH=../../target/debug/ ./ffi.py
```

`battery-ffi` crate should be built before that.
"""

import sys
import ctypes

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': 'dylib', 'win32': 'dll'}.get(sys.platform, 'so')
lib = ctypes.cdll.LoadLibrary('{}battery_ffi.{}'.format(prefix, extension))

STATE = {
    0: 'unknown',
    1: 'charging',
    2: 'discharging',
    3: 'empty',
    4: 'full',
}

TECHNOLOGY = {
    0: 'unknown',
    1: 'lithium-ion',
    2: 'lead-acid',
    3: 'lithium-polymer',
    4: 'nickel-metal-hydride',
    5: 'nickel-cadmium',
    6: 'nickel-zinc',
    7: 'lithium-iron-phosphate',
    8: 'rechargeable-alkaline-manganese',
}

#
# Wrappers around opaque pointers
#


class Manager(ctypes.Structure):
    pass


class Batteries(ctypes.Structure):
    pass


class Battery(ctypes.Structure):
    pass


#
# Bindings for exported functions
#

lib.battery_manager_new.argtypes = None
lib.battery_manager_new.restype = ctypes.POINTER(Manager)
lib.battery_manager_iter.argtypes = (ctypes.POINTER(Manager), )
lib.battery_manager_iter.restype = ctypes.POINTER(Batteries)
lib.battery_manager_free.argtypes = (ctypes.POINTER(Manager), )
lib.battery_manager_free.restype = None

lib.battery_iterator_next.argtypes = (ctypes.POINTER(Batteries), )
lib.battery_iterator_next.restype = ctypes.POINTER(Battery)

lib.battery_free.argtypes = (ctypes.POINTER(Battery), )
lib.battery_free.restype = None
lib.battery_str_free.argtypes = (ctypes.c_char_p, )
lib.battery_str_free.restype = None

lib.battery_get_vendor.argtypes = (ctypes.POINTER(Battery), )
lib.battery_get_vendor.restype = ctypes.c_char_p
lib.battery_get_model.argtypes = (ctypes.POINTER(Battery), )
lib.battery_get_model.restype = ctypes.c_char_p
lib.battery_get_serial_number.argtypes = (ctypes.POINTER(Battery), )
lib.battery_get_serial_number.restype = ctypes.c_char_p
lib.battery_get_state.restype = ctypes.c_uint8
lib.battery_get_energy.argtypes = (ctypes.POINTER(Battery), )
lib.battery_get_energy.restype = ctypes.c_float
lib.battery_get_energy_full.argtypes = (ctypes.POINTER(Battery), )
lib.battery_get_energy_full.restype = ctypes.c_float
lib.battery_get_energy_full_design.argtypes = (ctypes.POINTER(Battery), )
lib.battery_get_energy_full_design.restype = ctypes.c_float
lib.battery_get_energy_rate.argtypes = (ctypes.POINTER(Battery), )
lib.battery_get_energy_rate.restype = ctypes.c_float
lib.battery_get_voltage.argtypes = (ctypes.POINTER(Battery), )
lib.battery_get_voltage.restype = ctypes.c_float
lib.battery_get_technology.argtypes = (ctypes.POINTER(Battery), )
lib.battery_get_technology.restype = ctypes.c_uint8
lib.battery_get_time_to_full.argtypes = (ctypes.POINTER(Battery), )
lib.battery_get_time_to_full.restype = ctypes.c_float
lib.battery_get_time_to_empty.argtypes = (ctypes.POINTER(Battery), )
lib.battery_get_time_to_empty.restype = ctypes.c_float
lib.battery_get_state_of_charge.argtypes = (ctypes.POINTER(Battery), )
lib.battery_get_state_of_charge.restype = ctypes.c_float
lib.battery_get_temperature.argtypes = (ctypes.POINTER(Battery), )
lib.battery_get_temperature.restype = ctypes.c_float
lib.battery_get_state_of_health.argtypes = (ctypes.POINTER(Battery), )
lib.battery_get_state_of_health.restype = ctypes.c_float
lib.battery_get_cycle_count.argtypes = (ctypes.POINTER(Battery), )
lib.battery_get_cycle_count.restype = ctypes.c_uint32

if __name__ == '__main__':
    manager = lib.battery_manager_new()
    iterator = lib.battery_manager_iter(manager)
    while True:
        battery = lib.battery_iterator_next(iterator)
        if not battery:
            break

        print('Vendor', lib.battery_get_vendor(battery))
        print('Model', lib.battery_get_model(battery))
        print('S/N', lib.battery_get_serial_number(battery))
        print('State', STATE.get(lib.battery_get_state(battery)))
        print('Technology', TECHNOLOGY.get(lib.battery_get_technology(battery)))
        print('Energy (joule)', lib.battery_get_energy(battery))
        print('Energy full (joule)', lib.battery_get_energy_full_design(battery))
        print('Energy full design (joule)', lib.battery_get_energy_full_design(battery))
        print('Energy rate (W)', lib.battery_get_energy_rate(battery))
        print('Voltage (V)', lib.battery_get_voltage(battery))
        print('Time to full (sec)', lib.battery_get_time_to_full(battery))
        print('Time to empty (sec)', lib.battery_get_time_to_empty(battery))
        print('State of charge (%)', lib.battery_get_state_of_charge(battery))
        print('Temperature (K)', lib.battery_get_temperature(battery))
        print('State of health (%)', lib.battery_get_state_of_health(battery))
        print('Cycle count', lib.battery_get_cycle_count(battery))

        lib.battery_free(battery)

    lib.battery_iterator_free(battery)
    lib.battery_manager_free(manager)
