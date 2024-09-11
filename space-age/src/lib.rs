#[derive(Debug)]
pub struct Duration {
    pub seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}
pub trait Planet {
    const ORBITAL_PERIOD: f64;
    const EARTH_YEAR_SECONDS: f64 = 31_557_600f64;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (Self::EARTH_YEAR_SECONDS * Self::ORBITAL_PERIOD)
    }
}

macro_rules! planet {
    ( $planet_name:ident, $orbital_period:expr ) => {
        pub struct $planet_name;

        impl Planet for $planet_name {
            const ORBITAL_PERIOD: f64 = $orbital_period;
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
