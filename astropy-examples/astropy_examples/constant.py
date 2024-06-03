import astropy.units as u

# List all SI units
si_units = [attr for attr in dir(u.si) if not attr.startswith('_')]
for u in si_units:
    print(u)

cgs_units = [attr for attr in dir(u.cgs) if not attr.startswith('_')]
for u in si_units:
    print(u)
