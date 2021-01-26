import aircraft_sim

aircraft = aircraft_sim.PyAircraft(4)

for i in aircraft.get_values():
    print(i)

aircraft.test()
print()

for i in aircraft.get_values():
    print(i)
