use crate::Element;
use crate::{Isotope, UncertainFloat, AtomicScatteringFactor, XrayScatteringFactor, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 101,
        name: "Mendelevium",
        symbol: "Md",
        mass: 258.0,
        common_ions: vec![3],
        uncommon_ions: vec![2],
        xray_scattering: None,
        neutron_scattering: None,

        isotopes: vec![
            Isotope { 
                mass_number: 245,
                mass: UncertainFloat::new(245.081_02_f64, 0.000_40_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 246,
                mass: UncertainFloat::new(246.081_93_f64, 0.000_41_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 247,
                mass: UncertainFloat::new(247.081_8_f64, 0.003_9_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 248,
                mass: UncertainFloat::new(248.082_91_f64, 0.000_26_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 249,
                mass: UncertainFloat::new(249.083_f64, 0.024_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 250,
                mass: UncertainFloat::new(250.084_49_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 251,
                mass: UncertainFloat::new(251.084_92_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 252,
                mass: UncertainFloat::new(252.086_63_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 253,
                mass: UncertainFloat::new(253.087_28_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 254,
                mass: UncertainFloat::new(254.089_73_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 255,
                mass: UncertainFloat::new(255.091_075_f64, 0.000_008_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 256,
                mass: UncertainFloat::new(256.094_05_f64, 0.000_60_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 257,
                mass: UncertainFloat::new(257.095_535_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 258,
                mass: UncertainFloat::new(258.098_425_f64, 0.000_005_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 259,
                mass: UncertainFloat::new(259.100_5_f64, 0.002_2_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 260,
                mass: UncertainFloat::new(260.103_65_f64, 0.000_34_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
