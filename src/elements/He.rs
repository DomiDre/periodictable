use crate::Element;
use crate::mass::Mass;
use crate::utils::UncertainFloat;

pub fn load() -> Element {
    Element {
        atomic_number: 2,
        name: "helium",
        symbol: "He",
        common_ions: vec![],
        oxidation_states: vec![1, 2],
        isotopes:
            vec![
                Mass {
                    mass_number: 3,
                    mass: UncertainFloat::new(3.016_029_309_7_f64, 0.000_000_000_9_f64),
                    abundance: UncertainFloat::new(0.000_137_f64, 0.000_003_f64)
                },
                Mass {
                    mass_number: 4,
                    mass: UncertainFloat::new(4.002_603_249_7_f64, 0.000_000_001_0_f64),
                    abundance: UncertainFloat::new(99.999_863_f64, 0.000_003_f64)
                },
                Mass {
                    mass_number: 5,
                    mass: UncertainFloat::new(5.012_220_f64, 0.000_050_f64),
                    abundance: UncertainFloat::new(0.0, 0.0)
                },
                Mass {
                    mass_number: 6,
                    mass: UncertainFloat::new(6.018_888_1_f64, 0.000_001_1_f64),
                    abundance: UncertainFloat::new(0.0, 0.0)
                },
                Mass {
                    mass_number: 7,
                    mass: UncertainFloat::new(7.028_030_f64, 0.000_030_f64),
                    abundance: UncertainFloat::new(0.0, 0.0)
                },
                Mass {
                    mass_number: 8,
                    mass: UncertainFloat::new(8.033_922_f64, 0.000_008_f64),
                    abundance: UncertainFloat::new(0.0, 0.0)
                },
                Mass {
                    mass_number: 9,
                    mass: UncertainFloat::new(9.043_820_f64, 0.000_070_f64),
                    abundance: UncertainFloat::new(0.0, 0.0)
                },
                Mass {
                    mass_number: 10,
                    mass: UncertainFloat::new(10.052_400_f64, 0.000_080_f64),
                    abundance: UncertainFloat::new(0.0, 0.0)
                },
            ]
    }
}

// impl ElementNeutronScattering for He {
//     fn b_coherent(&self) -> UncertainFloat {
//         UncertainFloat::new(3.26_f64, 0.03_f64)
//     }
// }