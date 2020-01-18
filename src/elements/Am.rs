use crate::Element;
use crate::{UncertainFloat, Isotope, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 95,
        name: "Americium",
        symbol: "Am",
        mass: 243.0_f64,
        common_ions: vec![3],
        uncommon_ions: vec![2, 4, 5, 6, 7],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(8.3_f64, 0.2_f64),
            b_p: None,
            b_m: None,
            bound_coherent_scattering_xs: Some(UncertainFloat::new(8.7_f64, 0.4_f64)),
            bound_incoherent_scattering_xs: Some(UncertainFloat::new(0.3_f64, 2.6_f64)),
            total_bound_scattering_xs: Some(UncertainFloat::new(9.0_f64, 2.6_f64)),
            absorption_xs: Some(UncertainFloat::new(75.3_f64, 1.8_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 231,
                mass: UncertainFloat::new(231.045_56_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 232,
                mass: UncertainFloat::new(232.046_59_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 233,
                mass: UncertainFloat::new(233.046_47_f64, 0.000_23_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 234,
                mass: UncertainFloat::new(234.047_79_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 235,
                mass: UncertainFloat::new(235.048_03_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 236,
                mass: UncertainFloat::new(236.049_57_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 237,
                mass: UncertainFloat::new(237.049_97_f64, 0.000_60_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 238,
                mass: UncertainFloat::new(238.051_98_f64, 0.000_50_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 239,
                mass: UncertainFloat::new(239.053_018_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 240,
                mass: UncertainFloat::new(240.055_288_f64, 0.000_015_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 241,
                mass: UncertainFloat::new(241.056_822_9_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 242,
                mass: UncertainFloat::new(242.059_543_f64, 0.000_021_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 243,
                mass: UncertainFloat::new(243.061_372_7_f64, 0.000_002_3_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 244,
                mass: UncertainFloat::new(244.064_279_4_f64, 0.000_002_3_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 245,
                mass: UncertainFloat::new(245.066_445_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 246,
                mass: UncertainFloat::new(246.069_768_f64, 0.000_020_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 247,
                mass: UncertainFloat::new(247.072_09_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 248,
                mass: UncertainFloat::new(248.075_75_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 249,
                mass: UncertainFloat::new(249.078_48_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
