use crate::Element;
use crate::{Isotope, UncertainFloat, AtomicScatteringFactor, XrayScatteringFactor, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 94,
        name: "Plutonium",
        symbol: "Pu",
        mass: 244.0,
        common_ions: vec![4],
        uncommon_ions: vec![2, 3, 5, 6, 7],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(7.7_f64, 0.1_f64),
            b_p: None,
            b_m: None,
            bound_coherent_scattering_xs: Some(UncertainFloat::new(7.5_f64, 0.2_f64)),
            bound_incoherent_scattering_xs: Some(UncertainFloat::new(0.2_f64, 0.6_f64)),
            total_bound_scattering_xs: Some(UncertainFloat::new(7.7_f64, 0.6_f64)),
            absorption_xs: Some(UncertainFloat::new(1017.3_f64, 2.1_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 228,
                mass: UncertainFloat::new(228.038_73_f64, 0.000_30_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 229,
                mass: UncertainFloat::new(229.040_14_f64, 0.000_80_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 230,
                mass: UncertainFloat::new(230.039_646_f64, 0.000_026_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 231,
                mass: UncertainFloat::new(231.041_26_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 232,
                mass: UncertainFloat::new(232.041_179_f64, 0.000_020_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 233,
                mass: UncertainFloat::new(233.042_99_f64, 0.000_50_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 234,
                mass: UncertainFloat::new(234.043_305_f64, 0.000_008_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 235,
                mass: UncertainFloat::new(235.045_282_f64, 0.000_022_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 236,
                mass: UncertainFloat::new(236.046_048_1_f64, 0.000_002_9_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 237,
                mass: UncertainFloat::new(237.048_403_8_f64, 0.000_002_5_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 238,
                mass: UncertainFloat::new(238.049_553_4_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 239,
                mass: UncertainFloat::new(239.052_156_5_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 240,
                mass: UncertainFloat::new(240.053_807_5_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(3.5_f64, 0.1_f64),
                    b_p: None,
                    b_m: None,
                    bound_coherent_scattering_xs: Some(UncertainFloat::new(1.54_f64, 0.09_f64)),
                    bound_incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    total_bound_scattering_xs: Some(UncertainFloat::new(1.54_f64, 0.09_f64)),
                    absorption_xs: Some(UncertainFloat::new(289.6_f64, 1.4_f64)),
                }),
            },
            Isotope { 
                mass_number: 241,
                mass: UncertainFloat::new(241.056_845_3_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 242,
                mass: UncertainFloat::new(242.058_736_8_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(8.1_f64, 0.1_f64),
                    b_p: None,
                    b_m: None,
                    bound_coherent_scattering_xs: Some(UncertainFloat::new(8.2_f64, 0.2_f64)),
                    bound_incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    total_bound_scattering_xs: Some(UncertainFloat::new(8.2_f64, 0.2_f64)),
                    absorption_xs: Some(UncertainFloat::new(18.5_f64, 0.5_f64)),
                }),
            },
            Isotope { 
                mass_number: 243,
                mass: UncertainFloat::new(243.061_997_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 244,
                mass: UncertainFloat::new(244.064_198_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 245,
                mass: UncertainFloat::new(245.067_739_f64, 0.000_015_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 246,
                mass: UncertainFloat::new(246.070_198_f64, 0.000_016_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 247,
                mass: UncertainFloat::new(247.074_07_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
