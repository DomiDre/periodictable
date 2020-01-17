use crate::{Element};
use crate::mass::Mass;
use crate::utils::UncertainFloat;
// ScatteringFactor::new(UncertainFloat::new(3.26_f64, 0.03_f64), None, None, false, false, Some(UncertainFloat::new(1.34_f64, 0.02_f64)), Some(UncertainFloat::new(0, 0.0)), Some(UncertainFloat::new(1.34_f64, 0.02_f64)), UncertainFloat::new(0.007_47_f64, 0.000_01_f64)), // 2, He

pub fn load() -> Element {
    Element {
        atomic_number: 1,
        name: "hydrogen",
        symbol: "H",
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        isotopes:
            vec![
                Mass { 
                    mass_number: 1,
                    mass: UncertainFloat::new(1.007_825_032_1_f64, 0.000_000_000_4_f64),
                    abundance: UncertainFloat::new(99.988_5_f64, 0.007_0_f64)
                },
                Mass { 
                    mass_number: 2,
                    mass: UncertainFloat::new(2.014_101_778_0_f64, 0.000_000_000_4_f64),
                    abundance: UncertainFloat::new(0.011_5_f64, 0.007_0_f64)
                },
                Mass { 
                    mass_number: 3,
                    mass: UncertainFloat::new(3.016_049_267_5_f64, 0.000_000_001_1_f64),
                    abundance: UncertainFloat::new(0.0, 0.0)
                },
                Mass { 
                    mass_number: 4,
                    mass: UncertainFloat::new(4.027_83_f64, 0.000_12_f64),
                    abundance: UncertainFloat::new(0.0, 0.0)
                },
                Mass { 
                    mass_number: 5,
                    mass: UncertainFloat::new(5.039_54_f64, 0.001_02_f64),
                    abundance: UncertainFloat::new(0.0, 0.0)
                },
                Mass { 
                    mass_number: 6,
                    mass: UncertainFloat::new(6.044_94_f64, 0.000_28_f64),
                    abundance: UncertainFloat::new(0.0, 0.0)
                },
            ]
    }
}

// impl ElementNeutronScattering for H {
//     fn b_coherent(&self) -> UncertainFloat {
//         UncertainFloat::new(-3.740_9_f64, 0.001_1_f64)
//     }
//     // fn neutron_sf(&self) -> NeutronScatteringFactor {
//     //     NeutronScatteringFactor {
//     //         
//     //         None,
//     //         None,
//     //         false,
//     //         false,
//     //         Some(UncertainFloat::new(1.756_8_f64, 0.001_0_f64)),
//     //         Some(UncertainFloat::new(80.26_f64, 0.06_f64)),
//     //         Some(UncertainFloat::new(82.02_f64, 0.06_f64)),
//     //         UncertainFloat::new(0.332_6_f64, 0.000_7_f64)
//     //     }
//     // }
// }