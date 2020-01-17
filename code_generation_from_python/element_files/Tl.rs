use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 81,
        name: "Thallium",
        symbol: "Tl",
        mass: 204.3833,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(8.776_f64, 0.005_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(9.678_f64, 0.011_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(0.21_f64, 0.15_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(9.89_f64, 0.15_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(3.43_f64, 0.06_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 177,
                mass: UncertainFloat::new(176.996_88_f64, 0.000_24_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 178,
                mass: UncertainFloat::new(177.995_23_f64, 0.000_23_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 179,
                mass: UncertainFloat::new(178.991_47_f64, 0.000_15_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 180,
                mass: UncertainFloat::new(179.990_19_f64, 0.000_48_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 181,
                mass: UncertainFloat::new(180.986_90_f64, 0.000_41_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 182,
                mass: UncertainFloat::new(181.985_61_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 183,
                mass: UncertainFloat::new(182.982_70_f64, 0.000_42_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 184,
                mass: UncertainFloat::new(183.981_76_f64, 0.000_32_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 185,
                mass: UncertainFloat::new(184.979_10_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 186,
                mass: UncertainFloat::new(185.978_55_f64, 0.000_39_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 187,
                mass: UncertainFloat::new(186.976_17_f64, 0.000_43_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 188,
                mass: UncertainFloat::new(187.975_92_f64, 0.000_24_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 189,
                mass: UncertainFloat::new(188.973_69_f64, 0.000_37_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 190,
                mass: UncertainFloat::new(189.973_79_f64, 0.000_46_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 191,
                mass: UncertainFloat::new(190.971_89_f64, 0.000_23_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 192,
                mass: UncertainFloat::new(191.972_14_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 193,
                mass: UncertainFloat::new(192.970_55_f64, 0.000_27_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 194,
                mass: UncertainFloat::new(193.971_05_f64, 0.000_22_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 195,
                mass: UncertainFloat::new(194.969_65_f64, 0.000_14_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 196,
                mass: UncertainFloat::new(195.970_52_f64, 0.000_15_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 197,
                mass: UncertainFloat::new(196.969_540_f64, 0.000_030_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 198,
                mass: UncertainFloat::new(197.970_470_f64, 0.000_090_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 199,
                mass: UncertainFloat::new(198.969_81_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 200,
                mass: UncertainFloat::new(199.970_945_f64, 0.000_007_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 201,
                mass: UncertainFloat::new(200.970_804_f64, 0.000_016_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 202,
                mass: UncertainFloat::new(201.972_091_f64, 0.000_016_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 203,
                mass: UncertainFloat::new(202.972_329_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(29.524_f64, 0.014_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(8.51_f64, 0.08_f64),
                    b_p: Some(UncertainFloat::new(9.08_f64, 0.10_f64)),
                    b_m: Some(UncertainFloat::new(6.62_f64, 0.10_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(6.14_f64, 0.28_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.14_f64, 0.04_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(6.28_f64, 0.28_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(11.4_f64, 0.2_f64)),
                }),
            },
            Isotope { 
                mass_number: 204,
                mass: UncertainFloat::new(203.973_849_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 205,
                mass: UncertainFloat::new(204.974_412_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(70.476_f64, 0.014_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(8.87_f64, 0.07_f64),
                    b_p: Some(UncertainFloat::new(5.15_f64, 0.10_f64)),
                    b_m: Some(UncertainFloat::new(9.43_f64, 0.10_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(11.39_f64, 0.17_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.007_f64, 0.001_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(11.40_f64, 0.17_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(0.104_f64, 0.017_f64)),
                }),
            },
            Isotope { 
                mass_number: 206,
                mass: UncertainFloat::new(205.976_095_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 207,
                mass: UncertainFloat::new(206.977_408_f64, 0.000_006_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 208,
                mass: UncertainFloat::new(207.982_005_f64, 0.000_003_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 209,
                mass: UncertainFloat::new(208.985_349_f64, 0.000_010_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 210,
                mass: UncertainFloat::new(209.990_066_f64, 0.000_012_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
