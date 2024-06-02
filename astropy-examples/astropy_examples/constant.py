from astropy import units
from astropy import constants as const
print(const.G)
F = (const.G * 3. * const.M_sun * 100 * units.kg) / (2.2 * units.au) ** 2

print(F.to(units.N))

print(const.c.to('km/s'))
