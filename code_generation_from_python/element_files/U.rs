use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 92,
        name: "Uranium",
        symbol: "U",
        mass: 238.02891,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(8.417_f64, 0.005_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(8.903_f64, 0.011_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.005_f64, 0.016_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(8.908_f64, 0.011_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(7.57_f64, 0.02_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 218,
                mass: UncertainFloat::new(218.023_49_f64, 0.000_10_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 219,
                mass: UncertainFloat::new(219.024_920_f64, 0.000_090_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 220,
                mass: UncertainFloat::new(220.024_71_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 221,
                mass: UncertainFloat::new(221.026_35_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 222,
                mass: UncertainFloat::new(222.026_07_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 223,
                mass: UncertainFloat::new(223.027_720_f64, 0.000_080_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 224,
                mass: UncertainFloat::new(224.027_590_f64, 0.000_027_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 225,
                mass: UncertainFloat::new(225.029_380_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 226,
                mass: UncertainFloat::new(226.029_340_f64, 0.000_020_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 227,
                mass: UncertainFloat::new(227.031_140_f64, 0.000_018_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 228,
                mass: UncertainFloat::new(228.031_366_f64, 0.000_017_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 229,
                mass: UncertainFloat::new(229.033_496_f64, 0.000_009_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 230,
                mass: UncertainFloat::new(230.033_927_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 231,
                mass: UncertainFloat::new(231.036_289_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 232,
                mass: UncertainFloat::new(232.037_146_3_f64, 0.000_002_9_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 233,
                mass: UncertainFloat::new(233.039_628_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(10.1_f64, 0.2_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(12.8_f64, 0.5_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.1_f64, 0.6_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(12.9_f64, 0.3_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(574.7_f64, 1.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 234,
                mass: UncertainFloat::new(234.040_945_6_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.005_5_f64, 0.000_2_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(12.4_f64, 0.3_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(19.3_f64, 0.9_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(19.3_f64, 0.9_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(100.1_f64, 1.3_f64)),
                }),
            },
            Isotope { 
                mass_number: 235,
                mass: UncertainFloat::new(235.043_923_1_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.720_0_f64, 0.005_1_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(10.50_f64, 0.03_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(13.78_f64, 0.11_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.2_f64, 0.2_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(14.0_f64, 0.2_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(680.9_f64, 1.1_f64)),
                }),
            },
            Isotope { 
                mass_number: 236,
                mass: UncertainFloat::new(236.045_561_9_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 237,
                mass: UncertainFloat::new(237.048_724_0_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 238,
                mass: UncertainFloat::new(238.050_782_6_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(99.274_5_f64, 0.010_6_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(8.407_f64, 0.007_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(8.871_f64, 0.011_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.0, 0.0)),
                    absorption_scattering_xs: Some(UncertainFloat::new(8.871_f64, 0.011_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(2.68_f64, 0.02_f64)),
                }),
            },
            Isotope { 
                mass_number: 239,
                mass: UncertainFloat::new(239.054_287_8_f64, 0.000_002_1_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 240,
                mass: UncertainFloat::new(240.056_586_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 241,
                mass: UncertainFloat::new(241.060_33_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 242,
                mass: UncertainFloat::new(242.062_93_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
