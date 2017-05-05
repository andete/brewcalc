// (c) 2017 Joost Yervante Damad <joost@damad.be>

pub struct Plato(f64);

pub struct Sg(f64);

// formula from https://www.brewersfriend.com/plato-to-sg-conversion-chart/
impl From<Sg> for Plato {
    fn from(p:Sg) -> Plato {
        let sg = p.0;
        let p = (-1.0 * 616.868) + (1111.14 * sg) - (630.272 * sg * sg) + (135.997 * sg * sg * sg);
        Plato(p)
    }
}

// formula from https://www.brewersfriend.com/plato-to-sg-conversion-chart/
impl From<Plato> for Sg {
    fn from(p:Plato) -> Sg {
        let plato = p.0;
        let sg = 1.0 + (plato / (258.6 - ( (plato/258.2) *227.1) ) );
        Sg(sg)
    }
}
