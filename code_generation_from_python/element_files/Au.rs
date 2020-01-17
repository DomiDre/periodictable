use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 79,
        name: "Gold",
        symbol: "Au",
        mass: 196.96655,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(7.90_f64, 0.07_f64),
            b_p: Some(UncertainFloat::new(6.26_f64, 0.10_f64)),
            b_m: Some(UncertainFloat::new(9.90_f64, 0.14_f64)),
            coherent_scattering_xs: Some(UncertainFloat::new(7.32_f64, 0.12_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.43_f64, 0.05_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(7.75_f64, 0.13_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(98.65_f64, 0.09_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 171,
                mass: UncertainFloat::new(170.991_77_f64, 0.000_27_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 172,
                mass: UncertainFloat::new(171.990_11_f64, 0.000_36_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 173,
                mass: UncertainFloat::new(172.986_40_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 174,
                mass: UncertainFloat::new(173.984_92_f64, 0.000_16_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 175,
                mass: UncertainFloat::new(174.981_55_f64, 0.000_26_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 176,
                mass: UncertainFloat::new(175.980_27_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 177,
                mass: UncertainFloat::new(176.977_22_f64, 0.000_25_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 178,
                mass: UncertainFloat::new(177.975_98_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 179,
                mass: UncertainFloat::new(178.973_41_f64, 0.000_37_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 180,
                mass: UncertainFloat::new(179.972_40_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 181,
                mass: UncertainFloat::new(180.969_95_f64, 0.000_48_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 182,
                mass: UncertainFloat::new(181.969_62_f64, 0.000_39_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 183,
                mass: UncertainFloat::new(182.967_62_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 184,
                mass: UncertainFloat::new(183.967_47_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 185,
                mass: UncertainFloat::new(184.965_81_f64, 0.000_23_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 186,
                mass: UncertainFloat::new(185.966_00_f64, 0.000_15_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 187,
                mass: UncertainFloat::new(186.964_56_f64, 0.000_16_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 188,
                mass: UncertainFloat::new(187.965_09_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 189,
                mass: UncertainFloat::new(188.963_89_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 190,
                mass: UncertainFloat::new(189.964_699_f64, 0.000_017_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 191,
                mass: UncertainFloat::new(190.963_650_f64, 0.000_050_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 192,
                mass: UncertainFloat::new(191.964_810_f64, 0.000_017_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 193,
                mass: UncertainFloat::new(192.964_132_f64, 0.000_010_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 194,
                mass: UncertainFloat::new(193.965_339_f64, 0.000_012_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 195,
                mass: UncertainFloat::new(194.965_018_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 196,
                mass: UncertainFloat::new(195.966_551_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 197,
                mass: UncertainFloat::new(196.966_552_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(100, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 198,
                mass: UncertainFloat::new(197.968_225_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 199,
                mass: UncertainFloat::new(198.968_748_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 200,
                mass: UncertainFloat::new(199.970_720_f64, 0.000_060_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 201,
                mass: UncertainFloat::new(200.971_641_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 202,
                mass: UncertainFloat::new(201.973_79_f64, 0.000_18_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 203,
                mass: UncertainFloat::new(202.975_137_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 204,
                mass: UncertainFloat::new(203.977_71_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 205,
                mass: UncertainFloat::new(204.979_61_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
